use std::path::PathBuf;

use civx::{AIR, queries::{ListCollections, ListCreators, ListImages, ListModels, ListTags, LookupUsers}};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, name = "civx", bin_name = "civx", styles = crate::clap_styles())]
pub struct Args {
	#[command(subcommand)]
	command: Command
}

#[derive(Subcommand)]
pub enum Command {
	Articles {
		#[command(subcommand)]
		command: ArticleAction
	},

	Collections {
		#[command(subcommand)]
		command: CollectionAction
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
		command: ImageAction
	},

	Models {
		#[command(subcommand)]
		command: ModelAction
	},

	Tags(ListTags<'static>),
	
	Users(LookupUsers<'static>),

	Whoami,
}
 
#[derive(Subcommand)]
pub enum ModelAction {
	Search {
		#[clap(flatten)]
		query: Option<ListModels<'static>>,

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

#[derive(Subcommand)]
pub enum ImageAction {
	Search(ListImages<'static>),
}

#[derive(Subcommand)]
pub enum ArticleAction {
	Search,
	Save { id: i64, },
	Get { id: i64, },
}

#[derive(Subcommand)]
pub enum CollectionAction {
	Search(ListCollections<'static>),
	List { id: u32 },
}
