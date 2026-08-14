use std::io;

use reqwest::Method;
use tokio::{fs, io::AsyncRead};
use futures_util::TryStreamExt;
use tokio_util::io::StreamReader;

use crate::{CivitAI, Result, error::Error, hashes::{Blake3, Crc32, Sha256}, models::{File, Hashes}, reader::{Hash, Hasher, HasherDyn, HasherExt, VerifyingReader}};

pub mod hashes {
	pub type Crc32 = crc32fast::Hasher;
	pub type Sha256 = sha2::Sha256;
	pub type Blake3 = blake3::Hasher;
}

trait HashGetter<H: Hasher> {
	fn get_hash(&self) -> Option<&String>;
}

impl HashGetter<Crc32> for Hashes {
	fn get_hash(&self) -> Option<&String> {
		self.crc32.as_ref()
	}
}

impl HashGetter<Sha256> for Hashes {
	fn get_hash(&self) -> Option<&String> {
		self.sha256.as_ref()
	}
}

impl HashGetter<Blake3> for Hashes {
	fn get_hash(&self) -> Option<&String> {
		self.blake3.as_ref()
	}
}

impl Hashes {
	#[allow(private_bounds)]
	pub fn get<H, const N: usize>(&self) -> Option<Hash<H>>
	where Self: HashGetter<H>, H: HasherExt<N> {
		HashGetter::<H>::get_hash(self)
			.map(|s| H::from_hex(s))
			.transpose().ok().flatten()
	}

	pub fn check_reader(&self, reader: impl AsyncRead + Unpin, content_length: Option<u64>) 
		-> crate::Result<VerifyingReader<impl AsyncRead + Unpin, Box<dyn HasherDyn + Unpin>>> {
		let (hash, hasher) = if let Some(hash) = self.get::<Blake3, _>() {
			(hash.to_dyn(), Box::new(Blake3::new()) as Box<dyn HasherDyn + Unpin>)
		} else if let Some(hash) = self.get::<Sha256, _>() {
			(hash.to_dyn(), Box::new(Sha256::new()) as Box<dyn HasherDyn + Unpin>)
		} else if let Some(hash) = self.get::<Crc32, _>() {
			(hash.to_dyn(), Box::new(Crc32::new()) as Box<dyn HasherDyn + Unpin>)
		} else {
			return Err(Error::MissingHash);
		};

		Ok(VerifyingReader::new_hasher(reader, hasher, hash, content_length))
	}
	
	#[allow(private_bounds)]
	pub fn check_reader_hash<H, const N: usize>(&self, reader: impl AsyncRead + Unpin, content_length: Option<u64>) 
		-> crate::Result<VerifyingReader<impl AsyncRead + Unpin, H>>
	where Self: HashGetter<H>, H: HasherExt<N> + Unpin {
		let hash = self.get::<H, N>().ok_or(Error::MissingHash)?;

		Ok(VerifyingReader::new(reader, hash, content_length))
	}
}

impl File {
	pub async fn download(&self, client: &CivitAI) -> Result<VerifyingReader<impl AsyncRead + Unpin, Box<dyn HasherDyn + Unpin>>> {
		let response = client.make_request(Method::GET, 
			self.download_url.as_ref())?.send().await?;
			
		let content_length = response.content_length();

		let reader = response.bytes_stream()
			.map_err(io::Error::other);

		self.hashes.check_reader(StreamReader::new(reader), content_length)
	}

	pub async fn download_to_file(&self, path: impl AsRef<std::path::Path>, client: &CivitAI) -> Result<()> {
		let mut file = fs::File::create(path).await?;
		let mut reader = self.download(client).await?;

		tokio::io::copy(&mut reader, &mut file).await?;

		reader.verify()?;

		Ok(())
	}
}
