use crate::{api_calls::Response, Furse, Result};
use reqwest::{IntoUrl, Url};
use serde::{de::DeserializeOwned, Serialize};

lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.curseforge.com/v1/").unwrap();
}

impl Furse {
    /// Perform a GET request to `url` and deserialise to `T`
    pub(crate) fn get<T>(&self, url: impl IntoUrl) -> Result<Response<T>>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .header("x-api-key", &self.api_key)
            .send()?;

        match response.error_for_status_ref() {
            Ok(_) => Ok(response.json()?),
            Err(error) => Err(crate::Error::RequestError(error, response)),
        }
    }

    /// Perform a GET request to `url` with `body`
    pub(crate) fn post<T, B>(&self, url: impl IntoUrl, body: &B) -> Result<Response<T>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        Ok(self
            .client
            .post(url)
            .json(body)
            .header("x-api-key", &self.api_key)
            .send()
            ?
            .error_for_status()?
            .json()
            ?)
    }
}
