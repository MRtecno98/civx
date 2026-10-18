use futures::{StreamExt, TryStream, stream};

use crate::{CivitAI, error::Error, models::{File, Model, ModelVersion, ModelVersionEntry}};

#[derive(Debug, Clone, Copy)]
pub struct VersionEntryExt<'a> {
	pub version: &'a ModelVersionEntry,
	pub model: &'a Model,
}

#[derive(Debug, Clone)]
pub struct ModelVersionExt<'a> {
	pub version: ModelVersion,
	pub model: &'a Model,
}

#[derive(Debug, Clone, Copy)]
pub struct FileExt<'a, V = ModelVersion> {
	pub file: &'a File,
	pub version: &'a V,
	pub model: &'a Model,
}

type EntryFileExt<'a> = FileExt<'a, ModelVersionEntry>;

impl<'a, V> FileExt<'a, V> {
	fn new<T>(file: &'a File, version: &'a T) -> Self 
		where T: AsRef<V> + AsRef<Model> {
		Self {
			file,
			version: version.as_ref(),
			model: version.as_ref(),
		}
	}
}

impl Model {
	pub fn version_entries(&self) -> impl Iterator<Item = VersionEntryExt<'_>> {
		self.model_versions.iter().map(move |version| VersionEntryExt {
			version,
			model: self,
		})
	}

	pub fn versions<'a>(&'a self, client: &'a CivitAI) -> impl TryStream<Ok = ModelVersionExt<'a>, Error = Error> + 'a {
		stream::iter(self.model_versions.iter())
			.map(async |version| {
				Ok(ModelVersionExt {
					version: version.request_full(client).await?,
					model: self,
				})
			}).buffer_unordered(10) // Avoid tanking the server with hundreds of requests at once
	}

	pub fn files(&self) -> impl Iterator<Item = EntryFileExt<'_>> + '_ {
		self.version_entries().flat_map(|e| e.version.files.iter().map(
			move |file| FileExt {
				file,
				version: e.version,
				model: e.model,
			}))
	}
}

impl ModelVersionExt<'_> {
	pub fn files(&self) -> impl Iterator<Item = FileExt<'_, ModelVersion>> + '_ {
		self.version.files.iter().map(move |file| FileExt::new(file, self))
	}
}

impl AsRef<ModelVersionEntry> for VersionEntryExt<'_> {
	fn as_ref(&self) -> &ModelVersionEntry {
		self.version
	}
}

impl AsRef<Model> for VersionEntryExt<'_> {
	fn as_ref(&self) -> &Model {
		self.model
	}
}

impl AsRef<ModelVersion> for ModelVersionExt<'_> {
	fn as_ref(&self) -> &ModelVersion {
		&self.version
	}
}

impl AsRef<Model> for ModelVersionExt<'_> {
	fn as_ref(&self) -> &Model {
		self.model
	}
}

impl<V> AsRef<File> for FileExt<'_, V> {
	fn as_ref(&self) -> &File {
		self.file
	}
}

impl AsRef<ModelVersion> for FileExt<'_, ModelVersion> {
	fn as_ref(&self) -> &ModelVersion {
		self.version
	}
}

impl AsRef<ModelVersionEntry> for FileExt<'_, ModelVersionEntry> {
	fn as_ref(&self) -> &ModelVersionEntry {
		self.version
	}
}

impl<V> AsRef<Model> for FileExt<'_, V> {
	fn as_ref(&self) -> &Model {
		self.model
	}
}
