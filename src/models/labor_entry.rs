use super::super::schema::labor_entry;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::super::handlers::db::{CreateLaborEntry};
use bigdecimal::{BigDecimal};

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="labor_entry"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborEntry {
    pub id: String,
    pub business_id: String,
    pub employee_id: String,
    pub name: String,
    pub clock_in: DateTime<Utc>,
    pub clock_out: DateTime<Utc>,
    pub pay_rate: BigDecimal,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}


pub fn deserialize_all(msg: &CreateLaborEntry) -> Vec<LaborEntry> {
    let data = &msg.data.clone();
    data.into_iter()
        .map(|json| serde_json::from_str::<LaborEntry>(&json.to_string()))
        .filter(|model| model.is_ok())
        .map(|model| model.unwrap())
        .collect::<Vec<LaborEntry>>()
}
