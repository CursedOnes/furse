use super::{
    common_structs::ModLoaderType,
    *,
};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SearchQuery<'a> {
    /// Filter by game id
    pub game_id: ID,
    /// Filter by section id (discoverable via Categories)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_id: Option<ID>,
    /// Filter by category id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<ID>,
    /// Filter by game version string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_version: Option<&'a str>,
    /// Filter by free text search in the mod name and author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_filter: Option<&'a str>,
    /// Filter by `ModsSearchSortField` enumeration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ModsSearchSortField>,
    /// 'asc' if sort is in ascending order, 'desc' if sort is in descending order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// Filter only mods associated to a given modloader (Forge, Fabric ...). Must be coupled with gameVersion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_loader_type: Option<ModLoaderType>,
    /// Filter only mods that contain files tagged with versions of the given `game_version_type_id`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_version_type_id: Option<ID>,
    /// Filter by slug (coupled with `class_id` will result in a unique result)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<&'a str>,
    /// A zero based index of the first item to include in the response
    pub index: Number,
    /// The number of items to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Number>,
}

impl Default for SearchQuery<'_> {
    fn default() -> Self {
        Self {
            game_id: 432,
            class_id: None,
            category_id: None,
            game_version: None,
            search_filter: None,
            sort_field: None,
            sort_order: None,
            mod_loader_type: None,
            game_version_type_id: None,
            slug: None,
            index: 0,
            page_size: None,
        }
    }
}

impl AsRef<Self> for SearchQuery<'_> {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModsSearchSortField {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
}

#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    #[default]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
