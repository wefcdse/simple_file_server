use std::{fs::Metadata, time::SystemTime};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use stupid_utils::predule::*;

use tokio::fs;

use crate::sending_structs::{EntryMetadata, FileBasicInfo, FileInfo, FileMetadata, PathEntryType};

pub async fn serve_dir_info(
    State(root): State<String>,
    Path(path): Path<String>,
) -> Result<Json<FileInfo>, StatusCode> {
    let file_path = format!("{root}/{path}");
    let r: Result<_, StatusCodeErr> = async {
        let metadata = fs::metadata(&file_path).await?;
        match &metadata {
            m if m.is_file() => {
                return Ok(Json(FileInfo {
                    file_type: PathEntryType::File,
                    entries: None,
                    metadata: from_metadata_to_entry_metadata(&metadata),
                    file_metadata: from_file_path_to_file_metadata(&file_path),
                }));
            }
            m if m.is_dir() => {}
            _ => {
                return Err(StatusCodeErr {
                    s: StatusCode::NOT_FOUND,
                });
            }
        };
        debug_assert!(metadata.is_dir());

        let mut read_dir = fs::read_dir(&file_path).await?;
        let mut entries = Vec::new();
        while let Some(entry) = read_dir.next_entry().await? {
            let file_name = entry.file_name().to_str().to_result()?.to_owned();
            let file_type = entry
                .file_type()
                .await
                .map(|f| match f {
                    v if v.is_file() => Some(PathEntryType::File),
                    v if v.is_dir() => Some(PathEntryType::Dir),
                    _ => None,
                })?
                .to_result()?;
            entries.push(FileBasicInfo {
                name: file_name,
                file_type,
                metadata: from_metadata_to_entry_metadata(&entry.metadata().await?),
                file_metadata: from_file_path_to_file_metadata(&entry.path()),
            })
        }
        Ok(Json(FileInfo {
            file_type: PathEntryType::Dir,
            entries: Some(entries),
            metadata: from_metadata_to_entry_metadata(&metadata),
            file_metadata: None,
        }))
    }
    .await;
    Ok(r?)
}

struct StatusCodeErr {
    s: StatusCode,
}

impl<E: std::error::Error> From<E> for StatusCodeErr {
    fn from(_value: E) -> Self {
        return Self {
            s: StatusCode::NOT_FOUND,
        };
    }
}

impl From<StatusCodeErr> for StatusCode {
    fn from(value: StatusCodeErr) -> Self {
        value.s
    }
}

pub fn from_metadata_to_entry_metadata(metadata: &Metadata) -> EntryMetadata {
    let err_time = SystemTime::UNIX_EPOCH;
    let created_time = metadata.created().unwrap_or(err_time);
    let accessed_time = metadata.accessed().unwrap_or(err_time);
    let modified_time = metadata.modified().unwrap_or(err_time);
    let size = metadata.len();
    EntryMetadata {
        created_time,
        accessed_time,
        modified_time,
        size,
    }
}
pub fn from_file_path_to_file_metadata<P: AsRef<std::path::Path>>(path: P) -> Option<FileMetadata> {
    let path = path.as_ref();
    let file_name = path.file_name()?;
    let file_name = format!("{:?}", file_name);
    let file_name = file_name[1..file_name.len() - 1].to_owned();
    let extend_name = path.extension().map(|extend_name| {
        let extend_name = format!("{:?}", extend_name);
        let extend_name = extend_name[1..extend_name.len() - 1].to_owned();
        extend_name
    });
    let has_extend_name = extend_name.is_some();
    Some(FileMetadata {
        file_name,
        extend_name,
        has_extend_name,
    })
}

#[test]
fn trying() {
    let t = SystemTime::UNIX_EPOCH;
    dbg!(t);
}
