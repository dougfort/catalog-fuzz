use std::env;
use std::io::{self, Write};
use failure::Error;
use hyper::Client;
use hyper::rt::{self, Future, Stream};
use log::{debug, info, trace};
use pretty_env_logger;

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    info!("Catalog Fuzzer starts");

    rt::run(rt::lazy(|| {
        let server_address: String = env::var("GM_CATALOG_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:8080".into())
            .parse()
            .expect("Can't parse GM_CATALOG_ADDRESS");

        let client = Client::new();

        let uri = format!("http://{}/services", server_address).parse().unwrap();
        info!("uri = {}", uri);

        client
            .get(uri)
            .and_then(|res| {
                trace!("Response: {}", res.status());
                res
                    .into_body()
                    // body is a stream; so as each chunk arrives
                    .for_each(|chunk| {
                        debug!("in chunk");
                        io::stdout()
                            .write_all(&chunk)
                            .map_err(|e| {
                                panic!("stdout failed: {}", e);
                            })
                    })
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })    
    }));

    Ok(())
}
