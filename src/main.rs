#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::{Json, Value};

mod db;
mod schema;

mod user;
use user::User;


#[get("/")]
fn read_all(connection: db::Connection) -> Json<Value> {
    Json(json!(User::read_all(&connection)))
}


#[post("/", data = "<user>")]
fn create(user: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = user.into_inner();
    Json(Hero::create(insert, &connection))
}

#[get("/<id>")]
fn read(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!(User::read(id, &connection)))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, connection: db::Connection) -> Json<Value> {
    let update = User { id: id, ..user.into_inner() };
    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": User::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/user", routes![create, update, delete, read])
        .mount("/users", routes![read_all])
        .launch();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(1,1);
    }

    #[test]
    fn test2() {
        assert_eq!(2,2);
    }
}
