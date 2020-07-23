#[macro_use]
extern crate log;

use std::io::Read;
use std::{env, io};

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use curl::easy::{Easy, List};
use dotenv::dotenv;
use todoist_habit_tracker::todoist_habit_api::manager::*;

async fn index() -> impl Responder {
    "Hello you! Have a great day ❤️"
}

async fn webhook_complete_item(event: web::Json<Event>) -> io::Result<String> {
    let mut serialized = String::new();
    if event.has_day_string() {
        let content = event.add_one_day_to_content();

        // Data to send
        let mut event_update = EventUpdate {
            content: event.event_data.content.to_string(),
        };
        event_update.content = content;
        serialized = serde_json::to_string(&event_update)?;

        // Initialize Curl
        let mut easy = Easy::new();
        easy.url(
            format!(
                "https://api.todoist.com/rest/v1/tasks/{}",
                event.event_data.id
            )
            .as_str(),
        )?;
        easy.post(true)?;
        easy.post_field_size(serialized.len() as u64)?;

        // Header
        let mut list = List::new();
        let bearer = format!(
            "{}",
            env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN must be set")
        );
        list.append(format!("Authorization: Bearer {}", bearer).as_str())?;
        list.append("Content-Type: application/json")?;
        easy.http_headers(list)?;

        // Send
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| Ok(serialized.as_bytes().read(buf).unwrap_or(0)))?;
        transfer.perform()?;
    }
    let log_string = format!(
        "Event trigger name : {} - id : {} - content : {} - content sent : {}",
        event.event_name, event.event_data.id, event.event_data.content, serialized
    );
    info!("{}", log_string);
    Ok(log_string)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let address = format!(
        "127.0.0.1:{}",
        env::var("SERVER_PORT").expect("SERVER_PORT must be set")
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route(
                "/api/webhook/complete-item",
                web::post().to(webhook_complete_item),
            )
    })
    .bind(address)?
    .run()
    .await
}
