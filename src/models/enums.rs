use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Enums {
	pub model_type: Vec<String>,
	pub model_file_type: Vec<String>,
	pub active_base_model: Vec<String>,
	pub base_model: Vec<String>,
	pub base_model_type: Vec<String>,
}
