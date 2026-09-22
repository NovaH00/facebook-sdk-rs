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
    /// Supports text, photos, or a single video:
    /// - **Text-only post**: Pass an empty `media` slice.
    /// - **Photo post**: Pass one or more [`PostMedia::Photo`] items.
    /// - **Single video post**: Pass exactly one [`PostMedia::Video`] item.
    ///
    /// # Parameters
    ///
    /// * `message` — The text content of the post.
    /// * `media` — Media items ([`PostMedia`]) to attach.
    ///
    /// # Errors
    ///
    /// Returns [`GraphError::InvalidMedia`] if `media` contains both photos and videos,
    /// or if it contains multiple videos. Returns [`GraphError`] if the API request fails.
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
    /// // Post with photos
    /// let response = post_api
    ///     .create_post("Check this out!", vec![
    ///         PostMedia::photo_with_caption("https://example.com/photo1.jpg", "First photo"),
    ///         "https://example.com/photo2.jpg".into(),
    ///     ])
    ///     .await
    ///     .unwrap();
    ///
    /// // Post with single video
    /// let response = post_api
    ///     .create_post("Watch this video!", vec![
    ///         PostMedia::video("https://example.com/clip.mp4"),
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
        let photo_count = media.iter().filter(|m| matches!(m, PostMedia::Photo { .. })).count();
        let video_count = media.iter().filter(|m| matches!(m, PostMedia::Video { .. })).count();

        if photo_count > 0 && video_count > 0 {
            return Err(GraphError::InvalidMedia(
                "Cannot mix photos and videos in a single post. Meta Graph API only supports either photos or a single video.".to_string(),
            ));
        }

        if video_count > 1 {
            return Err(GraphError::InvalidMedia(
                "Cannot attach multiple videos to a single post. Meta Graph API allows at most 1 video per post.".to_string(),
            ));
        }

        let msg = message.into();

        // Single video post: publish directly to POST /me/videos
        if video_count == 1 {
            let PostMedia::Video { url, caption } = media.into_iter().next().unwrap() else {
                unreachable!();
            };

            let mut query = QueryParams::new()
                .insert("file_url", url.as_str());

            if !msg.is_empty() {
                query = query.insert("description", msg.as_str());
            } else if let Some(ref cap) = caption {
                query = query.insert("description", cap.as_str());
            }

            if let Some(ref cap) = caption {
                query = query.insert("title", cap.as_str());
            }

            #[derive(serde::Deserialize)]
            struct VideoResponse { id: String }

            let resp = self.page_graph_client
                .request(Method::POST, "/me/videos")
                .query_params(query)
                .send::<VideoResponse>()
                .await?;

            return Ok(CreatePostResponse::for_video(resp.id));
        }

        // Photo or text-only post
        let mut media_ids: Vec<String> = Vec::with_capacity(media.len());
        for item in &media {
            #[derive(serde::Deserialize)]
            struct UploadResponse { id: String }

            let PostMedia::Photo { url, caption } = item else {
                unreachable!();
            };

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

        // Build the feed POST params.
        let mut params = QueryParams::new()
            .insert("message", msg);

        for (i, media_id) in media_ids.iter().enumerate() {
            params = params.insert_owned(
                format!("attached_media[{}][media_fbid]", i),
                media_id.as_str(),
            );
        }

        #[derive(serde::Deserialize)]
        struct FeedResponse { id: String }

        let resp = self.page_graph_client
            .request(Method::POST, "/me/feed")
            .query_params(params)
            .send::<FeedResponse>()
            .await?;

        Ok(CreatePostResponse::for_post(resp.id))
    }
}

