#[macro_use] extern crate rocket;
use portfolioCMS::models::*;
use diesel::prelude::*;
use portfolioCMS::*;

#[get("/")]
fn index() {
    println!("hit!");
    use portfolioCMS::schema::projects::dsl::*;

    let connection = &mut establish_connection();
    let results = projects
        .limit(5)
        .select(Project::as_select())
        .load(connection)
        .expect("Error loading projects");

    println!("Displaying {} posts", results.len());
    for project in results {
        println!("{}", project.title);
        println!("-----------\n");
        println!("{}", project.bodyone);
        println!("{}", project.bodytwo);
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

