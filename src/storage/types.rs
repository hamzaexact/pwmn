use crate::error::CreateErr;
use crate::p_std::uid::Uid;
use crate::storage::init::{PARENT_FD_NAME, PARENT_FL_NAME};
use crate::storage::vault_utl::is_vault_exisits;
use bincode::{Decode, Encode};
use chacha20poly1305::aead::generic_array::typenum::NotEq;
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

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct Register {
    pub r_name: String,
    pub metadata: RegMetadata,
    pub entries: Vec<Entry>,
    pub log: Vec<LogEntry>,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct Entry {
    pub entry_id: String,
    pub used_for: Vec<String>,
    pub password: String,
    pub notes: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub metadata: EntryMetadata,
    pub custom_field: Option<HashMap<String, CustomValue>>,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub enum CustomValue {
    Text(String),
    Number(i32),
    Bool(bool),
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct LogEntry {
    pub timestamp: i64,
    pub operation: Operation,
    pub entry_id: Option<String>,
    pub status: bool,
    pub details: String,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub enum Operation {
    InsertEntry,
    UpdateEntry,
    FetchEntry,
    DeleteEntry,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct RegMetadata {
    pub created_at: i64,
    pub modified_at: i64,
    pub access_count: u32,
    pub n_of_entries: u32,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct EntryMetadata {
    pub created_at: i64,
    pub modified_at: i64,
    pub fetched_cnt: u32,
    pub password: String,
    pub strength_score: u8,
    pub created_by: CreatedBy,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub enum CreatedBy {
    Manual,
    Generated,
}

impl Register {
    pub fn new(r_name: &str) -> Self {
        let now = Local::now().timestamp();
        Self {
            r_name: r_name.to_string(),
            metadata: RegMetadata {
                created_at: now,
                modified_at: now,
                access_count: 0,
                n_of_entries: 0,
            },
            entries: Vec::new(),
            log: Vec::new(),
        }
    }
}
