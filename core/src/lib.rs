use std::net::{TcpListener, TcpStream};
// use std::io::BufReader;
use web_threads::*;
use std::io::prelude::*;
use std::str;
use routers::*;
use request_response::*;
use httparse::EMPTY_HEADER;


pub fn server_listen <'a> (addr: &str, port: &str, threads: usize, route_map: Vec<RouteHandle>) {

  let listener = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();
  // Opens a TCP Listener, a protocol that will listen for requests across the internet/network. We bind the listener to a PORT number, and assume it has no errors (unwrap). 
  let pool = ThreadPool::new(threads); // Initiate 4 threads. A server CPU with less than two cores cannot achieve this.

  for stream in listener.incoming() {
    // Listens for a request (the data stream) from the client (a browser)
    let stream = stream.unwrap();
    
    let route_map = route_map.clone();
    //Copies the route map data to another place in memory by "cloning it" and passing it to one of the four available threads.
    pool.execute(|| {
      handle_connection(stream, route_map);
    });
  }
}
/* =========================================== */

/* ========================================== */
fn handle_connection (mut tcp_stream: TcpStream, route_map: Vec<RouteHandle>) {

  let mut buffer = [0; 1024]; // Buffer size. Each number represents one byte (one character length). 1024 is ideal for catching the essential information from a TCP request. 
  let mut headers = [EMPTY_HEADER; 16];
  tcp_stream.read(&mut buffer).unwrap(); // Parses binary buffer from the request


  let request = forge_request(&buffer, &mut headers);

  let response = Routes::map(&request, route_map); // Maps the url+method from request struct to appropriate route defined by programmer in the "route_map" vector, which was declared in main file. Response will ultimately be turned into a Vec<u8> buffer stream, but is handled differently depending on the return stream and type of data received (i.e, a file, plain text, data stream). 

  
  match response {
    Response::Webpage(file) => tcp_stream.write_all(file.as_bytes()).unwrap(), //Writes the string data from file into a buffer for the TCP stream and returns the binary data to client as a response
    Response::File(buf) => tcp_stream.write_all(&buf).unwrap(),
    // Response::Stream(buf) => tcp_stream.write_all(&buf).unwrap(),
    _ => panic!("Response invalid!") //If the response is a UDP request or something? This should rarely occur
  }
  
  // Write our string (response) to the stream, sending it back down the TCP connection line to be received by client. This is the foundation for sending HTML, which Browsers can interpret and read via HTTP response headers
}