// <setup>
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

// This struct represents state
struct AppState {
    app_name: String,
}

fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}
// </setup>

// <setup_mutable>
struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

fn _index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}
// </setup_mutable>

// <make_app_mutable>
fn _main() {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || { // move counter into the closure
        App::new()
            .register_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(_index))
    })
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
// </make_app_mutable>

// <start_app>
pub fn main() {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
// </start_app>
