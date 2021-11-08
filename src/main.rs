#![allow(unused)]
use dotenv::dotenv;
use std::{env, net::SocketAddr};
use uuid_generator::serve_uuid;
#[tokio::main]
async fn main() {
    dotenv();
    dotenv().expect("Please consider adding configuration in a dotenv file");
    // env! fetches env variables at compilation so this won't work!
    // let uuid_server_ip = env!("UUID_SERVER_IP", "You forgot to specify UUID_SERVER_IP");
    let uuid_server_ip =
        env::var("UUID_SERVER_IP").expect("You forgot to specify UUID_SERVER_IP");
    // let server = serve_uuid().await;
    let uuid_server_ip: SocketAddr = uuid_server_ip
        .parse()
        .expect("Unable to parse socket address. Correct format = \"a.b.c.d:PORT\"");
    // Start uuid Server
    serve_uuid(uuid_server_ip).await;
}
