use rocket;
use rocket_contrib::Template;
use rocket::response::NamedFile;

use std::path::{Path, PathBuf};

use std::collections::HashMap;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", context)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

pub fn run() {
    rocket::ignite().mount("/", routes![index, files]).attach(Template::fairing()).launch();
}