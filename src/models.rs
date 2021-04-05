use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{self};
use diesel::{mysql::MysqlConnection, QueryDsl, RunQueryDsl};

use super::schema::notes;
#[derive(Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "notes"]
pub struct Note {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub created_at: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

impl Note {
    pub fn create(note: Note, conn: &MysqlConnection) -> Note {
        diesel::insert_into(notes::table)
            .values(&note)
            .execute(conn)
            .expect("Error creating new note");

        notes::table.order(notes::id.desc()).first(conn).unwrap()
    }

    pub fn read_all(conn: &MysqlConnection) -> Vec<Note> {
        notes::table
            .order(notes::id.asc())
            .load::<Note>(conn)
            .unwrap()
    }
    pub fn read_single(id: i32, conn: &MysqlConnection) -> Result<Note, Error> {
        notes::table.find(id).first(conn)
    }

    pub fn update(id: i32, note: Note, conn: &MysqlConnection) -> Result<usize, Error> {
        let n: Result<Note, Error> = Note::read_single(id, &conn);

        match n {
            Ok(_) => diesel::update(notes::table.find(id))
                .set(&note)
                .execute(conn),
            Err(e) => Err(e),
        }
    }

    pub fn delete(id: i32, conn: &MysqlConnection) -> Result<bool, Error> {
        let n: Result<Note, Error> = Note::read_single(id, &conn);
        match n {
            Ok(_) => Ok(diesel::delete(notes::table.find(id)).execute(conn).is_ok()),
            Err(e) => Err(e),
        }
    }
}
