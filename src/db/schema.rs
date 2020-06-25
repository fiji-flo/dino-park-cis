table! {
    use diesel::sql_types::*;
    use crate::db::types::*;

    profiles (uuid) {
        uuid -> Uuid,
        user_id -> Varchar,
        primary_email -> Varchar,
        primary_username -> Varchar,
        active -> Bool,
        trust -> Trust_type,
        version -> Int4,
        profile -> Jsonb,
    }
}
