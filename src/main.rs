use axum::{Router, extract::Json, response::Json as ResponseJson, routing::post};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize)]
struct Person {
    // no: u32,
    name: String,
    age: u32,
    birthday: String,
}

#[derive(Serialize)]
struct PersonInfo {
    age: u32,
    birthday: String,
}

/// lets do this the rust way in case we need person info to person conversion in more places
impl From<Person> for PersonInfo {
    fn from(person: Person) -> Self {
        PersonInfo {
            age: person.age,
            birthday: person.birthday,
        }
    }
}

async fn group_users(Json(people): Json<Vec<Person>>) -> ResponseJson<HashMap<String, PersonInfo>> {
    let grouped: HashMap<String, PersonInfo> = people
        .into_iter()
        .map(|person| (person.name.clone(), person.into()))
        .collect();

    ResponseJson(grouped)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/group-users", post(group_users));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
