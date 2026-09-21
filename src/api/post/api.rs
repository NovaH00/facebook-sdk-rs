use crate::graph::{
    PageGraphClient,
    GraphConnection,
    GraphError,
    Method,
    QueryParams,
};
use super::models::{Post, CreatePostResponse, PostMedia};


/// High-level API for reading a Page's posts.
///
/// Provides paginated access to `GET /me/posts` with automatic deduplication.
/// For post operations (like, unlike, delete), use the [`PostOperations`](super::PostOperations)
/// trait which is implemented by this type.
///
/// # Example
///
/// ```rust,no_run
/// # async fn _test() {
/// # use facebook_sdk_rs::api::post::PostApi;
/// # use facebook_sdk_rs::graph::PageGraphClient;
/// # let client: PageGraphClient = unimplemented!();
/// let post_api = PostApi::new(client);
/// let posts = post_api.collect_paginated_posts(None).await.unwrap();
/// for post in &posts {
///     println!("{}", post.message.as_deref().unwrap_or("(no text)"));
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PostApi {
    page_graph_client: PageGraphClient,
}

impl PostApi {
    /// Creates a new `PostApi` from a Page-scoped Graph client.
    pub fn new(
        page_graph_client: PageGraphClient
    ) -> Self {
        Self {
            page_graph_client,
        }
    }

    /// Fetches the first page of the Page's posts.
    ///
    /// Calls `GET /me/posts`. Use [`next_paginated_posts`](Self::next_paginated_posts)
    /// to fetch subsequent pages.
    pub async fn first_paginated_posts(
        &self,
        limit: Option<u32>
    ) -> Result<GraphConnection<Post>, GraphError> {

        let mut request = self.page_graph_client
            .request(Method::GET, "/me/posts")
            .fields(Post::fields());


        if let Some(limit) = limit {
            request = request.limit(limit);
        };

        request
            .send::<GraphConnection<Post>>()
            .await
    }

    /// Fetches the next page of the Page's posts using cursor pagination.
    pub async fn next_paginated_posts(
        &self,
        limit: Option<u32>,
        current: &GraphConnection<Post>
    ) -> Result<GraphConnection<Post>, GraphError> {
        let after = current.paging
            .as_ref()
            .and_then(|p| p.cursors.as_ref())
            .and_then(|c| c.after.as_deref());

        let mut request = self.page_graph_client
            .request(Method::GET, "/me/posts")
            .fields(Post::fields());


        if let Some(limit) = limit {
            request = request.limit(limit);
        };

        if let Some(cursor) = after {
            request = request.after(cursor);
        }

        request.send::<GraphConnection<Post>>().await
    }

    /// Fetches all of the Page's posts, handling pagination automatically.
    ///
    /// Deduplicates results by post ID.
    pub async fn collect_paginated_posts(
        &self,
        limit: Option<u32>,
    ) -> Result<Vec<Post>, GraphError> {
        let mut all = Vec::new();
        let mut seen = std::collections::HashSet::new();
        let mut conn = self.first_paginated_posts(limit).await?;

        loop {
            if conn.data.is_empty() {
                break;
            }

            let unique: Vec<Post> = conn.data
                .drain(..)
                .filter(|p| seen.insert(p.id.clone()))
                .collect();

            if unique.is_empty() {
                break;
            }

            all.extend(unique);

            if !conn.has_more() {
                break;
            }

            conn = self.next_paginated_posts(limit, &conn).await?;
        }

        Ok(all)
    }

    /// Creates a new post on the Page.
    ///
    /// Calls `POST /me/feed`. If `media` is non-empty, each photo is first
    /// uploaded as an unpublished photo via `POST /me/photos`, and the resulting
    /// photo IDs are attached to the feed post. An empty `media` creates a text-only post.
    ///
    /// # Parameters
    ///
    /// * `message` — The text content of the post.
    /// * `media` — Media items ([`PostMedia`]) to attach. Pass an empty
    ///   `Vec` for a text-only post. Plain `&str` and `String` URLs can also be
    ///   passed via `PostMedia` conversions.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError`] if any photo upload fails or if the feed post
    /// request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # async fn _test() {
    /// # use facebook_sdk_rs::api::post::{PostApi, PostMedia};
    /// # use facebook_sdk_rs::graph::PageGraphClient;
    /// # let client: PageGraphClient = unimplemented!();
    /// let post_api = PostApi::new(client);
    ///
    /// // Text-only post
    /// let response = post_api
    ///     .create_post("Hello from the SDK!", vec![])
    ///     .await
    ///     .unwrap();
    ///
    /// // Post with photos (with or without caption)
    /// let response = post_api
    ///     .create_post("Check this out!", vec![
    ///         PostMedia::photo_with_caption("https://example.com/photo1.jpg", "First photo"),
    ///         "https://example.com/photo2.jpg".into(),
    ///     ])
    ///     .await
    ///     .unwrap();
    /// println!("Created post: {}", response.id);
    /// # }
    /// ```
    pub async fn create_post(
        &self,
        message: impl Into<String>,
        media: Vec<PostMedia>,
    ) -> Result<CreatePostResponse, GraphError> {
        // Step 1: upload each photo as unpublished photo, collect IDs.
        let mut media_ids: Vec<String> = Vec::with_capacity(media.len());
        for item in &media {
            #[derive(serde::Deserialize)]
            struct UploadResponse { id: String }

            let PostMedia::Photo { url, caption } = item;
            let mut query = QueryParams::new()
                .insert("url", url.as_str())
                .insert("published", "false");

            if let Some(caption) = caption {
                query = query.insert("caption", caption.as_str());
            }

            let resp = self.page_graph_client
                .request(Method::POST, "/me/photos")
                .query_params(query)
                .send::<UploadResponse>()
                .await?;

            media_ids.push(resp.id);
        }

        // Step 2: build the feed POST params.
        // attached_media[N][media_fbid] is the Graph API convention for multi-media posts.
        let mut params = QueryParams::new()
            .insert("message", message.into());

        for (i, media_id) in media_ids.iter().enumerate() {
            params = params.insert_owned(
                format!("attached_media[{}][media_fbid]", i),
                media_id.as_str(),
            );
        }

        self.page_graph_client
            .request(Method::POST, "/me/feed")
            .query_params(params)
            .send::<CreatePostResponse>()
            .await
    }
}

