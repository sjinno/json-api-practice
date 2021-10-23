#[macro_use]
extern crate lazy_static;

use failure::Error;
use http::StatusCode;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::convert::Infallible;
use std::net::SocketAddr;

lazy_static! {
    static ref DATA: Value =
        serde_json::from_str(&std::fs::read_to_string("src/data.json").unwrap()).unwrap();
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Recipes {
    recipe_names: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(recipe_api))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}

async fn recipe_api(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("Hello, world!".into())),
        (&Method::GET, "/recipes") => {
            // println!("{}", DATA["recipes"]);
            let mut recipes = Recipes {
                recipe_names: vec![],
            };

            if let Value::Array(data) = &DATA["recipes"] {
                for recipe in data {
                    if let Value::String(name) = &recipe["name"] {
                        recipes.recipe_names.push(name.to_string());
                    }
                }
                // println!("{:?}", recipes);
                // return Ok(Response::new(
                //     (serde_json::to_string(&recipes).unwrap()).into(),
                // ));
                let body = serde_json::to_string(&recipes).unwrap().into();
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .body(body)
                    .unwrap();
                println!("{:?}", response);
                return Ok(response);
            }

            Ok(Response::new("".into()))
        }
        _ => Ok(Response::new("Hello, world!".into())),
    }
}
