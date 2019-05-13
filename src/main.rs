#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::Data;
use rocket::response::content;

extern crate rand;

extern crate syntect;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_file;

mod paste_id;
use paste_id::PasteID;

mod lang;
use lang::Lang;

use std::io;
use std::path::Path;
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

      DELETE /<id>

          deletes the paste identified with id `<id>`

    © 2019 Edgar Albalate <dev.gardo@gmail.com> 
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


#[delete("/<id>")]
fn delete(id: PasteID) -> io::Result<String> {
    let filename = format!("upload/{}", id);
    std::fs::remove_file(filename)?;
    Ok(format!("File `{}` deleted.\n", id))
}

#[put("/<id>", data = "<paste>")]
fn put(id: PasteID, paste: Data) -> io::Result<String> {
    let filename = format!("upload/{}", id);
    let url = format!("{}/{}\n", "http://localhost:8000", id);
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>/<lang>")]
fn retrieve_syntaxed(id: PasteID, lang: Lang) -> content::Html<String>{
    let filename = format!("upload/{id}", id = id);
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
    let head = format!("<head><title>{}</title><style>{}</style></head>", id, style);
    let theme = &ts.themes["base16-ocean.dark"];
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    let body = format!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b);
    let code = highlighted_html_for_file(filename, &ss, theme).unwrap();
    let html: String = format!("<html>{}\n{}{}</body></html>", head, body, code);
    content::Html(html)
}

fn main() {
    rocket::ignite().mount("/", routes![
        // ROUTE /
        index, upload,
        // ROUTE /<id>
        retrieve, delete, put,
        // ROUTE /<id>/<lang>
        retrieve_syntaxed
    ]).launch();
}
