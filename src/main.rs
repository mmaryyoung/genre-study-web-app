#![deny(warnings)]

use handlebars::Handlebars;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::sync::Arc;
use serde_json::json;

async fn router(
    req: Request<Body>,
    handlebars: Arc<Handlebars<'static>>,
) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        "/" => home_page(req, handlebars),
        "/about" => about_page(req),
        _ => not_found(req),
    }
}

fn home_page(
    _req: Request<Body>,
    hbs: Arc<Handlebars<'static>>,
) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(
        hbs.render("home", &json!({
            "genres": [
                "rock",
                "blues",
                "metal"
            ]
        }))
            .unwrap(),
    )))
}

fn about_page(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(
        "This is Mary Yang's research study project.",
    )))
}

fn not_found(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("404: page not found")))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let hbs = Arc::new(handlebars());
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(move |_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        let hbs = hbs.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| router(req, hbs.clone()))) }
    });

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
fn handlebars() -> Handlebars<'static> {
    let mut reg = Handlebars::new();
    // enable dev mode for template reloading
    reg.set_dev_mode(true);
    // register a template from the file
    // modified the file after the server starts to see things changing
    reg.register_template_file("home", "./src/templates/index.hbs")
        .unwrap();
    reg.register_template_file("styles", "./src/templates/styles.hbs")
        .unwrap();
    reg.register_template_file("navbar", "./src/templates/navbar.hbs")
        .unwrap();

    reg
}
