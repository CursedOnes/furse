use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serde_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Category {
    pub id: ID,
    /// The game id related to the category
    pub game_id: ID,
    /// Category name
    pub name: String,
    /// The category slug as it appear in the URL
    pub slug: Option<String>, // The "Technology" category has no slug
    pub url: Url,
    /// URL for the category icon
    pub icon_url: Url,
    /// Last modified date of the category
    #[serde(deserialize_with = "deserialize_date_modified")]
    pub date_modified: UtcTime,
    /// Whether this is a top level category for other categories
    pub is_class: Option<bool>,
    /// The class which this category is under
    pub class_id: Option<ID>,
    pub parent_category_id: Option<ID>,
    pub display_index: Option<Number>,
}

//TODO proper decoding instead of this hack, needs to be able to handle e.g. "0001-01-01T00:00:00" from "Technology" category
fn deserialize_date_modified<'de, D>(deserializer: D) -> Result<UtcTime, D::Error> where D: serde::Deserializer<'de> { 
    let mut s: String = serde::Deserialize::deserialize(deserializer)?;

    if let Ok(v) = chrono::DateTime::parse_from_rfc3339(&s) {
        return Ok(v.with_timezone(&chrono::Utc));
    }

    s += "Z";

    match chrono::DateTime::parse_from_rfc3339(&s) {
        Ok(v) => Ok(v.with_timezone(&chrono::Utc)),
        Err(e) => Err(serde::de::Error::custom(e)),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serde_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SortableGameVersion {
    /// Original version name (e.g. 1.5b)
    pub game_version_name: String,
    /// Padded version used for sorting (e.g. 0000000001.0000000005)
    pub game_version_padded: String,
    /// Clean version (e.g. 1.5)
    pub game_version: String,
    pub game_version_release_date: UtcTime,
    pub game_version_type_id: Option<ID>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serde_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Pagination {
    /// A zero based index of the first item included in the response
    pub index: Number,
    /// The requested number of items to be included in the response
    pub page_size: Number,
    /// The actual number of items that were included in the response
    pub result_count: Number,
    /// The total number of items available in the request
    pub total_count: Number,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
}
