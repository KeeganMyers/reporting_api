use serde_json::{Value};
use bigdecimal::{BigDecimal};
use serde::{Serialize, Deserialize};
use super::{Model};
use diesel::pg::types::sql_types::{Jsonb};
use diesel::sql_types::{Numeric};

#[derive(Debug, Serialize, Deserialize, Clone,Queryable, QueryableByName)]
pub struct Lcp {
    #[sql_type = "Numeric"]
    pub value: BigDecimal,
    #[sql_type = "Jsonb"]
    pub time_frame: Value,
}

impl Model for Lcp {}
