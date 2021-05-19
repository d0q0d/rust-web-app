use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::heroes;

#[table_name = "user"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl User {
    pub fn create(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(heroes::table)
            .values(&user)
            .execute(connection)
            .expect("Error creating new user");
        users::table.order(users::id.desc()).first(connection).unwrap()
    }

    pub fn read_all(connection: &MysqlConnection) -> Vec<User> {
        users::table.order(users::id).load::<User>(connection).unwrap()
    }

    pub fn read(id: i32, connection: &MysqlConnection) -> User {
        users::table.find(id).first::<User>(connection).unwrap()
    }    

    pub fn update(id: i32, user: User, connection: &MysqlConnection) -> bool {
        diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(users::table.find(id)).execute(connection).is_ok()
    }
}
