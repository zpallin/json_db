# json\_db

A quick and easy json file management tool. The problem I was having is i/o operations in rust can get kind of messy. Alternative json file managers were either too clunky or had an unremoveable feature I didn't want to deal with. So I made my own.

## dependencies

| rust version | 1.16.0 |
| serde | 0.9.13 |
| serde\_derive | 0.9.13 |
| serde\_json | 0.9.10 |

## how to use

Not too pretty just yet, but it's a start.

```rust
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
```
