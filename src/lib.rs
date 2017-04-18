
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
pub use serde::{Serialize, Deserialize};
pub use std::default::Default;
pub use std::fs::File;
pub use std::io::{Read, Write};
pub use std::fs::OpenOptions;
pub use std::path::Path;

// generic empty struct for inserting empty db as default
#[derive(Serialize, Deserialize, Debug)]
pub struct EmptyJsonDB {}

//============================================================================== 
// Wrapper for the json DB which you will pass a mutable reference to the struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Json<S: Serialize + Deserialize> {
    data: S,
}

// empty trait for checking that it is a "jsonable" struct
// we are assuming
pub trait JsonAble {}
impl<S: Serialize + Deserialize> JsonAble for Json<S> {}

pub trait JsonMake<S: Serialize + Deserialize> {
    fn make(data: S) -> Json<S>;
}

impl<S: Serialize + Deserialize> JsonMake<S> for Json<S> {
    fn make(data: S) -> Json<S> {
        Json { data: data }
    }
}

//==============================================================================
// JsonDB

// decides how to write to the db
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonDB<J> where J: JsonAble {
    path: String,
    scheme: J,
}

impl<J> JsonDB<J> where J: JsonAble + Serialize + Deserialize {
    pub fn new(path: &str, scheme: J) -> Self {
        let mut db = JsonDB {
            path: path.to_owned(),
            scheme: scheme,
        };

        db.load(); 
        db
    }
    pub fn load(&mut self) {
        let path = Path::new(&self.path);

        if Path::exists(&path) {
            let mut f = File::open(&path).unwrap();
            let mut buf : String = String::new();
            f.read_to_string(&mut buf);
            //println!("LOADED FILE: {}", buf);
            self.scheme = serde_json::from_str(&buf).unwrap();
            //println!("SCHEME NOW: {}", self.get_data_string());
        } else {
            let serialized_scheme = self.get_data_string();
            //println!("NEW FILE: {}", serialized_scheme);

            match OpenOptions::new().create(true).write(true).open(&path) {
                Ok(ref mut file) => {
                    file.write_all(serialized_scheme.as_bytes()).unwrap();
                },
                Err(err) => { panic!("Failed to create file '{}'", &self.path); }
            }
        }
    }
    pub fn save(&self) {
        let path = Path::new(&self.path);

        let mut f = File::open(&path).unwrap();
        let serialized_scheme = self.get_data_string();

        //println!("SAVED: Write to '{}'\n{}", path.display(), serialized_scheme);

        match OpenOptions::new().write(true).open(&path) {
            Ok(ref mut file) => {
                file.write_all(serialized_scheme.as_bytes()).unwrap();
            },
            Err(err) => { panic!("Failed to write to file '{}'", &self.path); }
        }
    }
    pub fn update(&mut self, newdata: J) {
        self.scheme = newdata;
        let serialized_scheme = self.get_data_string(); 
        //println!("UPDATED: {}", serialized_scheme);
    }
    fn get_data_string(&self) -> String {
        serde_json::to_string_pretty(&self.scheme).unwrap()
    }
}

//============================================================================== 
// tests
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() { 
        use *;

        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct User {
            name: String,
            age: i32,
            occupation: String,
        }
        impl Default for User {
            fn default() -> User {
                User {
                    name: "".to_owned(),
                    age: -1,
                    occupation: "".to_owned(),
                }
            }
        }

        impl User {
            fn build() -> User {
                User::default()
            }
            fn name(&mut self, name: &str) -> &mut Self {
                self.name = name.to_owned();
                self
            }
            fn age(&mut self, age: i32) -> &mut Self {
                self.age = age;
                self
            }
            fn occupation(&mut self, occupation: &str) -> &mut Self {
                self.occupation = occupation.to_owned();
                self
            }
            fn finalize(&self) -> User {
                User{ 
                    name: self.name.to_owned(), 
                    age: self.age, 
                    occupation: self.occupation.to_owned() 
                }
            }
        }

        let mut my_json_db = JsonDB::new("./db.json", Json::make(User::default()));

        let mut new_user = User::build()
            .name("Zach")
            .age(30)
            .occupation("Programmer")
            .finalize();
        println!("DATA: {:?}", &new_user);
        my_json_db.update(Json::make(new_user.clone()));
        my_json_db.save();

        new_user.name = "Zeppelin".to_owned();
        println!("Changed user: {:?}", new_user);
        my_json_db.update(Json::make(new_user.clone()));
        my_json_db.save();
    }
}

