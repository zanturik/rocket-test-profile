use rocket_db_pools::Database;
use rocket_db_pools::Connection;
use rocket_db_pools::diesel::PgPool;

mod schema;

#[macro_use] extern crate rocket;


#[derive(Database)]
#[database("testdb")]
pub struct Db(pub PgPool);

use diesel::prelude::*;
use rocket_db_pools::diesel::prelude::RunQueryDsl;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::someentity)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Someentity {
    pub id: i32,
    pub name: String
}


#[get("/")]
async fn index(mut conn: Connection<Db>) -> Option<String> {
    use schema::someentity::dsl::*;
    match someentity.find(1).first::<Someentity>(&mut conn).await {
        Ok(entity) => Some(entity.name),
        Err(_) => None
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::init())
    .mount("/", routes![index])
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;

    #[test]
    fn test_hello() {
        use rocket::local::blocking::Client;

        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::ServiceUnavailable);

    }
}

