use crate::{v1::OpenCloudV1, v2::OpenCloudV2};
use reqwest::Client;
use zeroize::Zeroizing;

mod error;
#[cfg(feature = "rover")]
pub mod rover;
pub mod v1;
pub mod v2;

pub use error::Error;

pub struct OpenCloud {
    client: Client,
    secret: Zeroizing<String>,
}

impl OpenCloud {
    pub fn new(client: Client, secret: String) -> OpenCloud {
        OpenCloud {
            client,
            secret: Zeroizing::new(secret),
        }
    }

    pub fn v1<'c>(&'c self) -> OpenCloudV1<'c> {
        OpenCloudV1 { oc: self }
    }

    pub fn v2<'c>(&'c self) -> OpenCloudV2<'c> {
        OpenCloudV2 { oc: self }
    }
}

pub(crate) fn get_header_str<K: reqwest::header::AsHeaderName>(
    header_map: &reqwest::header::HeaderMap,
    key: K,
) -> Result<Option<&str>, reqwest::header::ToStrError> {
    header_map.get(key).map(|value| value.to_str()).transpose()
}

pub(crate) fn parse_header<'de, T: serde::Deserialize<'de>, K: reqwest::header::AsHeaderName>(
    header_map: &'de reqwest::header::HeaderMap,
    key: K,
) -> Result<Option<T>, reqwest::header::ToStrError> {
    Ok(get_header_str(header_map, key)?.map(|value| serde_json::from_str::<T>(value).unwrap()))
}
