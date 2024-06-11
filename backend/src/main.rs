use std::sync::Arc;

use rocket::serde::json::Json;

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

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(
        prisma::new_client()
            .await
            .expect("Failed to create a Prisma client."),
    );

    // #[cfg(debug_assertions)]
    // db._db_push(false).await.unwrap();

    rocket::build()
        .manage(Context { db })
        .mount("/", routes![index, get_users, get_subscriptions])
}
