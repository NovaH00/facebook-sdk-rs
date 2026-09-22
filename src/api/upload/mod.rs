//! Resumable file upload for the Meta Graph API.
//!
//! [`UploadApi`] implements Meta's Resumable Upload API protocol.
//! It supports uploading large files (such as videos, images, and PDFs)
//! either in a single request or in sequential chunks with resume capability.
//!
//! # Main Types
//!
//! - [`UploadApi`] — The API client for starting sessions and uploading chunks
//! - [`UploadFileType`] — Strongly typed enum for supported upload MIME types
//! - [`UploadSession`] — An initiated upload session holding a session ID
//! - [`UploadSessionStatus`] — Session status with the current byte offset
//! - [`UploadChunkResponse`] — Response returned after uploading a chunk

mod api;
mod models;

pub use api::UploadApi;
pub use models::{
    UploadFileType,
    UploadSession,
    UploadSessionStatus,
    UploadChunkResponse,
    deserialize_offset,
    deserialize_optional_offset,
};
