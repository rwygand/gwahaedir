mod battle_net;

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
    let token = battle_net::get_oauth_token(
        "f966f44ce65d45fda45d60f480158dab",
        "r16jKjXVp1WdSPRb6eHnBE6i2jlrnF9L",
        "us");

    let token = token.await;
    match token {
        Ok(t) => {
            let res = battle_net::get_roster(t.access_token);
            let res = res.await;
            Template::render("roster", res.unwrap())
        }
        Err(_) => Template::render("roster", ())
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![roster])
        .attach(Template::fairing())
}