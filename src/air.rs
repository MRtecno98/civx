use std::{borrow::Cow, fmt::{self, Display}, str::FromStr};

use serde::{Deserialize, Serialize, de::{self, Visitor}};

use crate::enums::{BaseModel, ModelType};

pub trait AirIdent {
	fn air(&self) -> Cow<'_, AIR>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AIR {
	pub ecosystem: Ecosystem,
	pub resource_type: ResourceType,
	pub source: Source,
	pub id: String,
	pub version: Option<String>,
	pub file_id: Option<String>,
	pub format: Option<Format>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
	InvalidPrefix,
	MissingRequiredParts,
}

impl Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ParseError::InvalidPrefix => write!(f, "invalid prefix"),
			ParseError::MissingRequiredParts => write!(f, "missing required parts"),
		}
	}
}

impl std::error::Error for ParseError {}

impl FromStr for AIR {
	type Err = ParseError;
	fn from_str(value: &str) -> Result<Self, Self::Err> {
		let (rest, format) = 
			if let Some((r, f)) = value.rsplit_once('.') {
				(r, Some(Format::from(f)))
			} else {
				(value, None)
			};
		
		let (rest, file_id) = 
			if let Some((r, f)) = rest.rsplit_once('+') {
				(r, Some(f.to_string()))
			} else {
				(rest, None)
			};

		let (rest, version) = 
			if let Some((r, v)) = rest.rsplit_once('@') {
				(r, Some(v.to_string()))
			} else {
				(rest, None)
			};

		let mut required_parts = rest.rsplitn(5, ':').collect::<Vec<_>>();
		required_parts.reverse();

		let required_parts = if required_parts.len() == 5 {
			let (prefix, rest) = required_parts.split_first().unwrap();

			if *prefix != "urn:air" {
				return Err(ParseError::InvalidPrefix);
			}

			rest
		} else {
			&required_parts
		};

		if required_parts.len() != 4 {
			return Err(ParseError::MissingRequiredParts);
		}

		let ecosystem = Ecosystem::from(required_parts[0]);
		let resource_type = ResourceType::from(required_parts[1]);
		let source = Source::from(required_parts[2]);

		let id = required_parts[3].to_string();

		Ok(AIR {
			ecosystem,
			resource_type,
			source,
			id,
			version,
			file_id,
			format,
		})
	}
}

impl Display for AIR {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "urn:air:{}:{}:{}:{}", 
			String::from(self.ecosystem.clone()), 
			String::from(self.resource_type.clone()), 
			String::from(self.source.clone()), 
			self.id)?;

		if let Some(version) = &self.version {
			write!(f, "@{version}")?;
		}

		if let Some(file_id) = &self.file_id {
			write!(f, "+{file_id}")?;
		}

		if let Some(format) = &self.format {
			write!(f, ".{}", String::from(format.clone()))?;
		}

		Ok(())
	}
}

impl<'de> Deserialize<'de> for AIR {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de> {
		struct AirVisitor;

        impl Visitor<'_> for AirVisitor {
            type Value = AIR;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an AI Resource Identifier (AIR)")
            }

            fn visit_str<E>(self, value: &str) -> Result<AIR, E>
            where
                E: de::Error,
            {
				value.parse().map_err(
					|e| E::custom(format!("failed to parse AIR identifier: {e}")))
            }
        }

        deserializer.deserialize_str(AirVisitor)
	}
}

impl Serialize for AIR {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer {
		serializer.serialize_str(&self.to_string())
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Ecosystem {
	SD15,
	SDXL,
	SD3,
	FLUX1,
	FLUX2,
	FLUX3,
	Illustrious,
	Other,
	Unknown(String),
}

impl From<BaseModel> for Ecosystem {
	fn from(base_model: BaseModel) -> Self {
		match base_model {
			BaseModel::SD15 
			| BaseModel::SD15Hyper
			| BaseModel::SD15LCM
			 => Ecosystem::SD15,
			
			BaseModel::SDXL09
			 | BaseModel::SDXL10
			 | BaseModel::SDXLTurbo
			 | BaseModel::SDXLLightning
			 | BaseModel::SDXLHyper
			 | BaseModel::SDXL10LCM
			 | BaseModel::SDXLDistilled
			 => Ecosystem::SDXL,

			BaseModel::SD3
			| BaseModel::SD35
			| BaseModel::SD35Large
			| BaseModel::SD35LargeTurbo
			| BaseModel::SD35Medium
			 => Ecosystem::SD3,
			
			BaseModel::Flux1S
			| BaseModel::Flux1D
			| BaseModel::Flux1Krea
			| BaseModel::Flux1Kontext
			 => Ecosystem::FLUX1,

			BaseModel::Flux2D 
			| BaseModel::Flux2Klein9B
			| BaseModel::Flux2Klein9Bbase
			| BaseModel::Flux2Klein4B
			| BaseModel::Flux2Klein4Bbase
			 => Ecosystem::FLUX2,

			BaseModel::Flux3Video => Ecosystem::FLUX3,
			BaseModel::Illustrious => Ecosystem::Illustrious,
			_ => Ecosystem::Other,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ResourceType {
	Checkpoint,
	LORA,
	Embedding,
	VAE,
	ControlNet,
	Upscaler,
	Other,
	Unknown(String),
}

impl From<ModelType> for ResourceType {
	fn from(model_type: ModelType) -> Self {
		match model_type {
			ModelType::Checkpoint => ResourceType::Checkpoint,
			ModelType::LORA => ResourceType::LORA,
			ModelType::TextualInversion => ResourceType::Embedding,
			ModelType::VAE => ResourceType::VAE,
			ModelType::Controlnet => ResourceType::ControlNet,
			ModelType::Upscaler => ResourceType::Upscaler,
			ModelType::Unknown(typ) => ResourceType::Unknown(typ),
			_ => ResourceType::Other,
		}
	}
}

impl From<ResourceType> for ModelType {
	fn from(resource_type: ResourceType) -> Self {
		match resource_type {
			ResourceType::Checkpoint => ModelType::Checkpoint,
			ResourceType::LORA => ModelType::LORA,
			ResourceType::Embedding => ModelType::TextualInversion,
			ResourceType::VAE => ModelType::VAE,
			ResourceType::ControlNet => ModelType::Controlnet,
			ResourceType::Upscaler => ModelType::Upscaler,
			ResourceType::Unknown(typ) => ModelType::Unknown(typ),
			ResourceType::Other => ModelType::Other,
		}
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Source {
	CivitAI,
	CivitAIR2,
	HuggingFace,
	Orchestrator,
	Other,
	Unknown(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Format {
	Safetensor,
	Ckpt,
	Diffuser,
	Pth,
	Unknown(String),
}

impl From<&str> for Ecosystem {
	fn from(s: &str) -> Self {
		match s {
			"sd15" => Ecosystem::SD15,
			"sdxl" => Ecosystem::SDXL,
			"sd3" => Ecosystem::SD3,
			"flux1" => Ecosystem::FLUX1,
			"flux2" => Ecosystem::FLUX2,
			"flux3" => Ecosystem::FLUX3,
			"illustrious" => Ecosystem::Illustrious,
			"other" => Ecosystem::Other,
			_ => Ecosystem::Unknown(s.to_string()),
		}
	}
}

impl From<Ecosystem> for String {
	fn from(e: Ecosystem) -> Self {
		match e {
			Ecosystem::SD15 => "sd15".into(),
			Ecosystem::SDXL => "sdxl".into(),
			Ecosystem::SD3 => "sd3".into(),
			Ecosystem::FLUX1 => "flux1".into(),
			Ecosystem::FLUX2 => "flux2".into(),
			Ecosystem::FLUX3 => "flux3".into(),
			Ecosystem::Illustrious => "illustrious".into(),
			Ecosystem::Other => "other".into(),
			Ecosystem::Unknown(s) => s,
		}
	}
}

impl From<&str> for ResourceType {
	fn from(s: &str) -> Self {
		match s {
			"checkpoint" => ResourceType::Checkpoint,
			"lora" => ResourceType::LORA,
			"embedding" => ResourceType::Embedding,
			"vae" => ResourceType::VAE,
			"controlnet" => ResourceType::ControlNet,
			"upscaler" => ResourceType::Upscaler,
			"other" => ResourceType::Other,
			_ => ResourceType::Unknown(s.to_string()),
		}
	}
}

impl From<ResourceType> for String {
	fn from(r: ResourceType) -> Self {
		match r {
			ResourceType::Checkpoint => "checkpoint".into(),
			ResourceType::LORA => "lora".into(),
			ResourceType::Embedding => "embedding".into(),
			ResourceType::VAE => "vae".into(),
			ResourceType::ControlNet => "controlnet".into(),
			ResourceType::Upscaler => "upscaler".into(),
			ResourceType::Other => "other".into(),
			ResourceType::Unknown(s) => s,
		}
	}
}

impl From<&str> for Source {
	fn from(s: &str) -> Self {
		match s {
			"civitai" => Source::CivitAI,
			"civitai-r2" => Source::CivitAIR2,
			"huggingface" => Source::HuggingFace,
			"orchestrator" => Source::Orchestrator,
			"other" => Source::Other,
			_ => Source::Unknown(s.to_string()),
		}
	}
}

impl From<Source> for String {
	fn from(s: Source) -> Self {
		match s {
			Source::CivitAI => "civitai".into(),
			Source::CivitAIR2 => "civitai-r2".into(),
			Source::HuggingFace => "huggingface".into(),
			Source::Orchestrator => "orchestrator".into(),
			Source::Other => "other".into(),
			Source::Unknown(s) => s,
		}
	}
}

impl From<&str> for Format {
	fn from(s: &str) -> Self {
		match s {
			"safetensor" => Format::Safetensor,
			"ckpt" => Format::Ckpt,
			"diffuser" => Format::Diffuser,
			"pth" => Format::Pth,
			_ => Format::Unknown(s.to_string()),
		}
	}
}

impl From<Format> for String {
	fn from(f: Format) -> Self {
		match f {
			Format::Safetensor => "safetensor".into(),
			Format::Ckpt => "ckpt".into(),
			Format::Diffuser => "diffuser".into(),
			Format::Pth => "pth".into(),
			Format::Unknown(s) => s,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
    use serde_test::{assert_tokens, assert_de_tokens, Token};

	#[test]
	pub fn air_serde_full() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "827184".to_string(),
			version: Some("2514310".to_string()),
			file_id: Some("2402203".to_string()),
			format: Some(Format::Pth),
		}, &[Token::String("urn:air:sdxl:checkpoint:civitai:827184@2514310+2402203.pth")]);
	}

	#[test]
	pub fn air_serde_min() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "43235".to_string(),
			version: None,
			file_id: None,
			format: None,
		}, &[Token::String("urn:air:sdxl:checkpoint:civitai:43235")]);
	}

	#[test]
	pub fn air_serde_min_noprefix() {
		assert_de_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "43235".to_string(),
			version: None,
			file_id: None,
			format: None,
		}, &[Token::String("sdxl:checkpoint:civitai:43235")]);
	}
	
	#[test]
	pub fn air_serde_min_version() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "43235".to_string(),
			version: Some("123456".to_string()),
			file_id: None,
			format: None,
		}, &[Token::String("urn:air:sdxl:checkpoint:civitai:43235@123456")]);
	}

	#[test]
	pub fn air_serde_min_file_id() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "43235".to_string(),
			version: None,
			file_id: Some("123456".to_string()),
			format: None,
		}, &[Token::String("urn:air:sdxl:checkpoint:civitai:43235+123456")]);
	}

	#[test]
	pub fn air_serde_min_format() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "43235".to_string(),
			version: None,
			file_id: None,
			format: Some(Format::Pth),
		}, &[Token::String("urn:air:sdxl:checkpoint:civitai:43235.pth")]);
	}

	#[test]
	pub fn air_serde_real_0() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "827184".to_string(),
			version: Some("2514310".to_string()),
			file_id: None,
			format: None,
		}, &[Token::String("urn:air:sdxl:checkpoint:civitai:827184@2514310")]);
	}

	#[test]
	pub fn air_serde_real_1() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::SDXL,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "827184".to_string(),
			version: Some("2514310".to_string()),
			file_id: Some("2402203".to_string()),
			format: None,
		}, &[Token::String("urn:air:sdxl:checkpoint:civitai:827184@2514310+2402203")]);
	}

	#[test]
	pub fn air_serde_real_2() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::Illustrious,
			resource_type: ResourceType::Checkpoint,
			source: Source::CivitAI,
			id: "795765".to_string(),
			version: Some("900661".to_string()),
			file_id: None,
			format: None,
		}, &[Token::String("urn:air:illustrious:checkpoint:civitai:795765@900661")]);
	}

	#[test]
	pub fn air_serde_real_3() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::Other,
			resource_type: ResourceType::Upscaler,
			source: Source::CivitAI,
			id: "147759".to_string(),
			version: Some("164821".to_string()),
			file_id: None,
			format: None,
		}, &[Token::String("urn:air:other:upscaler:civitai:147759@164821")]);
	}

	#[test]
	pub fn air_serde_real_4() {
		assert_tokens(&AIR {
			ecosystem: Ecosystem::Other,
			resource_type: ResourceType::Other,
			source: Source::CivitAIR2,
			id: "civitai-worker-assets".to_string(),
			version: Some("sam_vit_b_01ec64".to_string()),
			file_id: None,
			format: Some(Format::Pth),
		}, &[Token::String("urn:air:other:other:civitai-r2:civitai-worker-assets@sam_vit_b_01ec64.pth")]);
	}
}