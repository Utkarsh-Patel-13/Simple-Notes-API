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

pub mod database;
pub mod models;
pub mod schema;

use models::Note;

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
