// ---- start of file ----
// Rocket Routes for the App
use rocket::fs::FileServer;
use rocket::http::ContentType;

#[get("/")]
pub fn index() -> (ContentType, String) {
    (
        ContentType::HTML,
        format!("Hello World!"),
    )
 }
// ---- end of file ----
