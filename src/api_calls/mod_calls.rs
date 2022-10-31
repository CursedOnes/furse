use crate::{
    request::API_URL_BASE,
    structures::{mod_structs::*, search_query::SearchQuery, ID},
    Furse, Result,
};

impl Furse {
    /// Get mod with ID `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod
    /// let terralith_mod = curseforge.get_mod(513688).await?;
    /// // Check that it is made by Starmute
    /// assert!(terralith_mod.authors[0].name == "Starmute");
    /// # Ok(()) }
    /// ```
    pub async fn get_mod(&self, mod_id: ID) -> Result<Mod> {
        Ok(self
            .get(API_URL_BASE.join("mods/")?.join(&mod_id.to_string())?)
            .await?
            .data)
    }

    /// Get the description of mod with ID `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's description
    /// let terralith_mod_description = curseforge.get_mod_description(513688).await?;
    /// // The description would obviously contains the mod's name
    /// assert!(terralith_mod_description.contains("Terralith"));
    /// # Ok(()) }
    /// ```
    pub async fn get_mod_description(&self, mod_id: ID) -> Result<String> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&format!("{}/", mod_id))?
                    .join("description")?,
            )
            .await?
            .data)
    }

    /// Search mods with search query
    ///
    /// Example:
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Search query for mods with foo in name, descending
    /// let search_query = furse::structures::search_query::SearchQuery {
    ///     class_id: Some(6),
    ///     search_filter: Some("foo"),
    ///     sort_order: Some(furse::structures::search_query::SortOrder::Desc),
    ///     ..Default::default()
    /// };
    /// // Search with search query
    /// let found_mods = curseforge.search_mods(search_query).await?;
    /// # Ok(()) }
    /// ```
    pub async fn search_mods(&self, search_query: impl AsRef<SearchQuery<'_>>) -> Result<Vec<Mod>> {
        let mut url = API_URL_BASE.join("mods/search")?;
        url.set_query(Some(&serde_urlencoded::to_string(search_query.as_ref())?));
        Ok(self.get(url).await?.data)
    }
}
