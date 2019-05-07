# Reporting API
A simple http api that aggregates POS data. This api has been created with
rust and actix which is an actor based framework. Actors in this framework are intended
to have a long lifespan. As such to expedite the import process it also makes use
of short lived threads to request and store data in postgres. Once all records
have been imported a set of materialized views are refreshed. Since
synchronizing with the POS api is outside of the scope of this project all
aggregation and date/time calculations can be front loaded.

---
# Use:
The api has two routes
/import
load all data from the POS api

/reporting
returns json encoded reporting data based on the report requests. It honors the
following query parameters:
- **business_id** (uuid) - The id of the business to run this report for.
- **report** (LCP | FCP | EGS) - The abbreviated name of the report to run.
- **timeInterval** (hour | day | week | month) - The time interval to aggregate the data.
- **start** (date) - The start date used to constrain the results. ISO-8601 date
- **end** (date) - The end date used to constrain the results. ISO-8601 date.
- **limit** (int) - Specify the maximum number of records to return defaults to
100.

---
# Installation

- Install rust
 ``` curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh ```

- Install diesel to interact with the DB
  ``` cargo install diesel_cli ```

- In order to limit dependencies and avoid impacting other databases that may
already be present on the system I have been running postgres inside docker

- Create the host directory
``` mkdir data ```
- Start the container
``` docker run --name postgres -v $(pwd)/data:/var/lib/postgresql/data -p 5532:5432 -d postgres:9.4-alpine ```

- If you would prefer to host postgres elsewhere please change DATABASE_URL in the
.env file

- Setup the DB and run the migrations
``` diesel setup
   diesel migration run ```

- Compile the project
``` cargo build --release ```

- Run the server
``` target/release/reporting_api ```

- Import  Records from the target POS system
 ``` curl -X POST 127.0.0.1:8088/import ```
