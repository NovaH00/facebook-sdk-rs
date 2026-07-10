use std::marker::PhantomData;
use std::str::FromStr;

use strum_macros::{Display, EnumString};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;
use serde::{Deserialize};

pub const OAUTH_BASE_URL: &str = "https://www.facebook.com";

/// Facebook Graph API OAuth permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum AppPermission {
    /// Basic profile information. This permission is granted by default.
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

/// Facebook Login authorization flow modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken<Owner, TokenLifetime> {
    value: String,
    _owner: PhantomData<Owner>,
    _lifetime: PhantomData<TokenLifetime>
}

pub mod token_owner {
    #[derive(Debug, Clone, Copy)]
    pub struct User;

    #[derive(Debug, Clone, Copy)]
    pub struct Page;
}

pub mod token_lifetime {
    #[derive(Debug, Clone, Copy)]
    pub struct Short;

    #[derive(Debug, Clone, Copy)]
    pub struct Long;
}

pub type UserToken<L> = AccessToken<token_owner::User, L>;

pub type ShortLivedUserToken = UserToken<token_lifetime::Short>;
pub type LongLivedUserToken = UserToken<token_lifetime::Long>;

pub type PageToken = AccessToken<token_owner::Page, token_lifetime::Long>;

impl<O, L> AccessToken<O, L> {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            _owner: PhantomData,
            _lifetime: PhantomData,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct AccessTokenInfo {
    pub app_id: String,
    pub application: String,
    pub user_id: Option<String>,

    #[serde(rename = "type")]
    pub token_type: String,

    pub is_valid: bool,

    #[serde(with = "ts_seconds_option")]
    pub expires_at: Option<DateTime<Utc>>,

    #[serde(with = "ts_seconds_option")]
    pub issued_at: Option<DateTime<Utc>>,

    #[serde(with = "ts_seconds_option")]
    pub data_access_expires_at: Option<DateTime<Utc>>,

    pub scopes: Vec<AppPermission>,
}


impl AccessTokenInfo {
    /// Returns true if the token has no valid data access window.
    pub fn is_data_access_expired(&self) -> bool {
        self.data_access_expires_at
            .map(|dt| dt <= Utc::now())
            .unwrap_or(true)
    }

    /// Returns true if the token itself has expired (structural expiry).
    pub fn is_token_expired(&self) -> bool {
        match self.expires_at {
            Some(dt) if dt.timestamp() == 0 => false, // Facebook convention: 0 = never expires
            Some(dt) => dt <= Utc::now(),
            None => true,
        }
    }
}


