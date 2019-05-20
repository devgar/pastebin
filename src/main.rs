#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::{Data, State};
use rocket::response::content;

extern crate rand;

extern crate syntect;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet, Theme};
use syntect::html::highlighted_html_for_file;

mod paste_id;
use paste_id::PasteID;

mod lang;
use lang::Lang;

use std::io;
use std::path::Path;
use std::fs::File;

struct Highlight {
    ss: SyntaxSet,
    theme: Theme,
    r: u8, g: u8, b:u8
}

static STYLE: &'static str = "
    pre {
        font-size:13px;
        font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
    }";

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
fn retrieve_syntaxed(id: PasteID, lang: Lang, h: State<Highlight>) -> ontent::Html<String> {
    let filename = format!("upload/{id}", id = id);
    let head = format!("<head><title>{} - {}</title><style>{}</style></head>", id, lang, STYLE);
    let body = format!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", h.r, h.g, h.b);
    let code = highlighted_html_for_file(filename, &h.ss, &h.theme).unwrap();
    content::Html(format!("<html>{}\n{}{}</body></html>", head, body, code))
}

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = ts.themes["base16-ocean.dark"].clone();
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    rocket::ignite()
    .manage(Highlight {
        ss: ss,
        theme: theme,
        r: c.r, g: c.g, b: c.b
    })
    .mount("/", routes![
        // ROUTE /
        index, upload,
        // ROUTE /<id>
        retrieve, delete, put,
        // ROUTE /<id>/<lang>
        retrieve_syntaxed
    ]).launch();
}
