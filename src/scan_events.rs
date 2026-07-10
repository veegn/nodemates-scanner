use serde::Serialize;

use crate::models::ScanResult;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanMode {
    Scan,
    Cache,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanStatus {
    Completed,
    Stopped,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ScanEvent {
    Start {
        mode: ScanMode,
        target: String,
        ports: Vec<u16>,
        total: usize,
        completed: usize,
        resumed: bool,
    },
    Progress {
        mode: ScanMode,
        completed: usize,
        total: usize,
        ip: String,
        port: u16,
    },
    Done {
        status: ScanStatus,
        completed: usize,
        total: usize,
    },
    Info {
        message: String,
    },
    Error {
        message: String,
    },
    Result {
        result: Box<ScanResult>,
    },
}

impl ScanEvent {
    pub fn into_text(self) -> String {
        serde_json::to_string(&self).expect("scan event serialization should not fail")
    }
}
