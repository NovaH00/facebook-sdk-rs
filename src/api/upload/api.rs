use std::path::Path;
use crate::graph::{
    UserGraphClient,
    GraphError,
    Method,
    QueryParams,
};
use super::models::{
    UploadFileType,
    UploadSession,
    UploadSessionStatus,
    UploadChunkResponse,
};

/// High-level API for Meta's Resumable Upload API.
///
/// Use `UploadApi` to upload large files (videos, images, PDFs) to Meta servers.
/// Upon completion, Meta returns a file handle `h`.
/// You can pass this handle to other APIs to publish content.
///
/// # Example
///
/// ```rust,no_run
/// # async fn _test() {
/// use facebook_sdk_rs::api::upload::{UploadApi, UploadFileType};
/// use facebook_sdk_rs::graph::UserGraphClient;
///
/// # let user_client: UserGraphClient = unimplemented!();
/// let upload_api = UploadApi::new(&user_client, "your-app-id");
///
/// let file_bytes = vec![0u8; 1024];
/// let handle = upload_api
///     .upload("video.mp4", UploadFileType::Mp4, file_bytes)
///     .await
///     .unwrap();
/// println!("Uploaded handle: {}", handle);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct UploadApi {
    user_graph_client: UserGraphClient,
    app_id: String,
}

impl UploadApi {
    /// Creates a new `UploadApi` instance.
    pub fn new(
        user_graph_client: &UserGraphClient,
        app_id: impl Into<String>,
    ) -> Self {
        Self {
            user_graph_client: user_graph_client.clone(),
            app_id: app_id.into(),
        }
    }

    /// Starts a new upload session.
    ///
    /// Calls `POST /{app_id}/uploads`.
    /// Returns an [`UploadSession`] containing the upload session ID.
    pub async fn start_session(
        &self,
        file_name: impl Into<String>,
        file_length: u64,
        file_type: impl Into<UploadFileType>,
    ) -> Result<UploadSession, GraphError> {
        let file_type = file_type.into();
        let query = QueryParams::new()
            .insert("file_name", file_name.into())
            .insert("file_length", file_length.to_string())
            .insert("file_type", file_type.to_string());

        let endpoint = format!("/{}/uploads", self.app_id);
        self.user_graph_client
            .request(Method::POST, endpoint)
            .query_params(query)
            .send::<UploadSession>()
            .await
    }

    /// Uploads a binary chunk to an active upload session.
    ///
    /// Calls `POST /{upload_session_id}` with binary data.
    /// Sets the `file_offset` header to the specified byte offset.
    /// Uses the `Authorization: OAuth <token>` header for authorization.
    pub async fn upload_chunk(
        &self,
        upload_session_id: &str,
        file_offset: u64,
        chunk: Vec<u8>,
    ) -> Result<UploadChunkResponse, GraphError> {
        self.user_graph_client
            .request(Method::POST, upload_session_id)
            .use_oauth_header(true)
            .header("file_offset", file_offset.to_string())
            .body(chunk)
            .send::<UploadChunkResponse>()
            .await
    }

    /// Queries the status of an active or interrupted upload session.
    ///
    /// Calls `GET /{upload_session_id}`.
    /// Returns the current [`UploadSessionStatus`] with the next expected `file_offset`.
    pub async fn get_session_status(
        &self,
        upload_session_id: &str,
    ) -> Result<UploadSessionStatus, GraphError> {
        self.user_graph_client
            .request(Method::GET, upload_session_id)
            .use_oauth_header(true)
            .send::<UploadSessionStatus>()
            .await
    }

    /// Uploads an entire file in a single request.
    ///
    /// Starts an upload session and transmits all bytes starting at offset 0.
    /// Returns the uploaded file handle `h`.
    pub async fn upload(
        &self,
        file_name: impl Into<String>,
        file_type: impl Into<UploadFileType>,
        data: Vec<u8>,
    ) -> Result<String, GraphError> {
        let total_length = data.len() as u64;
        let session = self.start_session(file_name, total_length, file_type).await?;

        let response = self.upload_chunk(&session.id, 0, data).await?;
        response.h.ok_or_else(|| {
            GraphError::UploadError(
                "Upload finished but Meta did not return a file handle".to_string(),
            )
        })
    }

    /// Uploads data in chunks of the specified size.
    ///
    /// Starts a session and uploads sequential chunks.
    /// If an intermediate chunk succeeds, it advances the offset.
    /// Returns the final file handle `h`.
    pub async fn upload_in_chunks(
        &self,
        file_name: impl Into<String>,
        file_type: impl Into<UploadFileType>,
        data: &[u8],
        chunk_size: usize,
    ) -> Result<String, GraphError> {
        if chunk_size == 0 {
            return Err(GraphError::UploadError(
                "chunk_size must be greater than 0".to_string(),
            ));
        }

        let total_length = data.len() as u64;
        let session = self.start_session(file_name, total_length, file_type).await?;

        self.upload_data_loop(&session.id, data, chunk_size, 0).await
    }

    /// Resumes an interrupted upload session with the provided data.
    ///
    /// Queries the current offset from Meta and transmits the remaining chunks.
    pub async fn resume_upload_in_chunks(
        &self,
        upload_session_id: &str,
        data: &[u8],
        chunk_size: usize,
    ) -> Result<String, GraphError> {
        if chunk_size == 0 {
            return Err(GraphError::UploadError(
                "chunk_size must be greater than 0".to_string(),
            ));
        }

        let status = self.get_session_status(upload_session_id).await?;
        self.upload_data_loop(upload_session_id, data, chunk_size, status.file_offset).await
    }

    /// Uploads a file from the local file system.
    ///
    /// Automatically detects the file name and infers the file type if not provided.
    pub async fn upload_file(
        &self,
        file_path: impl AsRef<Path>,
        file_type: Option<UploadFileType>,
    ) -> Result<String, GraphError> {
        let path = file_path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                GraphError::UploadError("Invalid file path name".to_string())
            })?;

        let resolved_type = match file_type {
            Some(t) => t,
            None => {
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or_default();
                UploadFileType::from_extension(ext).ok_or_else(|| {
                    GraphError::UploadError(format!(
                        "Could not infer file type from extension: {}",
                        ext
                    ))
                })?
            }
        };

        let bytes = std::fs::read(path).map_err(|e| {
            GraphError::UploadError(format!("Failed to read file: {}", e))
        })?;

        self.upload(file_name, resolved_type, bytes).await
    }

    async fn upload_data_loop(
        &self,
        session_id: &str,
        data: &[u8],
        chunk_size: usize,
        initial_offset: u64,
    ) -> Result<String, GraphError> {
        let total_length = data.len() as u64;
        let mut offset = initial_offset;

        while offset < total_length {
            let start = offset as usize;
            let end = (start + chunk_size).min(data.len());
            let chunk = data[start..end].to_vec();

            let response = self.upload_chunk(session_id, offset, chunk).await?;
            if let Some(handle) = response.h {
                return Ok(handle);
            }

            if let Some(next_offset) = response.file_offset {
                offset = next_offset;
            } else {
                let status = self.get_session_status(session_id).await?;
                offset = status.file_offset;
            }
        }

        Err(GraphError::UploadError(
            "Upload completed without returning a file handle".to_string(),
        ))
    }
}
