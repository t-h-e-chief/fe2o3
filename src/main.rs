// main.rs
use actix_web::{web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use serde::{Serialize, Deserialize};

/// Representation of a service health check
#[derive(Serialize, Deserialize, Debug)]
struct HealthCheck {
    /// array of application manifests
    #[serde(rename = "fe3o2")]
    application: Vec<Manifest>,
}

/// Representation of an application manifest
#[derive(Serialize, Deserialize, Debug)]
struct Manifest {
    /// The application release version
    version: String,
    /// The application description
    description: String,
    /// The last commit sha for the git repository
    lastcommitsha: String,
}

/// Example handler for the web server root. Returns "Hello World"
///
/// # Arguments
///
/// * `_req` - The HttpRequest object
///
fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

/// Returns the server status as a json blob
///
/// # Arguments
///
/// * `_req` - The HttpRequest object
///
fn status(_req: HttpRequest) -> impl Responder {
    // Build the status info
    let manifest = Manifest {
        version: build::version().to_string(),
        description: build::description().to_string(),
        lastcommitsha: build::short_sha().to_string()
    };
    // Build the application health check.
    let status = HealthCheck { application: vec![manifest] };
    // Convert the Point to a JSON string.
    let body = serde_json::to_string_pretty(&status).unwrap();
    // Return the response.
    HttpResponse::Ok().content_type("application/json").body(body)
}

/// Runs the web server on port 8080
///
fn main() {
    println!("Http Server running on port 8080.");
    println!("Press ctrl+c to exit.");
    println!("");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/status", web::get().to(status))
    })
    .bind("0.0.0.0:8080")
    .unwrap()
    .run()
    .unwrap();
}

#[test]
fn test_build_version() {
    assert_eq!(build::version().to_string(), "1.0.0");
}

#[test]
fn test_build_description() {
    assert_eq!(build::description().to_string(), "interview technical test");
}


// Include codegen files produced during compilation process.
mod build {
    include!(concat!(env!("OUT_DIR"), "/version.rs"));
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}
