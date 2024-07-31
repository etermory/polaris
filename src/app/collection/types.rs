use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
	app::vfs::{self},
	db,
};

// TODO no longer needed!!
#[derive(Clone, Debug, FromRow, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiString(pub Vec<String>);

impl MultiString {
	pub const SEPARATOR: &'static str = "\u{000C}";
}

impl From<Option<String>> for MultiString {
	fn from(value: Option<String>) -> Self {
		match value {
			None => Self(Vec::new()),
			Some(s) => Self(s.split(Self::SEPARATOR).map(|s| s.to_string()).collect()),
		}
	}
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Directory not found: {0}")]
	DirectoryNotFound(PathBuf),
	#[error("Artist not found")]
	ArtistNotFound,
	#[error("Album not found")]
	AlbumNotFound,
	#[error(transparent)]
	Database(#[from] sqlx::Error),
	#[error(transparent)]
	DatabaseConnection(#[from] db::Error),
	#[error(transparent)]
	Vfs(#[from] vfs::Error),
	#[error("Could not deserialize collection")]
	IndexDeserializationError,
	#[error("Could not serialize collection")]
	IndexSerializationError,
	#[error(transparent)]
	ThreadPoolBuilder(#[from] rayon::ThreadPoolBuildError),
	#[error(transparent)]
	ThreadJoining(#[from] tokio::task::JoinError),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum File {
	Directory(PathBuf),
	Song(PathBuf),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Song {
	pub path: PathBuf,
	pub virtual_path: PathBuf,
	pub virtual_parent: PathBuf,
	pub track_number: Option<i64>,
	pub disc_number: Option<i64>,
	pub title: Option<String>,
	pub artists: MultiString,
	pub album_artists: MultiString,
	pub year: Option<i64>,
	pub album: Option<String>,
	pub artwork: Option<PathBuf>,
	pub duration: Option<i64>,
	pub lyricists: MultiString,
	pub composers: MultiString,
	pub genres: MultiString,
	pub labels: MultiString,
	pub date_added: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
	pub virtual_path: PathBuf,
	pub virtual_parent: Option<PathBuf>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Artist {
	pub name: Option<String>,
	pub albums: Vec<Album>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Album {
	pub name: Option<String>,
	pub artwork: Option<PathBuf>,
	pub artists: Vec<String>,
	pub year: Option<i64>,
	pub date_added: i64,
	pub songs: Vec<Song>,
}
