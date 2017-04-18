
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate json_db;

use serde::{Serialize, Deserialize};
use json_db::*;

fn main() {
    #[derive(Serialize, Deserialize)]
    struct Data {
        name: String,
        number: i32,
    }
    let data = Data{ 
        name: "JsonDB".to_string(), 
        number: 26 
    };
    let mut db = JsonDB::new("db.json", Json::make(data));

    let newdata = Data{
        name: "jsondb".to_string(),
        number: 9001,
    };
    db.update(Json::make(newdata));
    db.save();
}
