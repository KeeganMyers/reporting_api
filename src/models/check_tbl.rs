use super::super::schema::check_tbl;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::super::handlers::db::{CreateCheck};

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="check_tbl"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Check {
    pub id: String,
    pub business_id: String,
    pub employee_id: String,
    pub name: String,
    pub closed: bool,
    pub closed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub fn deserialize_all(msg: &CreateCheck) -> Vec<Check> {
    let data = &msg.data.clone();
    data.into_iter()
        .map(|json| serde_json::from_str::<Check>(&json.to_string()))
        .filter(|model| model.is_ok())
        .map(|model| model.unwrap())
        .collect::<Vec<Check>>()
}
