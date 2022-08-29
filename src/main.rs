// Made on basis of subxt/examples/examples/subscribe_all_events.rs
#![feature(plugin, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use futures::StreamExt;
use models::Event;
use models::NewEvent;
use routes::*;
use std::env;
use subxt::{OnlineClient, PolkadotConfig};

mod database;
mod models;
mod routes;
mod schema;
mod static_files;

#[subxt::subxt(runtime_metadata_path = "artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = database::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes![index, new, show, delete, update])
        .mount("/", routes![static_files::all, static_files::index])
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::task::spawn(async move {
        rocket().launch();
    });
    tracing_subscriber::fmt::init();

    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    let mut event_sub = api.events().subscribe().await?;

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let conn = PgConnection::establish(&database_url).unwrap();

    while let Some(events) = event_sub.next().await {
        let events = events?;
        let block_hash = events.block_hash();

        println!("  Dynamic event details: {block_hash:?}");
        for event in events.iter() {
            let event = event?;
            let index = event.index();
            let pallet = event.pallet_name();
            let variant = event.variant_name();

            let new_event = NewEvent {
                description: String::from(format!("{index}::{pallet}::{variant}")),
                additional_info: String::from("additional_info"),
            };
            Event::insert(new_event, &conn);
            println!("    {index}::{pallet}::{variant} ");
        }
    }

    Ok(())
}
