use std::path::PathBuf;

use civx::{AIR, queries::{ListCollections, ListCreators, ListImages, ListModels, ListTags, LookupUsers}};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, name = "civx", bin_name = "civx", styles = crate::clap_styles())]
pub struct Args {
	#[command(subcommand)]
	pub command: Command,

	#[arg(long, global = true, default_value_t = false)]
	pub raw: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
	Articles {
		#[command(subcommand)]
		action: ArticleAction
	},

	Collections {
		#[command(subcommand)]
		action: CollectionAction
	},

	Creators(ListCreators<'static>),

	#[group(required = true, multiple = false)]
	Download {
		air: Option<AIR>,

		#[clap(long)]
		hash: Option<String>,

		#[clap(long)]
		list: Option<PathBuf>,
	},

	Images {
		#[command(subcommand)]
		action: ImageAction
	},

	Models {
		#[command(subcommand)]
		action: ModelAction
	},

	Tags(ListTags<'static>),
	
	Users(LookupUsers<'static>),

	Whoami,

	Login,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum ModelAction {
	Search {
		#[clap(flatten)]
		query: ListModels<'static>,

		#[clap(long)]
		hash: Option<String>,
	},

	Get { id: i64, },

	#[group(required = true, multiple = false)]
	BulkSearch {
		hashes: Option<Vec<String>>,

		#[clap(long)]
		input: Option<PathBuf>,
	},

	Version {
		id: i64,

		#[clap(long)]
		mini: bool,
	},
}

#[derive(Subcommand, Debug)]
pub enum ImageAction {
	Search(ListImages<'static>),
}

#[derive(Subcommand, Debug)]
pub enum ArticleAction {
	Search,
	Save { id: i64, },
	Get { id: i64, },
}

#[derive(Subcommand, Debug)]
pub enum CollectionAction {
	Search(ListCollections<'static>),
	List { id: u32 },
}
