/*
   Client sends a broadcast request to all the discoverable peers in the local network
   need a timeout to manage connection requests so that we don't do this all the time
*/
pub mod cmd;
pub mod discovery;
pub mod peer;
pub mod sample;
pub mod startup;

use startup::init_app;

#[tokio::main]
async fn main() {
    init_app().await;
}
