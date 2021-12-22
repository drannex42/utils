/* Rocket.rs SPA Routing (for Svelte, React, Vue, &c)
 * The way this works is easy: this will load static files (your SPA) and create a catch all for all those inglorious '404s' that Rocket.rs /thinks/ exist. 
 * Since your SPA will have it's own routing library, this uses the 404 page to 'hack' Rocket to load the index.html of your SPA! 
 * If it is a true 404 - then you should handle that on your SPA logic side.
 * Note: If you are using Rocket/Rust to also be your backend server, then add a secondary '.mount("/", routes![routes_here])' immediately preciding the first mount in this file. I've added an inline comment to help.
*/

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

// Get the index, and load your index.html from your client build distribution.
#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("client/dist/index.html")
}

// This allows all files to be grabbed, this will be your assets, images, css, &c.
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/").join(file)).ok()
}

/*
 * This 'hacks' the Rocket.rs 404 logic, and will load the SPA index entry point and your own routing logic from within your SPA.
 * You should also include an error page in your own SPA! 
*/ 
#[catch(404)]
fn not_found() -> io::Result<NamedFile> {
    NamedFile::open("client/dist/index.html")
}


fn main() {
    rocket::ignite()
         // .mount("/api/v1", routes![hello]) // You can uncomment this to deal with your API endpoints.
        .mount("/", routes![index, files])
        .register(catchers![not_found])
        .launch();
}
