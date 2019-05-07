use super::super::schema::employee;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::super::handlers::db::{CreateEmployee};
use bigdecimal::{BigDecimal};

#[table_name="employee"]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Employee {
    pub id: String,
    pub business_id: String,
    pub first_name: String,
    pub last_name: String,
    pub pay_rate: BigDecimal,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}


pub fn deserialize_all(msg: &CreateEmployee) -> Vec<Employee> {
    let data = &msg.data.clone();
    data.into_iter()
        .map(|json| serde_json::from_str::<Employee>(&json.to_string()))
        .filter(|model| model.is_ok())
        .map(|model| model.unwrap())
        .collect::<Vec<Employee>>()
}
