use chrono::{DateTime, Utc};
use serde::Serialize;

/// Seconds between Unix epoch (1970-01-01) and Apple Core Data epoch (2001-01-01).
pub const APPLE_EPOCH_OFFSET: i64 = 978_307_200;

/// Convert an Apple Core Data timestamp (f64 seconds since 2001-01-01) to a `DateTime<Utc>`.
///
/// Core Data stores timestamps as REAL (f64). Actual values are ~10^9 (well
/// within f64's exact integer range of 2^53), so the f64→i64 truncation is
/// lossless for any real-world timestamp.
#[allow(clippy::cast_possible_truncation)]
pub fn apple_to_datetime(ts: Option<f64>) -> Option<DateTime<Utc>> {
    ts.and_then(|t| DateTime::from_timestamp(t as i64 + APPLE_EPOCH_OFFSET, 0))
}

pub fn apple_to_date_string(ts: Option<f64>) -> Option<String> {
    apple_to_datetime(ts).map(|dt| dt.format("%Y-%m-%d").to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    Task,
    Project,
    Heading,
}

impl TaskType {
    pub const fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::Project,
            2 => Self::Heading,
            _ => Self::Task,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Open,
    Completed,
    Canceled,
}

impl TaskStatus {
    pub const fn from_i32(v: i32) -> Self {
        match v {
            2 => Self::Completed,
            3 => Self::Canceled,
            _ => Self::Open,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StartBucket {
    Inbox,
    Started,
    Someday,
}

impl StartBucket {
    pub const fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::Started,
            2 => Self::Someday,
            _ => Self::Inbox,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Task {
    pub uuid: String,
    pub title: String,
    #[serde(rename = "type")]
    pub kind: TaskType,
    pub status: TaskStatus,
    pub start: StartBucket,
    pub notes: Option<String>,
    pub project_uuid: Option<String>,
    pub project_title: Option<String>,
    pub area_uuid: Option<String>,
    pub area_title: Option<String>,
    pub tags: Vec<String>,
    pub checklist_count: i32,
    pub checklist_done: i32,
    pub created_date: Option<String>,
    pub modified_date: Option<String>,
    pub start_date: Option<String>,
    pub deadline: Option<String>,
    pub completion_date: Option<String>,
    pub index: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub uuid: String,
    pub title: String,
    pub status: TaskStatus,
    pub notes: Option<String>,
    pub area_uuid: Option<String>,
    pub area_title: Option<String>,
    pub tags: Vec<String>,
    pub task_count: i32,
    pub completed_count: i32,
    pub deadline: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Area {
    pub uuid: String,
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub uuid: String,
    pub title: String,
}
