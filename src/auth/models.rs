use std::marker::PhantomData;
use std::str::FromStr;

use strum_macros::{Display, EnumString};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;
use serde::{Deserialize, Serialize, Deserializer, Serializer};

/// The base URL for Facebook's OAuth dialog.
pub const OAUTH_BASE_URL: &str = "https://www.facebook.com";

/// Facebook Graph API OAuth permission scopes.
///
/// These are the standard permissions your app can request during the
/// Facebook Login flow. Pass a slice of these to
/// [`AppClient::get_oauth_url`](crate::auth::AppClient::get_oauth_url).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AppPermission {
    /// Basic profile information. Granted by default.
    #[strum(serialize = "public_profile")]
    PublicProfile,

    /// The user's primary email address.
    #[strum(serialize = "email")]
    Email,

    /// The user's friends who have also authorized your app.
    #[strum(serialize = "user_friends")]
    UserFriends,

    /// List Facebook Pages the user can access or manage.
    #[strum(serialize = "pages_show_list")]
    PagesShowList,

    /// Read Page metadata, posts, reactions, and other engagement data.
    #[strum(serialize = "pages_read_engagement")]
    PagesReadEngagement,

    /// Read user-generated content on a Page, such as visitor posts.
    #[strum(serialize = "pages_read_user_content")]
    PagesReadUserContent,

    /// Create, edit, and delete posts on a Page.
    #[strum(serialize = "pages_manage_posts")]
    PagesManagePosts,

    /// Moderate Page comments and reactions.
    #[strum(serialize = "pages_manage_engagement")]
    PagesManageEngagement,

    /// Manage Page metadata, including webhook subscriptions.
    #[strum(serialize = "pages_manage_metadata")]
    PagesManageMetadata,

    /// Manage a Page's call-to-action button.
    #[strum(serialize = "pages_manage_cta")]
    PagesManageCta,

    /// Send and receive Messenger messages on behalf of a Page.
    #[strum(serialize = "pages_messaging")]
    PagesMessaging,

    /// Read basic information for an Instagram Business or Creator account.
    #[strum(serialize = "instagram_basic")]
    InstagramBasic,

    /// Read and reply to Instagram Direct Messages.
    #[strum(serialize = "instagram_manage_messages")]
    InstagramManageMessages,

    /// Read, moderate, and reply to Instagram comments.
    #[strum(serialize = "instagram_manage_comments")]
    InstagramManageComments,

    /// Publish media to an Instagram Business account.
    #[strum(serialize = "instagram_content_publish")]
    InstagramContentPublish,

    /// Read Instagram account insights and analytics.
    #[strum(serialize = "instagram_manage_insights")]
    InstagramManageInsights,

    /// Read advertising accounts, campaigns, and performance data.
    #[strum(serialize = "ads_read")]
    AdsRead,

    /// Create and manage advertising campaigns.
    #[strum(serialize = "ads_management")]
    AdsManagement,

    /// Manage Meta Business assets such as Pages, ad accounts, and users.
    #[strum(serialize = "business_management")]
    BusinessManagement,

    /// Access information about members of Facebook Groups.
    #[strum(serialize = "groups_access_member_info")]
    GroupsAccessMemberInfo,

    /// Publish videos or create live broadcasts.
    #[strum(serialize = "publish_video")]
    PublishVideo,
}

impl<'de> Deserialize<'de> for AppPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        AppPermission::from_str(&value)
            .map_err(serde::de::Error::custom)
    }
}

/// Modifiers for the Facebook Login re-authorization flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum AppAuthType {
    /// Ask again for permissions that the user previously declined.
    #[strum(serialize = "rerequest")]
    Rerequest,

    /// Force the user through the authorization flow again.
    #[strum(serialize = "reauthorize")]
    Reauthorize,

    /// Require the user to re-enter their Facebook credentials before continuing.
    #[strum(serialize = "reauthenticate")]
    Reauthenticate,
}

impl Serialize for AppAuthType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AppAuthType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        <Self as std::str::FromStr>::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// A Facebook access token with phantom-type markers for owner and lifetime.
///
/// The type parameters prevent accidentally using a token at the wrong scope:
///
/// - `O` — token owner (User, Page)
/// - `L` — token lifetime (Short, Long)
///
/// Use the type aliases [`ShortLivedUserToken`], [`LongLivedUserToken`], or [`PageToken`]
/// for convenience.
///
/// ```rust,no_run
/// use facebook_sdk_rs::auth::{AccessToken, LongLivedUserToken};
///
/// let token: LongLivedUserToken = AccessToken::new("EA...abc123");
/// assert_eq!(token.as_str(), "EA...abc123");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken<Owner, TokenLifetime> {
    value: String,
    _owner: PhantomData<Owner>,
    _lifetime: PhantomData<TokenLifetime>
}

/// Phantom-type module for access token owners.
pub mod token_owner {
    /// Marker for user-scoped access tokens.
    #[derive(Debug, Clone, Copy)]
    pub struct User;

    /// Marker for page-scoped access tokens.
    #[derive(Debug, Clone, Copy)]
    pub struct Page;
}

/// Phantom-type module for access token lifetimes.
pub mod token_lifetime {
    /// Marker for short-lived tokens (typically 1-2 hours).
    #[derive(Debug, Clone, Copy)]
    pub struct Short;

    /// Marker for long-lived tokens (typically 60 days).
    #[derive(Debug, Clone, Copy)]
    pub struct Long;
}

/// Short-lived user access token (1-2 hour expiry).
pub type ShortLivedUserToken = AccessToken<token_owner::User, token_lifetime::Short>;

/// Long-lived user access token (60 day expiry).
pub type LongLivedUserToken = AccessToken<token_owner::User, token_lifetime::Long>;

/// Long-lived page access token.
pub type PageToken = AccessToken<token_owner::Page, token_lifetime::Long>;

impl<O, L> AccessToken<O, L> {
    /// Creates a new access token from a string value.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            _owner: PhantomData,
            _lifetime: PhantomData,
        }
    }

    /// Returns the token string.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

/// Metadata about an access token from the `/debug_token` endpoint.
///
///
/// Returned by [`AppClient::debug_token`](crate::auth::AppClient::debug_token).
#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AccessTokenInfo {
    /// The app ID that owns this token.
    pub app_id: String,
    /// The app name that owns this token.
    pub application: String,
    /// The user ID the token belongs to (None for page tokens).
    pub user_id: Option<String>,
    /// Token type (e.g. "USER", "PAGE", "APP").
    #[serde(rename = "type")]
    pub token_type: String,
    /// Whether the token is currently valid.
    pub is_valid: bool,
    /// When the token expires (None if never expires).
    #[serde(with = "ts_seconds_option")]
    pub expires_at: Option<DateTime<Utc>>,
    /// When the token was issued.
    #[serde(with = "ts_seconds_option")]
    pub issued_at: Option<DateTime<Utc>>,
    /// When the token's data access expires.
    #[serde(with = "ts_seconds_option")]
    pub data_access_expires_at: Option<DateTime<Utc>>,
    /// Permissions granted to this token.
    pub scopes: Vec<AppPermission>,
}

impl AccessTokenInfo {
    /// Returns `true` if the token's data access window has expired.
    ///
    /// After this date, the token can still be used but may not return
    /// user-specific data depending on Facebook's data policies.
    pub fn is_data_access_expired(&self) -> bool {
        self.data_access_expires_at
            .map(|dt| dt <= Utc::now())
            .unwrap_or(true)
    }

    /// Returns `true` if the token itself has structurally expired.
    ///
    /// A timestamp of 0 is Facebook's convention for "never expires"
    /// (e.g., page tokens with eternal expiry).
    pub fn is_token_expired(&self) -> bool {
        match self.expires_at {
            Some(dt) if dt.timestamp() == 0 => false,
            Some(dt) => dt <= Utc::now(),
            None => true,
        }
    }
}
