mod db;
mod display;
mod record;
mod table;
mod value;

use db::*;

fn main() {
    let mut person_db = DB::new();

    person_db.add_table("name table", field_types!("ID", "uint", "name", "str"));
    person_db.add_records("name table", vec![
        record!("ID", 0 as u32, "name", "nick"),
        record!("ID", 1 as u32, "name", "james"),
    ]).unwrap();

    person_db.add_table("age table", field_types!("ID", "uint", "age", "uint"));
    person_db.add_records("age table", vec![
        record!("ID", 0 as u32, "age", 15 as u32),
        record!("age", 45 as u32, "ID", 1 as u32),
    ]).unwrap();

    println!("{person_db}\n\n");

    let table = person_db
        .join_tables("name table", "age table", "ID")
        .unwrap();

    println!("{table}");
}
