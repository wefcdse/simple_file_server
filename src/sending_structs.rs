use std::time::SystemTime;

use serde::ser::SerializeStruct;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileInfo {
    #[serde(flatten)]
    pub file_type: PathEntryType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<FileBasicInfo>>,
    #[serde(flatten)]
    pub metadata: EntryMetadata,
    #[serde(flatten)]
    pub file_metadata: Option<FileMetadata>,
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct FileBasicInfo {
    pub name: String,
    #[serde(flatten)]
    pub file_type: PathEntryType,
    #[serde(flatten)]
    pub metadata: EntryMetadata,
    #[serde(flatten)]
    pub file_metadata: Option<FileMetadata>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct EntryMetadata {
    pub created_time: SystemTime,
    pub accessed_time: SystemTime,
    pub modified_time: SystemTime,
    pub size: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileMetadata {
    // pub size: usize,
    pub file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_name: Option<String>,
    pub has_extend_name: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum PathEntryType {
    File,
    Dir,
}
impl serde::Serialize for PathEntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (type_name, is_dir, is_file) = match self {
            PathEntryType::File => ("file", false, true),
            PathEntryType::Dir => ("dir", true, false),
        };
        let mut file_type = serializer.serialize_struct("FileType", 3)?;
        file_type.serialize_field("file_type", type_name)?;
        file_type.serialize_field("is_dir", &is_dir)?;
        file_type.serialize_field("is_file", &is_file)?;
        file_type.end()
    }
}
