use chrono::{DateTime, Utc};
use serde::Serialize;

/// Convert a Things 3 REAL timestamp to `DateTime<Utc>`.
///
/// Things stores `creationDate`, `userModificationDate`, and `stopDate`
/// as REAL columns containing Unix timestamps (seconds since 1970-01-01).
#[allow(clippy::cast_possible_truncation)]
pub fn real_ts_to_datetime(ts: Option<f64>) -> Option<DateTime<Utc>> {
    ts.and_then(|t| DateTime::from_timestamp(t as i64, 0))
}

pub fn real_ts_to_date_string(ts: Option<f64>) -> Option<String> {
    real_ts_to_datetime(ts).map(|dt| dt.format("%Y-%m-%d").to_string())
}

/// Decode a Things 3 "Things date" INTEGER into an ISO date string.
///
/// `startDate` and `deadline` are stored as bitpacked integers:
///   `YYYYYYYYYYYMMMMDDDDD0000000` in binary.
/// Year occupies bits 16+, month bits 12-15, day bits 7-11.
pub fn thingsdate_to_date_string(val: Option<i32>) -> Option<String> {
    val.and_then(|v| {
        let year = (v >> 16) & 0x7FF;
        let month = (v >> 12) & 0xF;
        let day = (v >> 7) & 0x1F;
        if year > 0 && month > 0 && day > 0 {
            Some(format!("{year}-{month:02}-{day:02}"))
        } else {
            None
        }
    })
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
