// Made on basis of subxt/examples/examples/subscribe_all_events.rs
#![feature(
    plugin,
    // custom_derive,
    // const_fn,
    decl_macro,
    // custom_attribute,
    proc_macro_hygiene
)]
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
use routes::*;
use std::env;

mod database;
mod models;
mod routes;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = database::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/api/v1/", routes![index, new, show, delete, update])
        .mount("/", routes![static_files::all, static_files::index])
}

// use database::database;
use futures::StreamExt;
use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "artifacts/polkadot_metadata.scale")]
pub mod polkadot {}
/// Subscribe to all events, and then manually look through them and
/// pluck out the events that we care about.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket().launch();
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;

    // Subscribe to any events that occur:
    let mut event_sub = api.events().subscribe().await?;

    // Our subscription will see the events emitted as a result of this:
    while let Some(events) = event_sub.next().await {
        let events = events?;
        let block_hash = events.block_hash();

        // We can dynamically decode events:
        println!("  Dynamic event details: {block_hash:?}");
        for event in events.iter() {
            let event = event?;
            let index = event.index();
            let pallet = event.pallet_name();
            let variant = event.variant_name();
            println!("    {index}::{pallet}::{variant} ");
        }
    }

    Ok(())
}
