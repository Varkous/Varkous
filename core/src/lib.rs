use std::net::{TcpListener, TcpStream};
use web_threads::*;
use dotenv::dotenv;
use std::io::{Read, Write};
use std::str;
use routers::*;
use httparse::EMPTY_HEADER;

// ===========================
#[macro_export]
macro_rules! str {
  ($a:expr) => {
      String::from($a)
  }
}


pub struct ServerOptions <'addr> {
    pub ip_address: &'addr str,
    pub port: usize,
    pub threads: usize,
    pub https: bool,
    pub route_map: Vec<RouteHandle>,
}

pub fn server_listen <'addr>(options: ServerOptions) {

  dotenv().ok();

  // Opens a TCP Listener, a protocol that will listen for requests across the internet/network. We bind the listener to a PORT number, and assume it has no errors (unwrap). 
  let listener = TcpListener::bind(format!("{}:{}", options.ip_address, options.port)).unwrap();

  // if (!options.https) {
  //   let cert = [
  //     std::env::var("key").expect("Private key (key) must be set in environment variable when using HTTPS"),
  //     std::env::var("cert").expect("Certificate (cert) must be set in environment variable when using HTTPS"),
  //     std::env::var("chain").expect("Chain (chain) must be set in environment variable when using HTTPS")];

  //     let acceptor = SslConnector::builder(SslMethod::tls()).unwrap().build();
  //     // &cert[0],&cert[1],&cert[2]
  // }

  let pool = ThreadPool::new(options.threads); // Initiate a given number of CPU threads. A server CPU with less than two cores cannot achieve this.

  for stream in listener.incoming() {
    // Listens for a request (the data stream) from the client (a browser)
    let stream = stream;
    match stream {
      Ok(stream) => {
        let route_map = options.route_map.clone();
        //Copies the route map data to another place in memory by "cloning it" and passing it to one of the four available threads when they are activated.
        pool.execute(|| {
          handle_connection(stream, route_map);
        });
      },
      Err(err) => println!("Stream upheld {}", err)
    }
    
  }
}
/* =========================================== */

/* ========================================== */
fn handle_connection (mut tcp_stream: TcpStream, route_map: Vec<RouteHandle>) {

  let mut buffer = [0; 1024]; // Buffer size. Each number represents one byte (one character length). 1024 is ideal for catching the essential information from a TCP request. 
  let mut headers = [EMPTY_HEADER; 16];
  tcp_stream.read(&mut buffer).unwrap(); // Parses binary buffer from the request


  let request = forge_request(&buffer, &mut headers);

  let response = Routes::map(&request, route_map); // Maps the url+method from request struct to appropriate route defined by programmer in the "route_map" vector, which was declared in main file. Response functions will locate all HTML/CSS/etc. files requested by client, which will ultimately be turned into a Vec<u8> buffer stream to be written to TCP stream below.

  match response {
    Response::Buffer(data) => {
      match  tcp_stream.write_all(&data) {
        Ok(stream) => stream,
        Err(err) => println!("TCP connection closed. {}", err)
      }
      // tcp_stream.flush().unwrap();
    }
  
    Response::Error(status) => {
      let error = Response::send(&request, "error.html", status);
      match error {
        Response::Buffer(data) => tcp_stream.write_all(&data).unwrap(),
        _ => print!("What now?")
      }
    }

  }
}



