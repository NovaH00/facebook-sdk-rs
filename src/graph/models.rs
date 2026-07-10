use std::fmt;

use serde::{Serialize, Deserialize};

pub const GRAPH_BASE_URL: &str = "https://graph.facebook.com";

#[derive(Debug, Default, Clone)]
pub struct Fields(Vec<String>);
impl Fields {

    pub fn push(mut self, field: impl Into<String>) -> Self {
        self.0.push(field.into());
        self
    }

    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl<const N: usize> From<[&str; N]> for Fields {
    fn from(fields: [&str; N]) -> Self {
        Self(fields.into_iter().map(str::to_owned).collect())
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueryParams(Vec<(&'static str, String)>);

impl QueryParams {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(
        mut self,
        key: &'static str,
        value: impl Into<String>,
    ) -> Self {
        self.0.push((key, value.into()));
        self
    }

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


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GraphVersion {
    #[default]
    V25_0,

    V24_0,
    V23_0,
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GraphCursor {
    pub before: Option<String>,
    pub after: Option<String>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GraphPaging {
    pub cursors: Option<GraphCursor>,
    pub next: Option<String>,
    pub previous: Option<String>
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GraphConnection<T> {
    pub data: Vec<T>,
    pub paging: Option<GraphPaging>
}

impl<T> GraphConnection<T> {
    pub fn has_more(&self) -> bool {
        self.paging
            .as_ref()
            .and_then(|p| p.next.as_ref())
            .is_some()
    }
}
