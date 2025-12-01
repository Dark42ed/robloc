use crate::{
    Error, OpenCloud,
    v2::{datastore::OcV2Datastore, groups::OcV2Groups},
};
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use std::{borrow::Cow, marker::PhantomData};

pub mod datastore;
pub mod groups;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PageToken(pub String);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PagerOptions {
    pub max_page_size: u32,
    pub filter: Option<String>,
}

impl Default for PagerOptions {
    fn default() -> Self {
        PagerOptions {
            max_page_size: 20,
            filter: None,
        }
    }
}

impl PagerOptions {
    pub fn max_page_size(mut self, size: u32) -> Self {
        self.max_page_size = size;
        self
    }

    pub fn filter(self, filter: String) -> Self {
        self.with_filter(Some(filter))
    }

    pub fn with_filter(mut self, filter: Option<String>) -> Self {
        self.filter = filter;
        self
    }
}

pub struct Pager<'c, T> {
    v2: &'c OpenCloudV2<'c>,
    key: Cow<'static, str>,
    base_url: String,
    page_token: Option<PageToken>,
    options: PagerOptions,
    _phantom: PhantomData<fn() -> T>,
}

impl<'c, 'de, T: DeserializeOwned> Pager<'c, T> {
    pub fn new(
        key: Cow<'static, str>,
        base_url: String,
        v2: &'c OpenCloudV2<'c>,
        options: PagerOptions,
    ) -> Pager<'c, T> {
        Pager {
            v2,
            key,
            base_url,
            options,
            page_token: None,
            _phantom: PhantomData,
        }
    }

    async fn advance_page_inner(&mut self) -> Result<JsonValue, Error> {
        let mut url = self.base_url.clone();
        url += "&maxPageSize=";
        url += &self.options.max_page_size.to_string();
        if let Some(filter) = &self.options.filter {
            url += "&filter=";
            url += filter;
        }
        if let Some(token) = &self.page_token {
            url += "&pageToken=";
            url += &token.0;
        }
        let raw = self
            .v2
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Map<String, JsonValue>>()
            .await?;

        self.page_token = Some(PageToken(
            raw.get("nextPageToken")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        ));
        Ok(raw.get(&*self.key).unwrap().clone())
    }

    // Separated into a separate function so monomorphization doesn't blow everything up
    pub async fn advance_page(&mut self) -> Result<Vec<T>, Error> {
        if self.page_token.as_ref().is_some_and(|t| t.0.is_empty()) {
            return Ok(Vec::new());
        }
        return Ok(serde_json::from_value(self.advance_page_inner().await?).unwrap());
    }
}

pub struct OpenCloudV2<'c> {
    pub(crate) oc: &'c OpenCloud,
}

impl<'c> OpenCloudV2<'c> {
    fn request<U: IntoUrl>(&self, method: reqwest::Method, url: U) -> reqwest::RequestBuilder {
        self.oc
            .client
            .request(method, url)
            .header("x-api-key", &*self.oc.secret)
    }

    async fn get_default<T: DeserializeOwned>(&self, url: &str) -> Result<T, Error> {
        async fn get_default_internal(
            this: &OpenCloudV2<'_>,
            url: &str,
        ) -> Result<JsonValue, Error> {
            Ok(this
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?)
        }
        Ok(serde_json::from_value(get_default_internal(self, url).await?).unwrap())
    }

    fn get<U: IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::GET, url)
    }

    fn post<U: IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::POST, url)
    }

    fn patch<U: IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::PATCH, url)
    }

    pub fn groups(&'c self) -> OcV2Groups<'c> {
        OcV2Groups { v2: self }
    }

    pub fn datastore(&'c self) -> OcV2Datastore<'c> {
        OcV2Datastore { v2: self }
    }
}
