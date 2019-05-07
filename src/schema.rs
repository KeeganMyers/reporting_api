table! {
    business (id) {
        id -> Text,
        name -> Varchar,
    }
}

table! {
    check_tbl (id) {
        id -> Text,
        business_id -> Text,
        employee_id -> Text,
        name -> Varchar,
        closed -> Bool,
        closed_at -> Timestamptz,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    employee (id) {
        id -> Text,
        business_id -> Text,
        first_name -> Varchar,
        last_name -> Varchar,
        pay_rate -> Numeric,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    labor_entry (id) {
        id -> Text,
        business_id -> Text,
        employee_id -> Text,
        name -> Varchar,
        clock_in -> Timestamptz,
        clock_out -> Timestamptz,
        pay_rate -> Numeric,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    ordered_item (id) {
        id -> Text,
        business_id -> Text,
        employee_id -> Text,
        check_id -> Text,
        item_id -> Text,
        cost -> Numeric,
        price -> Numeric,
        voided -> Bool,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    business,
    check_tbl,
    employee,
    labor_entry,
    ordered_item,
);
