#![allow(unused)]
use std::convert::Infallible;
use std::net::SocketAddr;
// use std::sync::Arc;
// use std::sync::Mutex;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use rand::{thread_rng, Rng};
use uuid::Uuid;

pub async fn serve_uuid(addr: SocketAddr) {
    // Step 1 : Generate a couple of UUIDs from thread_rng()
    // Step 2 : Start a server
    // Step 3 : Listen for connections, and serve them a UUID string on GET @ uuid_server/get_uuid
    // Step 4 : Write a client that polls for a UUID using Hyper

    let uuid_service = make_service_fn(|_| async {
        Ok::<_, Infallible>(service_fn(|_req: Request<Body>| {
            generate_session_uuid(_req)
        }))
    });

    let server = Server::bind(&addr).serve(uuid_service);
    if let Err(e) = server.await {
        eprintln!("Server error : {:?}", e);
    }
}

async fn generate_session_uuid(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // let mut lock_rng = rng.lock();
    // let bytes = rng.gen::<[u8; 16]>();
    // let mut builder = uuid::Builder::from_bytes(bytes);
    // Ok(Response::new(builder.build().to_string().into()))
    let random_uuid = Uuid::new_v4().to_string();
    if req.method() == Method::GET && req.uri().path().contains("get_uuid") {
        Ok(Response::new(random_uuid.into()))
    } else {
        Ok(Response::new("".into()))
    }
}
