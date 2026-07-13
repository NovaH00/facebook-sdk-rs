# facebook-sdk-rs

Rust SDK for the [Facebook Graph API](https://developers.facebook.com/docs/graph-api)
and [Messenger Platform](https://developers.facebook.com/docs/messenger-platform).

## Features

- **Typed access tokens** — Phantom-type markers prevent mixing up user and page tokens at compile time
- **OAuth 2.0 flow** — Full Facebook Login: authorization URL generation, code exchange, token extension
- **Token debugging** — Inspect any token's validity, scopes, and expiry via `/debug_token`
- **Pagination** — Cursor-based pagination with automatic deduplication across all list APIs
- **Pages API** — List managed Pages, create Page-scoped API clients
- **Posts API** — List, like, unlike, delete, and get Page posts
- **Conversations API** — List Messenger conversations with automatic recipient resolution
- **Messages API** — List messages, send text replies with configurable `messaging_type`
- **Webhooks API** — Subscribe/unsubscribe Pages to webhook fields, deserialize incoming events
- **Structured errors** — Facebook API error codes, subcodes, fbtrace_id, and transient flags

## Architecture

The SDK is organized in three layers:

```
facebook-sdk-rs
├── auth          — OAuth 2.0, token types, token debugging
├── graph         — Graph API client, request builder, pagination
└── api           — High-level domain APIs
    ├── user      — User profile (/me)
    ├── page      — Page management, token extraction
    ├── post      — Post listing and operations (like, unlike, delete)
    ├── conversation — Messenger conversation listing
    ├── message   — Message history and send API
    └── webhook   — Subscription management and event deserialization
```

### Service Chain

The domain APIs follow a layered ownership pattern:

```
UserApi (user token)
  └── get_page_api() → PageApi (user token)
        └── get_graph_client(page) → PageGraphClient (page token)

PageGraphClient (page token) used directly with:
  ├── PostApi::new(client)
  ├── ConversationApi::new(client)
  ├── MessageApi::new(client)
  └── WebhookApi::new(client)
```

Each API carries only its GraphClient — you pass IDs (page_id, conversation_id, etc.)
as method arguments when making calls.

## Quick Start

```rust
use facebook_sdk_rs::auth::{AppClient, AppPermission, LongLivedUserToken};
use facebook_sdk_rs::api::{
    UserApi,
    conversation::ConversationApi,
    message::{MessageApi, MessagingType, SendMessagePayload},
};

// 1. Create an AppClient with Facebook app credentials
let app = AppClient::new(
    "your-app-id",
    "your-app-secret",
    "https://your-redirect-url.com/callback",
);

// 2. Build OAuth URL and redirect the user
let login_url = app.get_oauth_url(
    "state123",
    &[AppPermission::PagesShowList, AppPermission::PagesMessaging],
    None,
)?;

// 3. Exchange the authorization code for a long-lived user token
let user_token: LongLivedUserToken = app.login("auth-code-from-callback").await?;

// 4. Start making API calls
let user_api = UserApi::new(user_token);
let user = user_api.me().await?;
println!("Hello, {}!", user.name);

let page_api = user_api.get_page_api();
let pages = page_api.collect_paginated_pages(None).await?;

for page in &pages {
    let client = page_api.get_graph_client(page)?;
    let conv_api = ConversationApi::new(client.clone());
    let conversations = conv_api.collect_paginated_conversations(None).await?;

    for conv in &conversations {
        let msg_api = MessageApi::new(client.clone());
        let recipient_id = &conv.recipient(&page.id).unwrap().id;
        let response = msg_api.send_message(recipient_id, SendMessagePayload::Text { text: "Hello!".to_string() }, MessagingType::Response).await?;
        println!("Sent: {}", response.message_id);
    }
}
```

## API Reference

### `auth` module — Authentication & Token Management

#### `AppClient`

| Method | Description |
|--------|-------------|
| `new(app_id, app_secret, redirect_url)` | Creates a new AppClient |
| `set_version(version)` | Overrides the Graph API version |
| `get_oauth_url(state, scope, auth_type)` | Builds the Facebook OAuth consent URL |
| `login(code)` | Exchanges an authorization code for a long-lived user token |
| `debug_token(token)` | Inspects any access token's validity and metadata |

#### `AccessToken<O, L>`

| Method | Description |
|--------|-------------|
| `new(value)` | Creates a token from a string |
| `as_str()` | Returns the token string |

#### Type Aliases

| Alias | Represents |
|-------|------------|
| `ShortLivedUserToken` | Short-lived (1-2 hour) user token |
| `LongLivedUserToken` | Long-lived (60 day) user token |
| `PageToken` | Long-lived page token |

#### `AccessTokenInfo`

| Method | Description |
|--------|-------------|
| `is_data_access_expired()` | Checks if the data access window has expired |
| `is_token_expired()` | Checks if the token itself has expired |

#### `AppPermission`

OAuth permission scopes: `PublicProfile`, `Email`, `UserFriends`, `PagesShowList`,
`PagesReadEngagement`, `PagesReadUserContent`, `PagesManagePosts`,
`PagesManageEngagement`, `PagesManageMetadata`, `PagesManageCta`, `PagesMessaging`,
`InstagramBasic`, `InstagramManageMessages`, `InstagramManageComments`,
`InstagramContentPublish`, `InstagramManageInsights`, `AdsRead`, `AdsManagement`,
`BusinessManagement`, `GroupsAccessMemberInfo`, `PublishVideo`.

#### `AppAuthType`

Re-authorization modifiers: `Rerequest`, `Reauthorize`, `Reauthenticate`.

#### `AuthError`

| Variant | Description |
|---------|-------------|
| `Url` | URL parse error |
| `Request` | HTTP request failed |
| `MissingAccessToken` | Facebook response missing access token |

---

### `graph` module — Graph API Client & Request Builder

#### `GraphClient<O, L>`

| Method | Description |
|--------|-------------|
| `new(access_token)` | Creates a new GraphClient |
| `request(method, endpoint)` | Starts building a request |

#### Type Aliases

| Alias | Represents |
|-------|------------|
| `UserGraphClient` | `GraphClient` with a long-lived user token |
| `PageGraphClient` | `GraphClient` with a long-lived page token |

#### `GraphRequestBuilder<O, L>`

| Method | Description |
|--------|-------------|
| `base_url(url)` | Overrides the default Graph API URL |
| `version(version)` | Sets the API version |
| `fields([...])` | Sets the `fields` parameter for field selection |
| `query([(...)])` | Adds raw query parameters |
| `limit(n)` | Sets the pagination `limit` parameter |
| `after(cursor)` | Sets the `after` cursor for cursor-based pagination |
| `send::<T>()` | Sends the request and deserializes the response |

#### `GraphConnection<T>`

| Method | Description |
|--------|-------------|
| `has_more()` | Returns true if more pages are available |

#### `GraphVersion`

Variants: `V25_0`, `V24_0`, `V23_0`, `V22_0`. Defaults to `V25_0`.

#### `GraphError`

| Variant | Description |
|---------|-------------|
| `UrlParseError` | Failed to parse the request URL |
| `Request` | HTTP transport error |
| `Facebook { message, code, error_subcode, fbtrace_id, is_transient }` | Structured Facebook API error |
| `MissingAccessToken { origin, message }` | Missing required access token |


---

### `api` module — Domain APIs

#### `Participant`

| Field | Type |
|-------|------|
| `id` | `String` |
| `name` | `String` |
| `email` | `Option<String>` |

#### `UserApi`

| Method | Description |
|--------|-------------|
| `new(user_graph_client)` | Creates a new UserApi |
| `me()` | Fetches the authenticated user's profile (`GET /me`) |
| `get_page_api()` | Creates a PageApi for listing/managing Pages |

#### `User`

| Method | Description |
|--------|-------------|
| `fields()` | Returns field names for API selection |

#### `PageApi`

| Method | Description |
|--------|-------------|
| `new(user_graph_client)` | Creates a new PageApi |
| `first_paginated_pages(limit)` | Fetches first page of managed Pages |
| `next_paginated_pages(limit, current)` | Fetches next page using cursor |
| `collect_paginated_pages(limit)` | Fetches all Pages with auto-pagination |
| `get_graph_client(page)` | Extracts a PageGraphClient from a Page |
| `get_user_info(uid)` | Looks up a user by PSID |

#### `Page`

| Method | Description |
|--------|-------------|
| `fields()` | Returns field names for API selection |

#### `PageScopedUser`

| Method | Description |
|--------|-------------|
| `fields()` | Returns field names for API selection |

#### `PostApi`

| Method | Description |
|--------|-------------|
| `new(page_graph_client)` | Creates a new PostApi |
| `first_paginated_posts(limit)` | Fetches first page of posts |
| `next_paginated_posts(limit, current)` | Fetches next page using cursor |
| `collect_paginated_posts(limit)` | Fetches all posts with auto-pagination |

#### `Post`

| Method | Description |
|--------|-------------|
| `fields()` | Returns field names for API selection |

#### `PostOperations` trait

| Method | Description |
|--------|-------------|
| `like_post(post_id)` | Likes the given post (`POST /{post_id}/likes`) |
| `unlike_post(post_id)` | Removes like from the given post (`DELETE /{post_id}/likes`) |
| `delete_post(post_id)` | Deletes the given post (`DELETE /{post_id}`) |
| `get_post(id)` | Fetches a single post by ID |

Implemented by: `PostApi`, `PageApi`

#### `ConversationApi`

| Method | Description |
|--------|-------------|
| `new(page_graph_client)` | Creates a new ConversationApi |
| `first_paginated_conversations(limit)` | Fetches first page of conversations |
| `next_paginated_conversations(limit, current)` | Fetches next page using cursor |
| `collect_paginated_conversations(limit)` | Fetches all conversations with auto-pagination |
| `get_conversation(conversation_id)` | Fetches a single conversation by ID |

#### `Conversation`

| Method | Description |
|--------|-------------|
| `fields()` | Returns field names for API selection |
| `recipient(page_id)` | Returns the non-Page participant |

#### `MessageApi`

| Method | Description |
|--------|-------------|
| `new(page_graph_client)` | Creates a new MessageApi |
| `first_paginated_messages(conversation_id, limit)` | Fetches first page of messages |
| `next_paginated_messages(conversation_id, limit, current)` | Fetches next page using cursor |
| `collect_paginated_messages(conversation_id, limit)` | Fetches all messages with auto-pagination |
| `send_message(recipient_id, payload, messaging_type)` | Sends a text or media message (`POST /me/messages`) |

#### `Message`

| Method | Description |
|--------|-------------|
| `fields()` | Returns field names for API selection |

#### `MessagingType`

| Variant | Serialized as | Use case |
|---------|--------------|----------|
| `Response` | `"RESPONSE"` | Reply within 24h window |
| `Update` | `"UPDATE"` | Proactive update (e.g. order confirmation) |
| `MessageTag` | `"MESSAGE_TAG"` | Tagged message bypassing 24h limit |

#### `AttachmentData`

Variants: `Image`, `Video`, `File`, `Other(serde_json::Value)`

#### `SendMessageResponse`

| Field | Type |
|-------|------|
| `message_id` | `String` |
| `recipient_id` | `String` |

#### `WebhookApi`

| Method | Description |
|--------|-------------|
| `new(page_graph_client)` | Creates a new WebhookApi |
| `subscribe(page_id, fields)` | Subscribes the Page to webhook fields |
| `unsubscribe(page_id, fields)` | Unsubscribes the Page from specific fields |
| `unsubscribe_all(page_id)` | Unsubscribes the Page from all fields |
| `list(page_id)` | Lists apps installed on the Page |

#### `WebhookField`

Variants: `Messages`, `MessageDeliveries`, `MessageReactions`, `MessagingPostbacks`, `Feed`

#### `SubscribedApp`

| Field | Type |
|-------|------|
| `category` | `String` |
| `link` | `Option<String>` |
| `name` | `String` |
| `id` | `String` |

#### `events` submodule — Webhook payload types

| Type | Description |
|------|-------------|
| `WebhookPayload` | Top-level webhook POST payload |
| `WebhookEntry` | An entry for one page |
| `WebhookMessagingEvent` | Message, Delivery, or Reaction event |
| `WebhookParticipant` | Sender/recipient with PSID only |
| `MessageContent` | Message text, mid, attachments, echo flag |
| `DeliveryInfo` | Delivery receipt data |
| `ReactionInfo` | Reaction data |
| `Attachment` | Message attachment |
| `AttachmentPayload` | Attachment URL/title/sticker |

## Error Handling

The SDK uses two error types:

- **`AuthError`** — OAuth flow errors (URL parsing, HTTP failures, missing tokens)
- **`GraphError`** — API request errors (URL parsing, HTTP failures, structured Facebook errors with code/subcode/fbtrace)

Facebook API errors include the raw error code, subcode, trace ID, and transient flag,
allowing you to implement retry logic:

```rust
match result {
    Err(GraphError::Facebook { is_transient: Some(true), .. }) => {
        // Retry with backoff
    }
    Err(e) => {
        // Fail immediately
    }
    Ok(val) => { /* success */ }
}
```

## Cargo Features

The `utoipa` feature enables OpenAPI schema generation via the [utoipa](https://crates.io/crates/utoipa) crate:

```toml
[dependencies]
facebook-sdk-rs = { git = "...", features = ["utoipa"] }
```

All public model types gain `#[derive(utoipa::ToSchema)]` when this feature is active.

## License

MIT
