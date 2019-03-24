use std::env;
//use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use log::{debug, info, trace, error};
use pretty_env_logger;
use serde_derive::{Deserialize};
use serde_json;
use futures::{Async, Poll};

#[derive(Debug, Deserialize)]
struct ServiceInstance {
	name: String,
	start_time: u64,
}

#[derive(Debug, Deserialize)]
struct CatalogEntry {
	name:                        String,
	version: String,
	owner: String,
	capability: String,
	runtime: String,
	documentation: String,
	prometheusJob: String,
	minimum: usize,
	maximum: usize,
	authorized: bool,
	metered: bool,
	threaded: bool,
//	Instances: Vec<ServiceInstance>,
	MetricsTemplate:            String,
    ThreadsTemplate:            String,
	ZookeeperAnnouncementPoint: String,
}

fn main() {
    pretty_env_logger::init();
    info!("Catalog Fuzzer starts");

    let server_address: String = env::var("GM_CATALOG_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".into())
        .parse()
        .unwrap();

    let program = Program::new(server_address);

    rt::run(program);
}

fn get_catalog_entries(server_address: &str) -> impl Future<Item=Vec<CatalogEntry>, Error=FetchError> {
    let client = Client::new();

    let uri = format!("http://{}/services", server_address).parse().unwrap();
    info!("uri = {}", uri);

    client
        .get(uri)
        .and_then(|res| {
            trace!("Response: {}", res.status());
            trace!("Headers: {:#?}", res.headers());
            res.into_body().concat2()
        })
        .from_err::<FetchError>()
        .and_then(|body| {
            let catalog_entries = serde_json::from_slice(&body).unwrap();
            Ok(catalog_entries)
        }) 
        .from_err()
}

// Define a type so we can return multiple types of errors
#[derive(Debug)]
enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

struct Program {
    server_address: String,
}

impl Program {
    fn new(server_address: String) -> Self {
        Program{
            server_address: server_address,
        }
    } 
}

impl Future for Program {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        rt::spawn(get_catalog_entries(&self.server_address)
            // use the parsed vector
            .map(|catalog_entries| {
                // print users
                println!("users: {:#?}", catalog_entries);

            })
            // if there was an error print it
            .map_err(|e| {
                error!("Error: {:?}", e)
            })
        );
        rt::spawn(get_catalog_entries(&self.server_address)
            // use the parsed vector
            .map(|catalog_entries| {
                // print users
                println!("users: {:#?}", catalog_entries);

            })
            // if there was an error print it
            .map_err(|e| {
                error!("Error: {:?}", e)
            })
        );
        Ok(Async::NotReady)
    }

}