// Rocket Routes for the App
use crate::structs::List::*;
use crate::traits::List::*;
use crate::implementations::List::*;

use crate::PBHouse;
use markdown::{
    to_html_with_options, CompileOptions, Constructs, LineEnding, Options, ParseOptions,
};
use rocket::fs::FileServer;
use rocket::http::ContentType;
use tera::{Context, Tera};

#[get("/")]
pub fn index() -> (ContentType, String) {
    let this_pb = PBHouse::new();
    let app = App::new();
    let mut tera = Tera::default();
    let mut context = Context::new();

    tera.add_template_file("content/templates/index.html", Some("index.html"))
        .unwrap();

    context.insert("app_name", &"Fantasy Tavern Maker");
    context.insert("page_title", &"Example Output");
    context.insert("pb_name", &this_pb.name);

    let lf = LineEnding::LineFeed;
    let pb_general_info: Vec<String> = this_pb
        .general_info()
        .into_iter()
        .map(|line| markdown::to_html(&line))
        .collect();
    context.insert("pb_general_info", &pb_general_info);

    let history_profile: Vec<String> = this_pb
        .establishment_history_notes
        .into_iter()
        .map(|line| markdown::to_html(&line))
        .collect();
    context.insert("pb_history_profile", &history_profile);

    let redlight_profile: Vec<String> = this_pb
        .redlight_services
        .into_iter()
        .map(|line| markdown::to_html(&line))
        .collect();
    context.insert("redlight_profile", &redlight_profile);

    (
        ContentType::HTML,
        tera.render("index.html", &context)
            .expect("Vaild Tera Template"),
    )
}

#[get("/version")]
pub fn version() -> (ContentType, String) {
    let mut tera = Tera::default();

    tera.add_template_file("content/templates/version.html", Some("version.html"))
        .unwrap();
    let mut context = Context::new();
    let app = App::new();
    context.insert("app_name", &app.name);
    context.insert("app_version", &app.get_version());
    context.insert("page_title", &"App Version");
    (
        ContentType::HTML,
        tera.render("version.html", &context)
            .expect("Vaild Tera Template"),
    )
}
// ---- end of file ----
