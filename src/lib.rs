//! Rust SDK for the Facebook Graph API.
//!
//! Provides typed access tokens, OAuth 2.0 authentication, paginated API responses,
//! and wrappers for Pages, Posts, Conversations, Messages, and Webhooks.
//!
//! # Architecture
//!
//! The SDK is organized into three layers:
//!
//! - [`auth`] — OAuth 2.0 flow, token types, token debugging
//! - [`graph`] — Graph API client, request builder, pagination, error types
//! - [`api`] — High-level domain APIs (User, Page, Post, Conversation, Message, Webhook)
//!
//! # Quick Start
//!
//! ```rust,no_run
//! # async fn _test() {
//! use facebook_sdk_rs::auth::{AuthClient, AppPermission, LongLivedUserToken};
//! use facebook_sdk_rs::graph::{GraphClient, UserGraphClient};
//! use facebook_sdk_rs::api::user::UserApi;
//! use facebook_sdk_rs::api::page::PageApi;
//!
//! // 1. Create an AuthClient with your Facebook app credentials
//! let app_client = AuthClient::new(
//!     "your-app-id",
//!     "your-app-secret",
//!     "https://your-redirect-url.com/callback",
//! );
//!
//! // 2. Build the OAuth login URL
//! let login_url = app_client.get_oauth_url(
//!     "state123",
//!     &[AppPermission::PagesShowList, AppPermission::PagesMessaging],
//!     None,
//! ).unwrap();
//!
//! // 3. Exchange the authorization code for a long-lived user token
//! let user_token: LongLivedUserToken = app_client.login("auth-code-from-callback").await.unwrap();
//!
//! // 4. Start making API calls
//! let user_api = UserApi::new(UserGraphClient::new(user_token));
//! let user = user_api.me().await.unwrap();
//! let page_api = user_api.get_page_api();
//! # }
//! ```
pub mod auth;
pub mod graph;
pub mod api;
