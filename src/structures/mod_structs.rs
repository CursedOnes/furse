use super::{
    common_structs::Category,
    file_structs::{File, FileIndex},
    Datetime, Number, ID,
};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::clone::Clone;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Mod {
    /// The mod ID
    pub id: ID,
    /// The game ID this mod is for
    pub game_id: ID,
    /// The name of the mod
    pub name: String,
    /// The mod slug that would appear in the URL
    pub slug: String,
    /// Relevant links for the mod such as Issue tracker and Wiki
    pub links: ModLinks,
    /// Mod summary
    pub summary: String,
    /// Current mod status
    pub status: ModStatus,
    /// Number of downloads for the mod
    pub download_count: Number,
    /// Whether the mod is included in the featured mods list
    pub is_featured: bool,
    /// The main category of the mod as it was chosen by the mod author
    pub primary_category_id: ID,
    /// List of categories that this mod is related to
    pub categories: Vec<Category>,
    /// The class id this mod belongs to
    pub class_id: Option<Number>,
    /// List of the mod's authors
    pub authors: Vec<ModAuthor>,
    /// The mod's logo asset
    pub logo: Option<ModAsset>,
    /// List of screenshot assets
    pub screenshots: Vec<ModAsset>,
    /// The ID of the main file of the mod
    pub main_file_id: ID,
    /// List of latest files of the mod
    pub latest_files: Vec<File>,
    /// List of file related details for the latest files of the mod
    pub latest_files_indexes: Vec<FileIndex>,
    /// The creation date of the mod
    pub date_created: Datetime,
    /// The last time the mod was modified
    pub date_modified: Datetime,
    /// The release date of the mod
    pub date_released: Datetime,
    /// Is mod allowed to be distributed
    pub allow_mod_distribution: Option<bool>,
    /// The mod popularity rank for the game
    pub game_popularity_rank: Number,
    /// Is the mod available for search. This can be false when a mod is experimental, in a deleted state or has only alpha files
    pub is_available: bool,
    /// The mod's thumbs up count
    pub thumbs_up_count: Option<Number>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ModLinks {
    pub website_url: Url,
    pub wiki_url: Option<Url>,
    pub issues_url: Option<Url>,
    pub source_url: Option<Url>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ModAuthor {
    pub id: ID,
    pub name: String,
    pub url: Url,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ModAsset {
    pub id: ID,
    pub mod_id: ID,
    pub title: String,
    pub description: String,
    pub thumbnail_url: Url,
    pub url: Url,
}
