use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todos {
    pub id: i32,
    pub title: String,
    pub notes: String,
    // TODO: time tracking
    pub created_on: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub notes: &'a str,
    pub created_on: DateTime<Utc>,
}

