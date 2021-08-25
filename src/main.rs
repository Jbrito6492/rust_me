#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;
use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};
use std::io;
use std::path::{Path, PathBuf};
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        // 4.
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
    ]);
    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

#[get("/api/myrocket")]
fn myrocket() -> String {
    "My 🚀 server".to_string()
}
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("frontend/public/").join(file)).ok()
}
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("build/index.html")
}
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, myrocket, files,])
        .attach(make_cors())
}
fn main() {
    rocket().launch();
}
