pub mod db;
use failure::{Error, ResultExt};
use super::handlers::db::{DbExecutor,GetFcpReport, GetLcpReport, GetEgsReport, MaterializeViews};
use std::thread;
use std::sync::mpsc::{Sender,channel, Receiver};
use actix::prelude::*;
use actix_web::{HttpRequest,  HttpResponse};
use super::app::{AppState};
use super::handlers::db::{CreateBusiness, CreateCheck, CreateEmployee, CreateLaborEntry, CreateOrderedItem};
use super::models::{ResponseData, FcpResponse, Interval, LcpResponse, EgsResponse};
use futures::future::{Future, result};
use actix_web::AsyncResponder;

const TABLE_COUNT: usize = 5;

#[derive(Debug)]
pub enum Paths {
    Business,
    Check,
    Employee,
    LaborEntry,
    OrderedItem,
}


fn get(url: &str) -> Result<ResponseData, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header("Authorization", dotenv::var( "AUTH_TOKEN").unwrap())
            .send()
            .context("error during get request");
        match response {
         Ok(mut res) =>  Ok(res.json::<ResponseData>()?),
         Err(err) => Err(format_err!("{}",err.to_string())),
        }
}

/*
 *Given an api enpoint import all records from the api. To avoid potential issues request
 * records in batches of 500. After all records have been imported notify the listening materialize 
 * handler.
 */
fn start_import_queue (db_actor: &Addr<DbExecutor>, path: Paths,tx: &Sender<i32>) ->  thread::JoinHandle<()> {
      let path_str = match &path {
                                    Paths::Business => "businesses",
                                    Paths::Check => "checks",
                                    Paths::Employee => "employees",
                                    Paths::LaborEntry => "laborEntries",
                                    Paths::OrderedItem => "orderedItems"
                                };
      let actor = db_actor.clone();
      let limit =  500;
      let mut count = 0;
      let tx_chan = tx.clone();
      thread::spawn(move || loop {
        let url = format!("{}/{}?limit={}&offset={}", dotenv::var( "BASE_URL").unwrap(),path_str,limit, count);
        match  get(&url) {
         Ok(result) => {
             //extra if instead of guard is compensating for known issue highlighted in
             //this rfc https://github.com/rust-lang/rfcs/blob/master/text/0107-pattern-guards-with-bind-by-move.md

             println!("importing from {:?} {:?} of {:?}",path_str, count, result.count);
             if result.count > count {
                let data = result.data.clone();
                count = (data.into_iter().count() as i32) + count;

                match path {
                    Paths::Business => actor.do_send(CreateBusiness {data: result.data}),
                    Paths::Check => actor.do_send(CreateCheck {data: result.data}),
                    Paths::Employee => actor.do_send(CreateEmployee {data: result.data}),
                    Paths::LaborEntry => actor.do_send(CreateLaborEntry {data: result.data}),
                    Paths::OrderedItem => actor.do_send(CreateOrderedItem {data: result.data}),
                };
             } else {
                 tx_chan.send(1).unwrap();
                 break;
             }},
         Err(result)         => {
             println!("{:?}", result);
             break
         },
        }})
}

/*
 * Wait for all imports to complete then materialize views so that aggregations and date/time
 * calculation does not need to be re-evaluated on each api request.
 */
pub fn materialize_queue(db_actor: &Addr<DbExecutor>,  rx: Receiver<i32>) -> thread::JoinHandle<()> {
    let mut completed_queues = 0;
    let actor = db_actor.clone();
    thread::spawn(move || loop {
        if rx.recv().unwrap() == 1 {
            completed_queues += 1;
        }

        if completed_queues == TABLE_COUNT {
            println!("materializing views");
            actor.do_send(MaterializeViews{});
            println!("import complete");
            break;
        }
    })
}

/*
 *Start a seperate worker thread to import records from each endpoint, then materialize the views.
 */
pub fn import_records(req: &HttpRequest<AppState>) -> &'static str {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let mut queues = Vec::new();
    queues.push( start_import_queue(&req.state().db, Paths::Business,&tx));
    queues.push(start_import_queue(&req.state().db, Paths::Employee, &tx));
    queues.push(start_import_queue(&req.state().db, Paths::Check, &tx));
    queues.push(start_import_queue(&req.state().db, Paths::LaborEntry, &tx));
    queues.push(start_import_queue(&req.state().db, Paths::OrderedItem, &tx));
    queues.push(materialize_queue(&req.state().db, rx));

    for queue in queues {
        queue.join().expect("worker thread panicked");
    }

    "Started import Process"
}

/*
 * Main handler for the api. Evaluate query params sent over http and send them
 * to the applicable handler. Each handler returns a future in order to avoid blocking
 * the main thread.
 */
pub fn reports(req: &HttpRequest<AppState>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    let query = req.query();
    let interval = match query.get("timeInterval").map(|x| x.as_ref()).unwrap_or("hour") {
                    "hour" =>  Interval::Hour,
                    "day"  =>  Interval::Day,
                    "week" =>  Interval::Week,
                    "month" => Interval::Month,
                    _     => Interval::Hour
    };
    //avoid ownership errors when used inside future
    let interval2 = interval.clone();
    //The actix query function returns borrowed values wrapped in an option, map is used
    //to take ownership of the value without unwrapping.
    let limit = query.get("limit").map(|x| x.parse::<usize>().unwrap_or(100));
    let business_id =  query.get("business_id").map(|x| x.to_owned());
    let start =  query.get("start").map(|x| x.to_owned());
    let end =  query.get("end").map(|x| x.to_owned());
    match query["report"].as_ref() {
        "LCP" => req.state().db.send(GetLcpReport {interval: interval,
                                                   limit: limit,
                                                   business_id: business_id,
                                                   start: start,
                                                   end: end})
                                .from_err()
                                .and_then(move |res| {
                                    match res {
                                            Ok(reports) => Ok(HttpResponse::Ok().json(LcpResponse::new(reports, interval2))),
                                            Err(_) => Ok(HttpResponse::InternalServerError().into())
                                        }
                                }).responder(),
        "FCP" => req.state().db.send(GetFcpReport {interval: interval,
                                                   limit: limit,
                                                   business_id: business_id,
                                                   start: start,
                                                   end: end})
                                .from_err()
                                .and_then(move |res| {
                                    match res {
                                            Ok(reports) => Ok(HttpResponse::Ok().json(FcpResponse::new(reports, interval2))),
                                            Err(_) => Ok(HttpResponse::InternalServerError().into())
                                        }
                                }).responder(),
        "EGS" => req.state().db.send(GetEgsReport {interval: interval,
                                                   limit: limit,
                                                   business_id: business_id,
                                                   start: start,
                                                   end: end})
                                .from_err()
                                .and_then(move |res| {
                                    match res {
                                            Ok(reports) => Ok(HttpResponse::Ok().json(EgsResponse::new(reports, interval2))),
                                            Err(_) => Ok(HttpResponse::InternalServerError().into())
                                        }
                                }).responder(),
     _      =>       result(Ok(HttpResponse::InternalServerError()
                                             .into()))
                    .responder()

    }
}
