#[macro_use] extern crate rocket;
use portfolioCMS::models::*;
use diesel::prelude::*;
use portfolioCMS::*;

#[get("/")]
fn index() {
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

#[get("/create")]
fn create() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut bodyone = String::new();
    let mut bodytwo = String::new();
    let mut imgone = String::new();
    let mut imgtwo = String::new();

    let title = "mumble";
    let bodyone = "test";
    let bodytwo = "test";
    let imgone = "test";
    let imgtwo = "test";

    let project = create_project(connection, title, &bodyone, &bodytwo, &imgone, &imgtwo);
    println!("\nSaved draft {} with id {}", title, project.id);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create])
}

