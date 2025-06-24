diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Text,
        notes -> Text,
        created_on -> diesel::sql_types::TimestamptzSqlite,
    }
}
