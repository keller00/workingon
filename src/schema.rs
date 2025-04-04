diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        notes -> Text,
        created_on -> diesel::sql_types::TimestamptzSqlite,
        completed_on -> diesel::sql_types::Nullable<diesel::sql_types::TimestamptzSqlite>,
    }
}
