use std::fmt;

use serde::{Serialize, Deserialize};

/// The base URL for the Facebook Graph API.
pub const GRAPH_BASE_URL: &str = "https://graph.facebook.com";

/// A builder for the `fields` query parameter in Graph API requests.
///
/// Used to select specific fields from API responses for efficiency.
#[derive(Debug, Default, Clone)]
pub struct Fields(Vec<String>);

impl Fields {
    /// Appends a field name to the field list.
    pub fn push(mut self, field: impl Into<String>) -> Self {
        self.0.push(field.into());
        self
    }

    /// Returns the field names as a slice.
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl<const N: usize> From<[&str; N]> for Fields {
    fn from(fields: [&str; N]) -> Self {
        Self(fields.into_iter().map(str::to_owned).collect())
    }
}

/// A builder for raw query parameters.
///
/// Used internally by [`GraphRequestBuilder`](crate::graph::GraphRequestBuilder)
/// to accumulate parameters like `limit`, `after`, `access_token`, etc.
#[derive(Debug, Clone, Default)]
pub struct QueryParams(Vec<(&'static str, String)>);

impl QueryParams {
    /// Creates an empty parameter list.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Inserts a key-value pair into the parameter list.
    pub fn insert(
        mut self,
        key: &'static str,
        value: impl Into<String>,
    ) -> Self {
        self.0.push((key, value.into()));
        self
    }

    /// Returns the parameters as a slice.
    pub fn as_slice(&self) -> &[(&'static str, String)] {
        &self.0
    }
}

impl<const N: usize> From<[(&'static str, &str); N]> for QueryParams {
    fn from(params: [(&'static str, &str); N]) -> Self {
        Self(
            params
                .into_iter()
                .map(|(k, v)| (k, v.to_owned()))
                .collect(),
        )
    }
}

/// Supported Facebook Graph API versions.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GraphVersion {
    /// Graph API v25.0
    #[default]
    V25_0,

    /// Graph API v24.0
    V24_0,

    /// Graph API v23.0
    V23_0,

    /// Graph API v22.0
    V22_0
}

impl fmt::Display for GraphVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::V25_0 => write!(f, "v25.0"),
            Self::V24_0 => write!(f, "v24.0"),
            Self::V23_0 => write!(f, "v23.0"),
            Self::V22_0 => write!(f, "v22.0"),
        }
    }
}

/// Cursor-based pagination tokens returned by Facebook.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GraphCursor {
    /// Cursor for traversing backwards through results.
    pub before: Option<String>,
    /// Cursor for traversing forward through results.
    pub after: Option<String>
}

/// Pagination metadata from a paginated Graph API response.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GraphPaging {
    /// Cursor-based pagination tokens.
    pub cursors: Option<GraphCursor>,
    /// URL for the next page of results.
    pub next: Option<String>,
    /// URL for the previous page of results.
    pub previous: Option<String>
}

/// A paginated response from the Facebook Graph API.
///
/// Wraps the returned items in `data` along with optional `paging` metadata
/// for cursor-based pagination.
///
/// Use [`has_more`](Self::has_more) to check if additional pages exist, then
/// extract the `after` cursor from `paging.cursors` to fetch the next page.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GraphConnection<T> {
    /// The items returned by the current page.
    pub data: Vec<T>,
    /// Pagination metadata for navigating pages.
    pub paging: Option<GraphPaging>
}

impl<T> GraphConnection<T> {
    /// Returns `true` if there are more pages of results available.
    pub fn has_more(&self) -> bool {
        self.paging
            .as_ref()
            .and_then(|p| p.next.as_ref())
            .is_some()
    }
}
