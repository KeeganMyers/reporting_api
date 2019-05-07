use actix::prelude::*;
use diesel::{
    RunQueryDsl,
    PgConnection,
};
use serde_json::{Value};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error as DbError;
use diesel::dsl::sql_query;
use super::super::models::{Interval};
use super::super::models::lcp_per_hour::{Lcp};
use super::super::models::fcp_per_hour::{Fcp};
use super::super::models::egs_per_hour::{Egs};
use super::super::models::business::{Business, deserialize_all as deserialize_business};
use super::super::models::check_tbl::{Check, deserialize_all as deserialize_check};
use super::super::models::employee::{Employee, deserialize_all as deserialize_employee};
use super::super::models::labor_entry::{LaborEntry, deserialize_all as deserialize_labor_entries};
use super::super::models::ordered_item::{OrderedItem, deserialize_all as deserialize_ordered_items};

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

pub struct MaterializeViews {}
pub struct CreateBusiness {
    pub data: Vec<Value>,
}

pub struct GetLcpReport {
    pub interval: Interval,
    pub limit: Option<usize>,
    pub business_id: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

pub struct GetFcpReport {
    pub interval: Interval,
    pub limit: Option<usize>,
    pub business_id: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

pub struct GetEgsReport {
    pub interval: Interval,
    pub limit: Option<usize>,
    pub business_id: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

pub struct CreateEmployee {
    pub data: Vec<Value>,
}


pub struct CreateCheck {
    pub data: Vec<Value>,
}

pub struct CreateLaborEntry {
    pub data: Vec<Value>,
}

pub struct CreateOrderedItem {
    pub data: Vec<Value>,
}

impl Message for MaterializeViews {
    type Result = ();
}

impl Message for CreateBusiness {
    type Result = Result<Vec<Business>, DbError>;
}

impl Message for GetLcpReport {
    type Result = Result<Vec<Lcp>, DbError>;
}


impl Message for GetFcpReport {
    type Result = Result<Vec<Fcp>, DbError>;
}


impl Message for GetEgsReport {
    type Result = Result<Vec<Egs>, DbError>;
}

impl Message for CreateCheck {
    type Result = Result<Vec<Check>, DbError>;
}


impl Message for CreateEmployee{
    type Result = Result<Vec<Employee>, DbError>;
}

impl Message for CreateLaborEntry {
    type Result = Result<Vec<LaborEntry>, DbError>;
}

impl Message for CreateOrderedItem {
    type Result = Result<Vec<OrderedItem>, DbError>;
}

/*
 * Refresh all materialized views  since the tables will be created when migrate is called. Should
 * populate the required view tables with necessary data.
 */
impl Handler<MaterializeViews> for DbExecutor {
    type Result = ();

    fn handle(&mut self, _msg: MaterializeViews, _: &mut Self::Context) -> Self::Result
    {
        let conn: &PgConnection = &self.0.get().unwrap();
        let mut result_vec = Vec::new();
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW check_view").execute(conn));

        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW lcp_by_hour_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW lcp_by_day_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW lcp_by_week_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW lcp_by_month_view").execute(conn));


        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW fcp_by_hour_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW fcp_by_day_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW fcp_by_week_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW fcp_by_month_view").execute(conn));


        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW egs_by_hour_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW egs_by_day_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW egs_by_week_view").execute(conn));
        result_vec.push(sql_query("REFRESH MATERIALIZED VIEW egs_by_month_view").execute(conn));
        println!("{:?}", result_vec.iter()
                  .filter(|x| x.is_err())
                  .map(|x| match x {
                          Err(e) => e.to_string(),
                          _      => "".to_string(),
                         })
                  .collect::<Vec<String>>());
        }
    }

impl Handler<CreateBusiness> for DbExecutor {
    type Result = Result<Vec<Business>, DbError>;

    fn handle(&mut self, msg: CreateBusiness, _: &mut Self::Context) -> Self::Result
    {
        use crate::schema::business::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();
        let results = diesel::insert_into(business)
            .values(&deserialize_business(&msg))
            .get_results(conn);
        match results {
            Ok(results) => Ok(results),
            Err(err) => {
                   println!("{:?}", err);
                   Err(err)
            }
        }
    }
}

impl Handler<CreateCheck> for DbExecutor {
    type Result = Result<Vec<Check>, DbError>;

    fn handle(&mut self, msg: CreateCheck, _: &mut Self::Context) -> Self::Result
    {
        use crate::schema::check_tbl::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();
        let results = diesel::insert_into(check_tbl)
            .values(&deserialize_check(&msg))
            .get_results(conn);
        match results {
            Ok(results) => Ok(results),
            Err(err) => {
                   println!("{:?}", err);
                   Err(err)
            }
        }
    }
}

impl Handler<CreateEmployee> for DbExecutor {
    type Result = Result<Vec<Employee>, DbError>;

    fn handle(&mut self, msg: CreateEmployee, _: &mut Self::Context) -> Self::Result
    {
        use crate::schema::employee::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();
        let results = diesel::insert_into(employee)
            .values(&deserialize_employee(&msg))
            .get_results(conn);

        match results {
            Ok(results) => Ok(results),
            Err(err) => {
                   println!("{:?}", err);
                   Err(err)
            }
        }
    }
}

impl Handler<CreateLaborEntry> for DbExecutor {
    type Result = Result<Vec<LaborEntry>, DbError>;

    fn handle(&mut self, msg: CreateLaborEntry, _: &mut Self::Context) -> Self::Result
    {
        use crate::schema::labor_entry::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();
        let results = diesel::insert_into(labor_entry)
            .values(&deserialize_labor_entries(&msg))
            .get_results(conn);

        match results {
            Ok(results) => Ok(results),
            Err(err) => {
                   println!("{:?}", err);
                   Err(err)
            }
        }
    }
}

impl Handler<CreateOrderedItem> for DbExecutor {
    type Result = Result<Vec<OrderedItem>, DbError>;

    fn handle(&mut self, msg: CreateOrderedItem, _: &mut Self::Context) -> Self::Result
    {
        use crate::schema::ordered_item::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();
        let results = diesel::insert_into(ordered_item)
            .values(&deserialize_ordered_items(&msg))
            .get_results(conn);

        match results {
            Ok(results) => Ok(results),
            Err(err) => {
                   println!("{:?}", err);
                   Err(err)
            }
        }
    }
}

/*
 * Diesel documentation is limited in regards to converting a custom struct to a query, it seems
 * the ability to create queries from data structures is still limited. As a result given the scope
 * of this project string concatination is being used, but this approach should be avoided in a
 * production application.
 */
impl Handler<GetLcpReport> for DbExecutor {
    type Result = Result<Vec<Lcp>, DbError>;
    fn handle(&mut self, msg: GetLcpReport, _: &mut Self::Context) -> Self::Result
    {
        let mut query_str = String::from("SELECT * FROM ");
        let table_name = match &msg.interval {
                        Interval::Hour =>  "lcp_by_hour_view",
                        Interval::Day =>  "lcp_by_day_view",
                        Interval::Week =>  "lcp_by_week_view",
                        Interval::Month =>  "lcp_by_month_view",
                        };
            query_str.push_str(&table_name.to_string());
            query_str.push_str(" where value is not null ");

        if msg.business_id.is_some() {
            query_str.push_str(" and business_id = '");
            query_str.push_str(&msg.business_id.unwrap().to_string());
            query_str.push_str("'");
        }

        if msg.start.is_some() {
            query_str.push_str(" and start_time >= '");
            query_str.push_str(&msg.start.unwrap().to_string());
            query_str.push_str("'");
        }

        if msg.end.is_some() {
            query_str.push_str(" and end_time <= '");
            query_str.push_str(&msg.end.unwrap().to_string());
            query_str.push_str("'");
        }

        query_str.push_str("order by start_time asc");

        if msg.limit.is_some() {
            query_str.push_str(" limit ");
            query_str.push_str(&msg.limit.unwrap().to_string());
        }
        query_str.push_str(";");
        let conn: &PgConnection = &self.0.get().unwrap();
        sql_query(query_str).load(conn)
        }
    }


impl Handler<GetEgsReport> for DbExecutor {
    type Result = Result<Vec<Egs>, DbError>;

    fn handle(&mut self, msg: GetEgsReport, _: &mut Self::Context) -> Self::Result
    {
        let mut query_str = String::from("SELECT * FROM ");
        let table_name = match &msg.interval {
                        Interval::Hour =>  "egs_by_hour_view",
                        Interval::Day =>  "egs_by_day_view",
                        Interval::Week =>  "egs_by_week_view",
                        Interval::Month =>  "egs_by_month_view",
                        };
            query_str.push_str(&table_name.to_string());
            query_str.push_str(" where value is not null ");

        if msg.business_id.is_some() {
            query_str.push_str(" and business_id = '");
            query_str.push_str(&msg.business_id.unwrap().to_string());
            query_str.push_str("'");
        }

        if msg.start.is_some() {
            query_str.push_str(" and start_time >= '");
            query_str.push_str(&msg.start.unwrap().to_string());
            query_str.push_str("'");
        }

        if msg.end.is_some() {
            query_str.push_str(" and end_time <= '");
            query_str.push_str(&msg.end.unwrap().to_string());
            query_str.push_str("'");
        }

        query_str.push_str("order by start_time asc");

        if msg.limit.is_some() {
            query_str.push_str(" limit ");
            query_str.push_str(&msg.limit.unwrap().to_string());
        }
        query_str.push_str(";");
        let conn: &PgConnection = &self.0.get().unwrap();
        sql_query(query_str).load(conn)
        }
    }


impl Handler<GetFcpReport> for DbExecutor {
    type Result = Result<Vec<Fcp>, DbError>;

    fn handle(&mut self, msg: GetFcpReport, _: &mut Self::Context) -> Self::Result
    {
        let mut query_str = String::from("SELECT * FROM ");
        let table_name = match &msg.interval {
                        Interval::Hour =>  "fcp_by_hour_view",
                        Interval::Day =>  "fcp_by_day_view",
                        Interval::Week =>  "fcp_by_week_view",
                        Interval::Month =>  "fcp_by_month_view",
                        };
            query_str.push_str(&table_name.to_string());
            query_str.push_str(" where value is not null ");

        if msg.business_id.is_some() {
            query_str.push_str(" and business_id = '");
            query_str.push_str(&msg.business_id.unwrap().to_string());
            query_str.push_str("'");
        }

        if msg.start.is_some() {
            query_str.push_str(" and start_time >= '");
            query_str.push_str(&msg.start.unwrap().to_string());
            query_str.push_str("'");
        }

        if msg.end.is_some() {
            query_str.push_str(" and end_time <= '");
            query_str.push_str(&msg.end.unwrap().to_string());
            query_str.push_str("'");
        }

        query_str.push_str("order by start_time asc");

        if msg.limit.is_some() {
            query_str.push_str(" limit ");
            query_str.push_str(&msg.limit.unwrap().to_string());
        }
        query_str.push_str(";");
        let conn: &PgConnection = &self.0.get().unwrap();
        sql_query(query_str).load(conn)
        }
    }

/*
 * Actor is used to store the db connection pool so that it can be used by all db handlers.
 */
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
