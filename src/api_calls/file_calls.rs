use crate::{
    request::API_URL_BASE,
    structures::{file_structs::*, ID},
    Furse, Result,
};

impl Furse {
    /// Get the files of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's files
    /// let terralith_files = curseforge.get_mod_files(513688)?;
    /// // Check that the latest file is downloadable
    /// assert!(terralith_files[0].is_available);
    /// # Ok(()) }
    /// ```
    pub fn get_mod_files(&self, mod_id: ID) -> Result<Vec<File>> {
        let mut url = API_URL_BASE
            .join("mods/")?
            .join(&format!("{}/", mod_id))?
            .join("files")?;
        url.set_query(Some("pageSize=10000"));
        let resp: crate::api_calls::Response<Vec<File>> = self.get(url)?;
        //eprintln!("Resp entries={} pagination=\n{:?}",resp.data.len(),&resp.pagination);
        Ok(resp.data)
    }

    /// Get the file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # 
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file
    /// let terralith_file = curseforge.get_mod_file(513688, 3606078)?;
    /// // Check that it contains the version in the file name
    /// assert!(terralith_file.file_name.contains("v2.0.12"));
    /// # Ok(()) }
    /// ```
    pub fn get_mod_file(&self, mod_id: ID, file_id: ID) -> Result<File> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&format!("{}/", mod_id))?
                    .join("files/")?
                    .join(&file_id.to_string())?,
            )
            ?
            .data)
    }

    /// Get the changelog of the file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the Terralith mod's v2.0.12 file's changelog
    /// let terralith_file_changelog = curseforge.get_mod_file_changelog(513688, 3606078)?;
    /// // This update had huge performance updates so check that that is mentioned in the changelog
    /// assert!(terralith_file_changelog.contains("performance"));
    /// # Ok(()) }
    /// ```
    pub fn get_mod_file_changelog(&self, mod_id: ID, file_id: ID) -> Result<String> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&format!("{}/", mod_id))?
                    .join("files/")?
                    .join(&format!("{}/", file_id))?
                    .join("changelog")?,
            )
            ?
            .data)
    }

    /// Get the download URL of the file with `file_id` of mod with `mod_id`
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get information about the file
    /// let terralith_mod_file = curseforge.get_mod_file(513688, 3606078)?;
    /// // Get the file's download url
    /// let download_url = curseforge.file_download_url(513688, 3606078)?;
    /// // They should be the same url
    /// assert!(Some(download_url) == terralith_mod_file.download_url);
    /// # Ok(()) }
    /// ```
    pub fn file_download_url(&self, mod_id: ID, file_id: ID) -> Result<url::Url> {
        Ok(self
            .get(
                API_URL_BASE
                    .join("mods/")?
                    .join(&format!("{}/", mod_id))?
                    .join("files/")?
                    .join(&format!("{}/", file_id))?
                    .join("download-url")?,
            )
            ?
            .data)
    }

    /// Get a list of files from the `file_ids` provided
    ///
    /// Example:
    /// ```rust
    /// # fn main() -> Result<(), furse::Error> {
    /// # let curseforge = furse::Furse::new(env!("CURSEFORGE_API_KEY"));
    /// // Get the 2 files
    /// let files = curseforge.get_files(vec![3144153, 3778436])?;
    /// // The response should have the same amount of files
    /// assert!(files.len() == 2);
    /// # Ok(()) }
    /// ```
    pub fn get_files(&self, file_ids: Vec<ID>) -> Result<Vec<File>> {
        let file_ids = GetFilesBody { file_ids };
        let mut files: Vec<File> = self
            .post(API_URL_BASE.join("mods/")?.join("files")?, &file_ids)
            ?
            .data;
        let mut actual_files = Vec::new();
        for file_id in file_ids.file_ids {
            if let Some(index) = files.iter().position(|file| file.id == file_id) {
                actual_files.push(files.swap_remove(index));
            }
        }
        Ok(actual_files)
    }
}
