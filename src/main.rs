#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rand;

mod paste_id;

use paste_id::PasteID;

use std::io;
use std::path::Path;

use rocket::Data;

use std::fs::File;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
  let id = PasteID::new(3);
  let filename = format!("upload/{id}", id = id);
  let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

  paste.stream_to_file(Path::new(&filename))?;
  Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}


fn main() {
    rocket::ignite().mount("/", routes![index, upload, retrieve]).launch();
}
