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
async fn roster() -> String {
    let token = battle_net::get_oauth_token(
        "f966f44ce65d45fda45d60f480158dab",
        "r16jKjXVp1WdSPRb6eHnBE6i2jlrnF9L",
        "us");

    let token = token.await;
    let x = match token {
        Ok(t) => {
            let res = get_roster(t.access_token);
            let res = res.await;
            let r = match res {
                Ok(body) => body,
                _ => "Error!!".to_string()
            };
            r
        }
        Err(err) => format!("Error occured {}", err.to_string())
    };

    x
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![roster])
        .attach(Template::fairing())
}