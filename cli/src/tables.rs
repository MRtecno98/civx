
use civx::models::{CurrentUser, Model, ModelVersion, ModelVersionMinimal};
use super_table::{Cell, ContentArrangement, Table, presets::NOTHING};
use itertools::Itertools;

pub(crate) fn new_table() -> Table {
	let mut table = Table::new();
	table.set_content_arrangement(ContentArrangement::Dynamic);
	table.load_preset(NOTHING);
	table
}

pub fn model_table(models: &[Model]) {
	let mut table = new_table();

	table.set_header(vec!["Model", "ID", "Type", "Base Models", "NSFW", "Creator", "Tags"]);

	for model in models {
		let base_models = model.model_versions.iter()
			.map(|v| v.base_model.clone())
			.unique()
			.map(|bm| bm.to_string())
			.collect::<Vec<_>>()
			.join(", ");

		table.add_row(vec![
			model.name.clone(),
			model.id.to_string(),
			model.model_type.to_string(),
			base_models,
			model.nsfw.to_string(),
			model.creator.as_ref().map_or("N/A".to_string(), |c| c.username.clone()),
			model.tags.join(", "),
		]);
	}

	println!("{table}");
}

pub fn version_table(versions: &[ModelVersion]) {
	//println!("Debug: Version table with {} versions", versions.len());

	let mut table = new_table();
	table.set_header(vec!["Version", "ID", "Model ID", "Base Model", "Type", "NSFW", "Downloads", "Thumbs Up", "Thumbs Down", "Comments", "Tipped Amount"]);

	for version in versions {
		table.add_row(vec![
			version.name.clone(),
			version.id.to_string(),
			version.model_id.to_string(),
			version.base_model.to_string(),
			version.base_model_type.to_string(),
			version.nsfw_level.to_string(),
			version.stats.download_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.thumbs_up_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.thumbs_down_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.comment_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.tipped_amount_count.map_or("N/A".to_string(), |c| c.to_string()),
		]);
	}

	println!("{table}");
}

pub fn print_model(model: &Model) {
	//println!("Debug: Model with id {} and name {}", model.id, model.name);

	let mut table = new_table();

	table.add_row(vec![
		"Model".to_string(), model.name.to_string(), 
		"Thumbs Up".to_string(), model.stats.thumbs_up_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"ID".to_string(), model.id.to_string(), 
		"Thumbs Down".to_string(), model.stats.thumbs_down_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"Type".to_string(), model.model_type.to_string(), 
		"Comments".to_string(), model.stats.comment_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"Creator".to_string(), model.creator.as_ref().map_or("N/A".to_string(), |c| c.username.clone()),

		"Downloads".to_string(), model.stats.download_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"NSFW".to_string(), 
		if model.nsfw { 
			format!("true ({})", model.nsfw_level) 
		} else { "false".to_string() },

		"Tipped Amount".to_string(), model.stats.tipped_amount_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"SFW Only".to_string(), model.sfw_only.to_string(),
		"Minor".to_string(), model.minor.to_string(),
	]);

	table.add_row(vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);

	table.add_row(vec![
		Cell::new("Base Models"),
		Cell::new(
			model.model_versions.iter().map(|v| v.base_model.clone())
				.unique().map(|bm| bm.to_string()).collect::<Vec<_>>().join(", ")
		).set_colspan(3),
	]);

	table.add_row(vec![
		Cell::new("Tags"),
		Cell::new(model.tags.join(", ")).set_colspan(3),
	]);

	let mut version_table = new_table();
	version_table.set_header(vec!["Version", "ID", "Base Model", "Type", "Downloads", "Thumbs Up", "Thumbs Down", "Comments", "Tipped Amount"]);

	for version in model.model_versions.iter().sorted_by_key(|m| m.index) {
		version_table.add_row(vec![
			version.name.clone(),
			version.id.to_string(),
			version.base_model.to_string(),
			version.base_model_type.to_string(),
			version.stats.download_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.thumbs_up_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.thumbs_down_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.comment_count.map_or("N/A".to_string(), |c| c.to_string()),
			version.stats.tipped_amount_count.map_or("N/A".to_string(), |c| c.to_string()),
		]);
	}

	println!("{table}\n\n{version_table}");
}

pub fn print_version(version: &ModelVersion) {
	//println!("Debug: Version with id {} and name {}", version.id, version.name);

	let mut table = new_table();

	table.add_row(vec![
		"Model".to_string(), version.model.name.to_string(),
		"Thumbs Up".to_string(), version.stats.thumbs_up_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"Version".to_string(), version.name.to_string(), 
		"Thumbs Down".to_string(), version.stats.thumbs_down_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"ID".to_string(), version.id.to_string(), 
		"Comments".to_string(), version.stats.comment_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"Model ID".to_string(), version.model_id.to_string(), 
		"Downloads".to_string(), version.stats.download_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"Base Model".to_string(), version.base_model.to_string(),
		"Tipped Amount".to_string(), version.stats.tipped_amount_count.map_or("N/A".to_string(), |c| c.to_string())
	]);

	table.add_row(vec![
		"NSFW".to_string(), version.model.nsfw.to_string(),
		"Person of Interest".to_string(), version.model.poi.to_string(),
	]);

	table.add_row(vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);

	table.add_row(vec![
		Cell::new("AIR"),
		Cell::new(version.air.to_string()).set_colspan(3),
	]);

	let mut files_table = new_table();
	files_table.set_header(vec!["File Name", "Primary", "Type", "Pickle Scan", "Virus Scan", "Size (KB)", "URL"]);

	for file in &version.files {
		files_table.add_row(vec![
			file.name.clone(),
			file.primary.to_string(),
			file.file_type.to_string(),
			file.pickle_scan_result.to_string(),
			file.virus_scan_result.to_string(),
			file.size_kb.to_string(),
			file.download_url.to_string(),
		]);
	}

	println!("{table}\n\n{files_table}");
}

pub fn print_version_minimal(version: &ModelVersionMinimal) {
	let mut table = new_table();

	table.add_row(vec![
		"Model".to_string(), version.model_name.clone(),
		"Can Generate".to_string(), version.can_generate.to_string(),
	]);

	table.add_row(vec![
		"Version".to_string(), version.version_name.clone(),
		"Featured".to_string(), version.is_featured.to_string(),
	]);

	table.add_row(vec![
		"Base Model".to_string(), version.base_model.to_string(),
		"Requires Auth".to_string(), version.require_auth.to_string(),
	]);

	table.add_row(vec![
		"Availability".to_string(), version.availability.to_string(),
		"Check Permission".to_string(), version.check_permission.to_string(),
	]);

	table.add_row(vec![
		"Published".to_string(), version.published_at.format("%Y-%m-%d %H:%M UTC").to_string(),
		"Additional Charge".to_string(), version.additional_resource_charge.to_string(),
	]);

	table.add_row(vec![
		"File Type".to_string(), format!("{} ({})", version.file_type.to_string(), version.format),
		"Early Access Ends".to_string(), version.early_access_ends_at
			.map_or("N/A".to_string(), |d| d.format("%Y-%m-%d %H:%M UTC").to_string()),
	]);

	table.add_row(vec![
		"Size (KB)".to_string(), format!("{:.2}", version.size),
		"Free Trial Limit".to_string(), version.free_trial_limit.map_or("N/A".to_string(), |l| l.to_string()),
	]);

	table.add_row(vec![
		"SFW Only".to_string(), version.sfw_only.to_string(),
		"Minor".to_string(), version.minor.to_string(),
	]);

	table.add_row(vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()]);

	table.add_row(vec![
		Cell::new("AIR"),
		Cell::new(version.air.to_string()).set_colspan(3),
	]);

	table.add_row(vec![
		Cell::new("File Name"),
		Cell::new(&version.file_name).set_colspan(3),
	]);

	let mut hashes_table = new_table();

	for (name, hash) in [
		("AutoV1", &version.hashes.auto_v1),
		("AutoV2", &version.hashes.auto_v2),
		("AutoV3", &version.hashes.auto_v3),
		("SHA256", &version.hashes.sha256),
		("CRC32", &version.hashes.crc32),
		("BLAKE3", &version.hashes.blake3),
	] {
		if let Some(hash) = hash {
			hashes_table.add_row(vec![name, hash.as_str()]);
		}
	}

	// Remove the left padding so the nested table lines up with the other values
	if let Some(column) = hashes_table.column_mut(0) {
		column.set_padding((0, 1));
	}

	table.add_row(vec![
		Cell::new("Hashes"),
		Cell::new(if hashes_table.is_empty() { "N/A".to_string() } else { hashes_table.to_string() }).set_colspan(3),
	]);

	table.add_row(vec![
		Cell::new("URL(s)"),
		Cell::new(
			version.download_urls.iter().map(|u| u.to_string()).collect::<Vec<_>>().join("\n")
		).set_colspan(3),
	]);

	println!("{table}");
}

pub fn print_whoami(current: &CurrentUser) {
	let mut table = new_table();

	table.add_row(vec![
		"ID".to_string(), current.id.to_string(),
		"Username".to_string(), current.username.clone(),
	]);

	table.add_row(vec![
		"Membership Tier".to_string(), current.tier.to_string(),
		"Status".to_string(), current.status.to_string(),
	]);

	table.add_row(vec![
		"Is Member".to_string(), current.is_member.to_string(),
		"Subscriptions".to_string(), if current.subscriptions.is_empty() {
			"N/A".to_string()
		} else {
			current.subscriptions.join(", ")
		},
	]);

	println!("{table}");
}