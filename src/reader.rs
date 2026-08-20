use std::{io, pin::Pin, task::{Context, Poll}};

use sha2::{Digest, Sha256};
use tokio::io::{AsyncRead, ReadBuf};

pub trait Hasher: HasherDyn {
	type Output: AsRef<[u8]>;

	fn update(&mut self, bytes: &[u8]);
	fn finalize(self) -> Hash<Self>;
}

pub trait HasherDyn {
	fn update(&mut self, bytes: &[u8]);
	fn finalize(self: Box<Self>) -> DynHash;
}

pub trait HasherExt<const N: usize>: Hasher<Output = [u8; N]> + Sized {
	fn new() -> Self;

	fn from_hex(hex: &str) -> Result<Hash<Self>, hex::FromHexError> {
		Ok(Hash(hex::decode(hex)?.try_into()
			.map_err(|_| hex::FromHexError::InvalidStringLength)?))
	}
}

impl<H: Hasher> HasherDyn for H {
	fn update(&mut self, bytes: &[u8]) {
		<H as Hasher>::update(self, bytes);
	}

	fn finalize(self: Box<Self>) -> DynHash {
		DynHash(<H as Hasher>::finalize(*self).0.as_ref().to_owned())
	}
}

impl HasherDyn for Box<dyn HasherDyn + Unpin + '_> {
	fn update(&mut self, bytes: &[u8]) {
		(**self).update(bytes);
	}

	fn finalize(self: Box<Self>) -> DynHash {
		(*self).finalize()
	}
}

impl HasherExt<4> for crc32fast::Hasher {
	fn new() -> Self {
		crc32fast::Hasher::new()
	}
}

impl HasherExt<32> for Sha256 {
	fn new() -> Self {
		<Sha256 as Digest>::new()
	}
}

impl HasherExt<32> for blake3::Hasher {
	fn new() -> Self {
		blake3::Hasher::new()
	}
}

impl Hasher for crc32fast::Hasher {
	type Output = [u8; 4];
	
	fn update(&mut self, bytes: &[u8]) {
		crc32fast::Hasher::update(self, bytes);
	}

	fn finalize(self) -> Hash<Self> {
		Hash(crc32fast::Hasher::finalize(self).to_le_bytes())
	}
}

impl Hasher for Sha256 {
	type Output = [u8; 32];

	fn update(&mut self, bytes: &[u8]) {
		<Self as Digest>::update(self, bytes);
	}

	fn finalize(self) -> Hash<Self> {
		Hash(<Self as Digest>::finalize(self).into())
	}
}

impl Hasher for blake3::Hasher {
	type Output = [u8; 32];

	fn update(&mut self, bytes: &[u8]) {
		blake3::Hasher::update(self, bytes);
	}

	fn finalize(self) -> Hash<Self> {
		Hash(blake3::Hasher::finalize(&self).into())
	}
}

pub struct DynHash(pub Vec<u8>);
pub struct Hash<H: Hasher + ?Sized>(pub H::Output);

impl<H: Hasher + ?Sized> Hash<H> {
	pub fn to_dyn(&self) -> DynHash {
		DynHash(self.0.as_ref().to_owned())
	}

	pub fn to_hex(&self) -> String {
		hex::encode_upper(self.0.as_ref())
	}
}

impl DynHash {
	#[must_use]
	pub fn to_hex(&self) -> String {
		hex::encode_upper(&self.0)
	}
}

pub struct HashReader<R: AsyncRead + Unpin, H: HasherDyn + Unpin> {
	reader: R,
	hasher: H,
}

impl<R: AsyncRead + Unpin, H: HasherDyn + Unpin> HashReader<R, H> {
	pub fn new<const N: usize>(reader: R) -> Self where H: HasherExt<N> {
		Self {
			reader,
			hasher: H::new(),
		}
	}

	pub fn new_hasher(reader: R, hasher: H) -> Self {
		Self { reader, hasher }
	}

	pub fn into_inner(self) -> (R, H) {
		(self.reader, self.hasher)
	}
}

impl<R: AsyncRead + Unpin, H: HasherDyn + Unpin> AsyncRead for HashReader<R, H> {
	fn poll_read(
		mut self: Pin<&mut Self>, 
		cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) 
	-> Poll<io::Result<()>> {
		let filled_before = buf.filled().len();

		match Pin::new(&mut self.reader).poll_read(cx, buf) {
			Poll::Ready(Ok(())) => {
				let filled_after = buf.filled().len();
				let newly_read = &buf.filled()[filled_before..filled_after];

				if !newly_read.is_empty() {
					self.hasher.update(newly_read);
				}

				Poll::Ready(Ok(()))
			}

			other => other,
		}
	}
}

impl<R: AsyncRead + Unpin, H: HasherDyn + Unpin> HashReader<R, H> {
	pub fn finalize(self) -> (H::Output, R) where H: Hasher {
		let (reader, hasher) = self.into_inner();
		(<H as Hasher>::finalize(hasher).0, reader)
	}

	pub fn finalize_dyn(self) -> (DynHash, R) {
		let (reader, hasher) = self.into_inner();
		(Box::new(hasher).finalize(), reader)
	}

	pub fn finalize_hex(self) -> (String, R) {
		let (hash, reader) = self.finalize_dyn();
		(hash.to_hex(), reader)
	}
}

pub struct VerifyingReader<R: AsyncRead + Unpin, H: HasherDyn + Unpin> {
	reader: HashReader<R, H>,
	pub expected: DynHash,
	pub content_length: Option<u64>,
}

impl<R: AsyncRead + Unpin, H: HasherDyn + Unpin> AsyncRead for VerifyingReader<R, H> {
	fn poll_read(
		mut self: Pin<&mut Self>, 
		cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) 
	-> Poll<io::Result<()>> {
		Pin::new(&mut self.reader).poll_read(cx, buf)
	}
}

impl<R: AsyncRead + Unpin, H: HasherDyn + Unpin> VerifyingReader<R, H> {
	pub fn new<const N: usize>(reader: R, expected: &Hash<H>, content_length: Option<u64>) -> Self
	where H: HasherExt<N> {
		Self {
			reader: HashReader::<R, H>::new::<N>(reader),
			expected: expected.to_dyn(),
			content_length,
		}
	}

	pub fn new_hasher(reader: R, hasher: H, expected: DynHash, content_length: Option<u64>) -> Self {
		Self {
			reader: HashReader::<R, H>::new_hasher(reader, hasher),
			expected,
			content_length,
		}
	}

	pub fn verify(self) -> crate::Result<R> {
		let (hash, reader) = self.reader.finalize_dyn();

		if hash.0 == self.expected.0 {
			Ok(reader)
		} else {
			Err(crate::error::Error::HashMismatch { 
				expected: self.expected.to_hex(), 
				actual: hash.to_hex()
			})
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::hashes::{Blake3, Crc32, Sha256};

	async fn hash_reader_verify<H, const N: usize>(message: &'static [u8], expected_hash: &str) 
		-> Result<(), Box<dyn std::error::Error>> where H: HasherExt<N> + Unpin {
		let mut reader = HashReader::<_, H>::new(message);

		tokio::io::copy(&mut reader, &mut tokio::io::sink()).await?;

		assert_eq!(reader.finalize_hex().0.to_uppercase(), expected_hash.to_owned());

		Ok(())
	}

	#[tokio::test]
	async fn hash_reader_verify_sha256() -> Result<(), Box<dyn std::error::Error>> {
		const MESSAGE: &[u8] = b"Hello, world!";
		const EXPECTED_HASH: &str = "315F5BDB76D078C43B8AC0064E4A0164612B1FCE77C869345BFC94C75894EDD3";

		hash_reader_verify::<Sha256, _>(MESSAGE, EXPECTED_HASH).await
	}

	#[tokio::test]
	async fn hash_reader_verify_blake3() -> Result<(), Box<dyn std::error::Error>> {
		const MESSAGE: &[u8] = b"Hello, world!";
		const EXPECTED_HASH: &str = "EDE5C0B10F2EC4979C69B52F61E42FF5B413519CE09BE0F14D098DCFE5F6F98D";

		hash_reader_verify::<Blake3, _>(MESSAGE, EXPECTED_HASH).await
	}

	#[tokio::test]
	async fn hash_reader_verify_crc32() -> Result<(), Box<dyn std::error::Error>> {
		const MESSAGE: &[u8] = b"Hello, world!";
		const EXPECTED_HASH: &str = "E6C6E6EB";

		hash_reader_verify::<Crc32, _>(MESSAGE, EXPECTED_HASH).await
	}
}