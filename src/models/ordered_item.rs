use super::super::schema::ordered_item;
use serde::{Serialize, Deserialize};
use super::super::handlers::db::{CreateOrderedItem};
use bigdecimal::{BigDecimal};
use chrono::{DateTime, Utc};

#[table_name="ordered_item"]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct OrderedItem {
    pub id: String,
    pub business_id: String,
    pub employee_id: String,
    pub check_id: String,
    pub item_id: String,
    pub cost:  BigDecimal,
    pub price: BigDecimal,
    pub voided: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub fn deserialize_all(msg: &CreateOrderedItem) -> Vec<OrderedItem> {
    let data = &msg.data.clone();
    data.into_iter()
        .map(|json| serde_json::from_str::<OrderedItem>(&json.to_string()))
        .filter(|model| model.is_ok())
        .map(|model| model.unwrap())
        .collect::<Vec<OrderedItem>>()
}
