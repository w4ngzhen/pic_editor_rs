use crate::cli::AppArgs;
use crate::utils::{rand_u64, random_string};
use bevy::prelude::{Component, Resource};
use std::path::PathBuf;
use time::OffsetDateTime;

#[derive(Resource)]
pub struct Workspace {
    id: String,
    default_doc_id: DocId,
    doc_list: Vec<Doc>,
}

impl Workspace {
    pub fn get_workspace(file_path: &str) -> Workspace {
        let default_doc = Doc::new(file_path);
        let doc_id = default_doc.id;
        let doc_list = vec![default_doc];
        Workspace {
            id: random_string(8),
            default_doc_id: doc_id,
            doc_list,
        }
    }

    pub fn get_default_doc(&self) -> &Doc {
        let id = self.default_doc_id;
        self.doc_list.iter().find(|item| item.id == id).unwrap()
    }

    #[allow(unused)]
    pub fn get_doc(&self, id: &DocId) -> Option<&Doc> {
        self.doc_list.iter().find(|item| item.id == *id)
    }

    pub fn from_args(args: &AppArgs) -> Self {
        let mut file_path_buf: PathBuf;
        if let Some(file_path) = &args.file_path {
            file_path_buf = PathBuf::from(file_path);
            if !file_path_buf.exists() {
                file_path_buf = PathBuf::from("/Users/w4ngzhen/Pictures/demo.png");
            }
        } else {
            file_path_buf = PathBuf::from("/Users/w4ngzhen/Pictures/demo.png");
        }
        Self::get_workspace(file_path_buf.to_str().unwrap())
    }
}

pub struct Doc {
    /// document id
    pub id: DocId,
    /// document name
    pub name: String,
    /// original file path
    pub src_path: String,
    /// created time, ms since 19700-01-01 00:00:00
    pub created_time: i128,
    /// modified time, ms since 19700-01-01 00:00:00
    pub modified_time: Option<i128>,
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct DocId(pub u64);

impl Doc {
    pub fn new(file_path: &str) -> Self {
        let now = OffsetDateTime::now_utc();
        // 将当前时间转换为自 UNIX 纪元以来的毫秒数
        let ms = now.unix_timestamp_nanos() / 1_000_000;
        Self {
            id: DocId(rand_u64()),
            name: "Unnamed".into(),
            src_path: file_path.to_string(),
            created_time: ms,
            modified_time: None,
        }
    }
}
