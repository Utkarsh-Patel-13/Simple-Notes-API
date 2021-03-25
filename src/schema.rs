table! {
    notes (id) {
        id -> Nullable<Integer>,
        title -> Varchar,
        body -> Text,
        created_at -> Nullable<Timestamp>,
        modified_on -> Nullable<Timestamp>,
    }
}
