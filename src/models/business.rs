use super::super::schema::business;
use serde::{Serialize, Deserialize};
use super::{Model};
use super::super::handlers::db::{CreateBusiness};

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="business"]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Business {
    pub id: String,
    pub name: String,
}
impl Model for Business {}

pub fn deserialize_all(msg: &CreateBusiness) -> Vec<Business> {
    let data = &msg.data.clone();
    data.into_iter()
        .map(|json| serde_json::from_str::<Business>(&json.to_string()))
        .filter(|model| model.is_ok())
        .map(|model| model.unwrap())
        .collect::<Vec<Business>>()
}
