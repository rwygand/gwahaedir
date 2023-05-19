mod raider_io;

#[macro_use] extern crate rocket;
use std::collections::HashMap;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> =  HashMap::new();
    Template::render("index", context)
}

#[get("/roster")]
async fn roster() -> Template {
    let rio_client = raider_io::RaiderIO::new();
    let res = rio_client.get_roster();
    let res = res.await;
    Template::render("roster", res.unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![roster])
        .attach(Template::fairing())
}