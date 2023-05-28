pub mod file_calls;
pub mod fingerprint_calls;
pub mod mod_calls;
use super::structures::common_structs::Pagination;
use serde::{Deserialize, Serialize};

// API responses are returned in this 'response' json structure with the actual results in `data` and optional `pagination`
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "serde_deny_unknown_fields", serde(deny_unknown_fields))]
pub(crate) struct Response<T> {
    data: T,
    pagination: Option<Pagination>,
}
