pub mod business;
pub mod lcp_per_hour;
pub mod egs_per_hour;
pub mod fcp_per_hour;
pub mod check_tbl;
pub mod employee;
pub mod labor_entry;
pub mod ordered_item;

use super::models::egs_per_hour::{Egs};
use super::models::lcp_per_hour::{Lcp};
use super::models::fcp_per_hour::{Fcp};
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    pub count: i32,
    pub data: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReportType {
    LCP,
    EGS,
    FCP
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Interval {
    Hour,
    Day,
    Week,
    Month
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LcpResponse {
    pub report: &'static str,
    pub time_interval: &'static str,
    pub data: Vec<Lcp>,
}

fn interval_to_str (interval: Interval) -> &'static str {
    match interval {
        Interval::Hour => "hour",
        Interval::Day => "day",
        Interval::Week => "week",
        Interval::Month => "month",
    }
}

impl LcpResponse {
    pub fn new(data: Vec<Lcp>, interval: Interval) -> Self {
        LcpResponse {
            report: "LCP",
            time_interval: interval_to_str(interval),
            data: data
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct FcpResponse {
    pub report: &'static str,
    pub time_interval: &'static str,
    pub data: Vec<Fcp>,
}


impl FcpResponse {
    pub fn new(data: Vec<Fcp>, interval: Interval) -> Self {
        FcpResponse {
            report: "FCP",
            time_interval: interval_to_str(interval),
            data: data
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct EgsResponse {
    pub report: &'static str,
    pub time_interval: &'static str,
    pub data: Vec<Egs>,
}


impl EgsResponse {
    pub fn new(data: Vec<Egs>, interval: Interval) -> Self {
        EgsResponse {
            report: "EGS",
            time_interval: interval_to_str(interval),
            data: data
        }
    }
}

pub trait Model {}
