#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use rocket_contrib::json::{Json, JsonValue};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub mod database;
pub mod models;
pub mod schema;

use models::Note;

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn get_all_note(conn: database::Connection) -> Json<JsonValue> {
    Json(json!(Note::read_all(&conn)))
}

#[get("/<id>")]
fn get_single_note(id: i32, conn: database::Connection) -> Json<JsonValue> {
    Json(json!(Note::read_single(id, &conn)))
}

#[post("/", data = "<note>")]
fn add_note(note: Json<Note>, conn: database::Connection) -> Json<Note> {
    let insert = Note {
        id: None,
        title: note.title.clone(),
        body: note.body.clone(),
        created_at: None,
        modified_on: None,
    };
    Json(Note::create(insert, &conn))
}

#[put("/<id>", data = "<note>")]
fn update_note(id: i32, note: Json<Note>, conn: database::Connection) -> Json<JsonValue> {
    let update = Note {
        id: Some(id),
        title: note.title.clone(),
        body: note.body.clone(),
        created_at: None,
        modified_on: None,
    };
    Json(json!({ "success": Note::update(id, update, &conn) }))
}

#[delete("/<id>")]
fn delete_note(id: i32, conn: database::Connection) -> Json<JsonValue> {
    Json(json!({ "status": Note::delete(id, &conn) }))
}

fn main() {
    rocket::ignite()
        .attach(CORS())
        .manage(database::connect())
        .mount(
            "/api",
            routes![
                get_all_note,
                get_single_note,
                add_note,
                update_note,
                delete_note
            ],
        )
        .launch();
}
