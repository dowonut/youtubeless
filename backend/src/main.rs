use std::sync::Arc;

use rocket::{http::Status, serde::json::Json, Request};

use dotenv::dotenv;

#[macro_use]
extern crate rocket;

#[allow(warnings, unused)]
pub mod prisma;

use prisma::{subscription, user};

#[derive(Clone)]
pub struct Context {
    pub db: Arc<prisma::PrismaClient>,
}

pub type Ctx = rocket::State<Context>;

// Routing

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
async fn get_users(ctx: &Ctx) -> Json<Vec<user::Data>> {
    let query = ctx.db.user().find_many(vec![]);
    Json(query.exec().await.unwrap())
}

#[get("/subscriptions")]
async fn get_subscriptions(ctx: &Ctx) -> Json<Vec<subscription::Data>> {
    let query = ctx.db.subscription().find_many(vec![]);
    Json(query.exec().await.unwrap())
}

#[get("/search?<query>")]
async fn get_channels(query: &str) {
    println!("{query}");

    let youtube_api_key =
        std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be defined.");

    let resp: serde_json::Value = reqwest::get(format!(
        "https://www.googleapis.com/youtube/v3/search?part=id&key={youtube_api_key}"
    ))
    .await
    .unwrap()
    .json()
    // .text()
    .await
    .unwrap();

    println!("{resp:#?}");
}

// Catchers

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Failed to find '{}'. Try something else?", req.uri())
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> String {
    format!(
        "Something went wrong 🤔 ({}). Failed at request '{}'",
        status.code,
        req.uri()
    )
}

// Launch

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        prisma::new_client()
            .await
            .expect("Failed to create a Prisma client."),
    );

    dotenv().ok();

    rocket::build()
        .manage(Context { db })
        .register("/", catchers![not_found, default_catcher])
        .mount(
            "/",
            routes![index, get_users, get_subscriptions, get_channels],
        )
}
