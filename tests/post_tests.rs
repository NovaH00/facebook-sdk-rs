use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};
use facebook_sdk_rs::auth::PageToken;
use facebook_sdk_rs::graph::{GraphClient, GraphError};
use facebook_sdk_rs::api::post::{PostApi, PostMedia};

#[tokio::test]
async fn test_create_post_video_only() {
    let mock_server = MockServer::start().await;

    // Directly publishes to POST /v25.0/me/videos without published=false
    Mock::given(method("POST"))
        .and(path("/v25.0/me/videos"))
        .and(query_param("file_url", "https://example.com/clip.mp4"))
        .and(query_param("description", "Watch this clip!"))
        .and(query_param("title", "Clip title"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "id": "video_post_999" }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let page_token = PageToken::new("test_page_token");
    let client = GraphClient::new(page_token).with_base_url(mock_server.uri());
    let post_api = PostApi::new(client);

    let media = vec![
        PostMedia::video_with_caption("https://example.com/clip.mp4", "Clip title"),
    ];

    let resp = post_api
        .create_post("Watch this clip!", media)
        .await
        .unwrap();

    assert_eq!(resp.id, "video_post_999");
    assert_eq!(resp.post_url, "https://www.facebook.com/watch/?v=video_post_999");
}

#[tokio::test]
async fn test_create_post_photos_only() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v25.0/me/photos"))
        .and(query_param("url", "https://example.com/photo1.jpg"))
        .and(query_param("published", "false"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "id": "photo_1_id" }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v25.0/me/photos"))
        .and(query_param("url", "https://example.com/photo2.jpg"))
        .and(query_param("published", "false"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "id": "photo_2_id" }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v25.0/me/feed"))
        .and(query_param("message", "Two photos post"))
        .and(query_param("attached_media[0][media_fbid]", "photo_1_id"))
        .and(query_param("attached_media[1][media_fbid]", "photo_2_id"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({ "id": "feed_post_123" }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let page_token = PageToken::new("test_page_token");
    let client = GraphClient::new(page_token).with_base_url(mock_server.uri());
    let post_api = PostApi::new(client);

    let media = vec![
        PostMedia::photo("https://example.com/photo1.jpg"),
        PostMedia::photo("https://example.com/photo2.jpg"),
    ];

    let resp = post_api
        .create_post("Two photos post", media)
        .await
        .unwrap();

    assert_eq!(resp.id, "feed_post_123");
    assert_eq!(resp.post_url, "https://www.facebook.com/feed_post_123");
}

#[tokio::test]
async fn test_create_post_mixed_media_error() {
    let page_token = PageToken::new("test_page_token");
    let client = GraphClient::new(page_token);
    let post_api = PostApi::new(client);

    let media = vec![
        PostMedia::photo("https://example.com/photo.jpg"),
        PostMedia::video("https://example.com/video.mp4"),
    ];

    let err = post_api
        .create_post("Mixed post", media)
        .await
        .unwrap_err();

    match err {
        GraphError::InvalidMedia(msg) => {
            assert!(msg.contains("Cannot mix photos and videos"));
        }
        other => panic!("Expected InvalidMedia, got {:?}", other),
    }
}

#[tokio::test]
async fn test_create_post_multiple_videos_error() {
    let page_token = PageToken::new("test_page_token");
    let client = GraphClient::new(page_token);
    let post_api = PostApi::new(client);

    let media = vec![
        PostMedia::video("https://example.com/video1.mp4"),
        PostMedia::video("https://example.com/video2.mp4"),
    ];

    let err = post_api
        .create_post("Two videos post", media)
        .await
        .unwrap_err();

    match err {
        GraphError::InvalidMedia(msg) => {
            assert!(msg.contains("Cannot attach multiple videos"));
        }
        other => panic!("Expected InvalidMedia, got {:?}", other),
    }
}
