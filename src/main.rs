#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Rocket;

mod routes {
    use rocket_contrib::Json;

    #[derive(Serialize)]
    pub struct Status {
        status: String,
        hello: String,
    }

    #[get("/status")]
    pub fn status() -> Json<Status> {
        Json(Status {
            status: String::from("Ok"),
            hello: String::from("world"),
        })
    }
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![routes::status])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/status").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("{\"status\":\"Ok\",\"hello\":\"world\"}".into()));
    }
}
