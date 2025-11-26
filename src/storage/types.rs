use crate::error::CreateErr;
use crate::storage::init::{FNAME, ROOT_FDNAME};
use crate::storage::vault::is_vault_exisits;
use chrono::{DateTime, Local, Utc};
use rand::random;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Write;
use std::{
    env,
    fs::{self, File, OpenOptions, create_dir_all},
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Vault {
    pub magic: [u8; 4],
    pub version: u16,
    pub salt: [u8; 16],
    pub encrypted_data: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Register {
    pub r_name: String,
    pub metadata: RegMetadata,
    pub entries: Vec<Entry>,
    pub log: Vec<LogEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub entry_id: Uuid,
    pub used_for: Vec<String>,
    pub password: String,
    pub notes: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub metadata: EntryMetadata,
    pub custom_field: Option<HashMap<String, CustomValue>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CustomValue {
    Text(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub operation: Operation,
    pub entry_id: Option<Uuid>,
    pub status: bool,
    pub details: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Operation {
    InsertEntry,
    UpdateEntry,
    FetchEntry,
    DeleteEntry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegMetadata {
    pub created_at: DateTime<Local>,
    pub modified_at: DateTime<Local>,
    pub access_count: u32,
    pub n_of_entries: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryMetadata {
    pub created_at: DateTime<Local>,
    pub modified_at: DateTime<Local>,
    pub fetched_cnt: u32,
    pub password: String,
    pub strength_score: u8,
    pub created_by: CreatedBy,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CreatedBy {
    Manual,
    Generated,
}

impl Register {
    fn new(r_name: &str) -> Self {
        Self {
            r_name: r_name.to_string(),
            metadata: RegMetadata {
                created_at: Local::now(),
                modified_at: Local::now(),
                access_count: 0,
                n_of_entries: 0,
            },
            entries: Vec::new(),
            log: Vec::new(),
        }
    }
}
