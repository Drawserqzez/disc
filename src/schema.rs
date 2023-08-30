// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        event_time -> Timestamptz,
        created_time -> Nullable<Timestamptz>,
        domain -> Nullable<Varchar>,
        owner -> Nullable<Int8>,
    }
}
