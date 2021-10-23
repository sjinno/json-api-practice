use failure::Error;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

use std::convert::Infallible;
use std::io::Read;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Data always exists so it's ok to unwrap only for this occasion
    let recipes = load_data().unwrap();
    println!("{}", &recipes);

    Ok(Response::new("Hello, World".into()))
}

fn load_data() -> Result<serde_json::Value, Error> {
    let mut data = String::new();
    {
        let stdin = std::io::stdin();
        stdin.lock().read_to_string(&mut data)?;
    }

    let recipes: serde_json::Value = serde_json::from_str(&data)?;
    eprintln!("{}", &recipes);

    Ok(recipes)
}
