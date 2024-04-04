use rocket::get;
use rocket::routes;

#[get("/test")]
fn hello() -> &'static str {
    "Hello World\n"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![hello]).launch().await;
}
