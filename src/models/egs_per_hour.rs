use serde::{Serialize, Deserialize};
use bigdecimal::{BigDecimal};
use serde_json::{Value};
use super::{Model};
use diesel::pg::types::sql_types::{Jsonb};
use diesel::sql_types::{Numeric,Text};

#[derive(Debug, Serialize, Deserialize, Clone,Queryable, QueryableByName)]
pub struct Egs {
    #[sql_type = "Text"]
    pub employee: String,
    #[sql_type = "Numeric"]
    pub value: BigDecimal,
    #[sql_type = "Jsonb"]
    pub time_frame: Value
}

impl Model for Egs {}
