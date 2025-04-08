diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        notes -> Text,
        created -> diesel::sql_types::TimestamptzSqlite,
        completed -> diesel::sql_types::Nullable<diesel::sql_types::TimestamptzSqlite>,
    }
}
