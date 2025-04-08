use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(treat_none_as_null = true)]
pub struct Todos {
    pub id: i32,
    pub title: String,
    pub notes: String,
    // TODO: time tracking
    pub created: DateTime<Utc>,
    pub completed: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub notes: &'a str,
    pub created: DateTime<Utc>,
}
