use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header, query_param, body_bytes};
use facebook_sdk_rs::auth::LongLivedUserToken;
use facebook_sdk_rs::graph::GraphClient;
use facebook_sdk_rs::api::upload::{
    UploadApi,
    UploadFileType,
    UploadSessionStatus,
    UploadChunkResponse,
};

#[tokio::test]
async fn test_start_session() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v25.0/test_app_id/uploads"))
        .and(query_param("file_name", "test_video.mp4"))
        .and(query_param("file_length", "2048"))
        .and(query_param("file_type", "video/mp4"))
        .and(query_param("access_token", "fake_user_token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "id": "upload:session_12345"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let token = LongLivedUserToken::new("fake_user_token");
    let client = GraphClient::new(token).with_base_url(mock_server.uri());
    let upload_api = UploadApi::new(&client, "test_app_id");

    let session = upload_api
        .start_session("test_video.mp4", 2048, UploadFileType::Mp4)
        .await
        .unwrap();

    assert_eq!(session.id, "upload:session_12345");
}

#[tokio::test]
async fn test_upload_chunk_final() {
    let mock_server = MockServer::start().await;
    let chunk_data = b"hello binary world".to_vec();

    Mock::given(method("POST"))
        .and(path("/v25.0/upload:session_12345"))
        .and(header("Authorization", "OAuth fake_user_token"))
        .and(header("file_offset", "0"))
        .and(body_bytes(chunk_data.clone()))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "h": "handle_abc123"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let token = LongLivedUserToken::new("fake_user_token");
    let client = GraphClient::new(token).with_base_url(mock_server.uri());
    let upload_api = UploadApi::new(&client, "test_app_id");

    let resp = upload_api
        .upload_chunk("upload:session_12345", 0, chunk_data)
        .await
        .unwrap();

    assert!(resp.is_complete());
    assert_eq!(resp.handle(), Some("handle_abc123"));
}

#[tokio::test]
async fn test_get_session_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v25.0/upload:session_12345"))
        .and(header("Authorization", "OAuth fake_user_token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "id": "upload:session_12345",
                    "file_offset": "1024"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let token = LongLivedUserToken::new("fake_user_token");
    let client = GraphClient::new(token).with_base_url(mock_server.uri());
    let upload_api = UploadApi::new(&client, "test_app_id");

    let status = upload_api
        .get_session_status("upload:session_12345")
        .await
        .unwrap();

    assert_eq!(status.id, "upload:session_12345");
    assert_eq!(status.file_offset, 1024);
}

#[tokio::test]
async fn test_upload_single_shot() {
    let mock_server = MockServer::start().await;
    let data = vec![1, 2, 3, 4, 5];

    // Step 1: start session
    Mock::given(method("POST"))
        .and(path("/v25.0/test_app_id/uploads"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "id": "upload:session_one_shot"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // Step 2: upload chunk
    Mock::given(method("POST"))
        .and(path("/v25.0/upload:session_one_shot"))
        .and(header("file_offset", "0"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "h": "handle_full_upload"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let token = LongLivedUserToken::new("fake_user_token");
    let client = GraphClient::new(token).with_base_url(mock_server.uri());
    let upload_api = UploadApi::new(&client, "test_app_id");

    let handle = upload_api
        .upload("doc.pdf", UploadFileType::Pdf, data)
        .await
        .unwrap();

    assert_eq!(handle, "handle_full_upload");
}

#[tokio::test]
async fn test_upload_in_chunks() {
    let mock_server = MockServer::start().await;
    let data = vec![42u8; 15]; // 15 bytes total, chunk size = 10

    // Step 1: start session
    Mock::given(method("POST"))
        .and(path("/v25.0/test_app_id/uploads"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "id": "upload:session_chunks"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // Chunk 1: bytes 0..10
    Mock::given(method("POST"))
        .and(path("/v25.0/upload:session_chunks"))
        .and(header("file_offset", "0"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "id": "upload:session_chunks",
                    "file_offset": 10
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // Chunk 2: bytes 10..15
    Mock::given(method("POST"))
        .and(path("/v25.0/upload:session_chunks"))
        .and(header("file_offset", "10"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "h": "chunked_final_handle"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let token = LongLivedUserToken::new("fake_user_token");
    let client = GraphClient::new(token).with_base_url(mock_server.uri());
    let upload_api = UploadApi::new(&client, "test_app_id");

    let handle = upload_api
        .upload_in_chunks("video.mp4", UploadFileType::Mp4, &data, 10)
        .await
        .unwrap();

    assert_eq!(handle, "chunked_final_handle");
}

#[tokio::test]
async fn test_resume_upload_in_chunks() {
    let mock_server = MockServer::start().await;
    let data = vec![99u8; 20]; // 20 bytes total

    // GET status -> current offset = 10
    Mock::given(method("GET"))
        .and(path("/v25.0/upload:interrupted_session"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "id": "upload:interrupted_session",
                    "file_offset": "10"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // Resumed chunk: bytes 10..20
    Mock::given(method("POST"))
        .and(path("/v25.0/upload:interrupted_session"))
        .and(header("file_offset", "10"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "h": "resumed_success_handle"
                }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let token = LongLivedUserToken::new("fake_user_token");
    let client = GraphClient::new(token).with_base_url(mock_server.uri());
    let upload_api = UploadApi::new(&client, "test_app_id");

    let handle = upload_api
        .resume_upload_in_chunks("upload:interrupted_session", &data, 10)
        .await
        .unwrap();

    assert_eq!(handle, "resumed_success_handle");
}

#[test]
fn test_models_deserialization() {
    let json_numeric = r#"{"id": "upload:1", "file_offset": 500}"#;
    let status_num: UploadSessionStatus = serde_json::from_str(json_numeric).unwrap();
    assert_eq!(status_num.file_offset, 500);

    let json_string = r#"{"id": "upload:1", "file_offset": "1234"}"#;
    let status_str: UploadSessionStatus = serde_json::from_str(json_string).unwrap();
    assert_eq!(status_str.file_offset, 1234);

    let chunk_complete = r#"{"h": "handle_done"}"#;
    let chunk_resp: UploadChunkResponse = serde_json::from_str(chunk_complete).unwrap();
    assert!(chunk_resp.is_complete());
    assert_eq!(chunk_resp.handle(), Some("handle_done"));

    let chunk_progress = r#"{"id": "upload:1", "file_offset": "2048"}"#;
    let chunk_prog: UploadChunkResponse = serde_json::from_str(chunk_progress).unwrap();
    assert!(!chunk_prog.is_complete());
    assert_eq!(chunk_prog.file_offset, Some(2048));
}

#[test]
fn test_file_type_helpers() {
    assert_eq!(UploadFileType::from_extension("mp4"), Some(UploadFileType::Mp4));
    assert_eq!(UploadFileType::from_extension(".pdf"), Some(UploadFileType::Pdf));
    assert_eq!(UploadFileType::from_extension("jpeg"), Some(UploadFileType::Jpeg));
    assert_eq!(UploadFileType::from_extension("jpg"), Some(UploadFileType::Jpg));
    assert_eq!(UploadFileType::from_extension("png"), Some(UploadFileType::Png));
    assert_eq!(UploadFileType::from_extension("unknown"), None);
    assert_eq!(UploadFileType::Mp4.as_str(), "video/mp4");
}
