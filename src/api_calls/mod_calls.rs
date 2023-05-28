use crate::{
    request::API_URL_BASE,
    structures::{mod_structs::*, search_query::SearchQuery, ID},
    Furse, Result,
};
use serde::{Deserialize, Serialize};

impl Furse {
    /// Get mod with ID `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod
    /// let terralith_mod = curseforge.get_mod(513688)?;
    /// // Check that it is made by Starmute
    /// assert!(terralith_mod.authors[0].name == "Starmute");
    /// # Ok(()) }
    /// ```
    pub fn get_mod(&self, mod_id: ID) -> Result<Mod> {
        Ok(self
            .get(API_URL_BASE.join("mods/")?.join(&mod_id.to_string())?)
            ?
            .data)
    }

    /// Get multiple mods with IDs `mod_ids`
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get Xaero's minimap and worldmap mods
    /// let mods = curseforge.get_mods(vec![263420, 317780])?;
    /// // Check that both are made by `xaero96`
    /// assert!(mods[0].authors[0].name == "xaero96" && mods[1].authors[0].name == "xaero96");
    /// # Ok(()) }
    /// ```
    pub fn get_mods(&self, mod_ids: Vec<ID>) -> Result<Vec<Mod>> {
        #[derive(Deserialize, Serialize, Debug, Clone)]
        #[serde(rename_all = "camelCase")]
        #[cfg_attr(feature = "serde_deny_unknown_fields", serde(deny_unknown_fields))]
        struct GetModsByIdsListRequestBody {
            mod_ids: Vec<ID>,
        }
        Ok(self
            .post(
                API_URL_BASE.join("mods")?,
                &GetModsByIdsListRequestBody { mod_ids },
            )
            ?
            .data)
    }

    /// Get the description of mod with ID `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's description
    /// let terralith_mod_description = curseforge.get_mod_description(513688)?;
    /// // The description would obviously contains the mod's name
    /// assert!(terralith_mod_description.contains("Terralith"));
    /// # Ok(()) }
    /// ```
    pub fn get_mod_description(&self, mod_id: ID) -> Result<String> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&format!("{}/", mod_id))?
                    .join("description")?,
            )
            ?
            .data)
    }

    /// Search mods with search query
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Search query for mods with foo in name, descending
    /// let search_query = furse::structures::search_query::SearchQuery {
    ///     class_id: Some(6),
    ///     search_filter: Some("foo"),
    ///     sort_order: Some(furse::structures::search_query::SortOrder::Desc),
    ///     ..Default::default()
    /// };
    /// // Search with search query
    /// let found_mods = curseforge.search_mods(search_query)?;
    /// # Ok(()) }
    /// ```
    pub fn search_mods<'a>(&self, search_query: impl AsRef<SearchQuery<'a>>) -> Result<Vec<Mod>> {
        let mut url = API_URL_BASE.join("mods/search")?;
        url.set_query(Some(&serde_urlencoded::to_string(search_query.as_ref())?));
        Ok(self.get(url)?.data)
    }
}
