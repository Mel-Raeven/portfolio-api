use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub bodyone: String,
    pub bodytwo: String,
    pub imgone: String,
    pub imgtwo: String

}

use crate::schema::projects;

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub bodyone: &'a str,
    pub bodytwo: &'a str,
    pub imgone: &'a str,
    pub imgtwo: &'a str,
}
