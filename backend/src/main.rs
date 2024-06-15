use std::sync::Arc;
use rocket::{http::Status, http::Method, serde::json::Json, serde::Deserialize, serde::Serialize, Request};
use rocket_cors::{AllowedOrigins, CorsOptions};
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

// Structs

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Channel {
    id: String,
    title: String,
    custom_url: String,
    description: String,
    thumbnail: String,
    subscriber_count: String,
    view_count: String,
    video_count: String,
}

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
async fn get_channels(query: &str) -> Option<Json<Vec<Channel>>> {
    let youtube_api_key =
        std::env::var("YOUTUBE_API_KEY").expect("YOUTUBE_API_KEY must be defined.");

    // Make request to YouTube API
    let resp: serde_json::Value = reqwest::get(format!(
        "https://www.googleapis.com/youtube/v3/channels?part=snippet,statistics&forHandle={query}&maxResults=10&key={youtube_api_key}"
    ))
    .await
    .unwrap()
    .json()
    .await
    .unwrap();

    // Iterate through response items and build channel vector
    let mut final_items: Vec<Channel> = Vec::new();
    if let serde_json::Value::Array(items) = &resp["items"] {
        for item in items {
            let channel = Channel {
                id: item["id"].as_str().unwrap().to_string(),
                title: item["snippet"]["title"].as_str().unwrap().to_string(),
                custom_url: item["snippet"]["customUrl"].as_str().unwrap().to_string(),
                description: item["snippet"]["description"].as_str().unwrap().to_string(),
                thumbnail: item["snippet"]["thumbnails"]["default"]["url"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                subscriber_count: item["statistics"]["subscriberCount"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                view_count: item["statistics"]["viewCount"]
                    .as_str()
                    .unwrap()
                    .to_string(),
                video_count: item["statistics"]["videoCount"]
                    .as_str()
                    .unwrap()
                    .to_string(),
            };

            final_items.push(channel);
        }

        Some(Json(final_items))
    } else {
        None
    }
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

    let cors = CorsOptions::default().allowed_origins(AllowedOrigins::some_exact(&["http://127.0.0.1:8002", "http://localhost:8002"])).allowed_methods(vec![Method::Get].into_iter().map(From::from).collect()).allow_credentials(true);

    rocket::build()
        .manage(Context { db })
        .register("/", catchers![not_found, default_catcher])
        .attach(cors.to_cors().unwrap())
        .mount(
            "/",
            routes![index, get_users, get_subscriptions, get_channels],
        )
}
