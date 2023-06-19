pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewProject, Project};

pub fn create_project(conn: &mut PgConnection, title: &str, bodyone: &str, bodytwo: &str, imgone: &str, imgtwo: &str) -> Project {
    use crate::schema::projects;

    let new_project = NewProject { title, bodyone, bodytwo, imgone, imgtwo };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(Project::as_returning())
        .get_result(conn)
        .expect("Error saving new project")
}
