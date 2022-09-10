use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use web_threads::*;
use std::io::prelude::*;
use std::str;
use routers::*;
use std::time::{SystemTime};
// use regex::Regex;


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
fn forge_request(buffer: [u8; 1024]) -> Result<Request, ()> {

  let mut req_string = str::from_utf8(&buffer).unwrap().replace("\r\n", ": ");
  let mut req_parts = req_string.split(" ");
  // println!("{:?}", req_string);

  let method = match req_parts.next() {
      Some(method) => method.trim().to_string(),
      None => return Err(()),
  };
  let url = match req_parts.next() {
      Some(path) => path.trim().to_string(),
      None => return Err(()),
  };
  let protocol = match req_parts.next() {
    Some(method) => method.trim().to_string(),
    None => return Err(()),
  };
  let host = match req_parts.next() {
    Some(version) => version.trim().to_string(),
    None => return Err(()),
  };
  let source_ip = match req_parts.next() {
      Some(version) => version.trim().to_string(),
      None => return Err(()),
  };
  let connection = match req_parts.next() {
    Some(version) => version.trim().to_string(),
    None => return Err(()),
  };
  let time = SystemTime::now();

  let request = Ok(Request {
    method,
    url,
    source_ip,
    connection,
    time,
    protocol,
  });
  // println!("{:?}", request);
  return request;

}
/* ========================================== */

// VERIFY IF THE REQUEST HEADER PROPERTY (Sec-Fetch-Dest:) is equal to image/audio/video. And attempt to return data of that file from resources


/* ========================================== */
fn handle_connection (mut request_stream: TcpStream, route_map: Vec<RouteHandle>) {

  let buffer_reader = BufReader::new(&mut request_stream);
  // let mut buffer = [0; 1024]; // Buffer size. Each number represents one byte (one character length). 1024 is ideal for catching the essential information from a TCP request. 
  // stream.read(&mut buffer).unwrap(); // Parses binary buffer from the request
  let request = buffer_reader.lines().next().unwrap().unwrap();

  // let request = forge_request(buffer).unwrap(); // <<WIP>> Translates binary bytes into text properties, storing them in a struct which tells the CPU how to store the request information in memory (request struct) 
  let response = Routes::map(&request.as_bytes(), route_map); // Maps the url+method from request struct to appropriate route defined by programmer in the "route_map" vector, which was declared in main file. Response will ultimately be turned into a Vec<u8> buffer stream, but is handled differently depending on the return stream and type of data received (i.e, a file, plain text, data stream). 

  // stream.write(&response).unwrap(); //Writes the now-structured data into a buffer for the TCP stream
  // stream.flush().unwrap() //Returns the binary data on the TCP stream to the client as a response
  
  match response {
    Response::File(file) => {
      println!("File? {:?}", file);
      stream.write_all(file.as_bytes()).unwrap(); //Writes the now-structured data into a buffer for the TCP stream
      stream.flush().unwrap() //Returns the binary data on the TCP stream to the client as a response
    }
    _ => panic!("Response invalid!") //If the response is a UDP request or something? This should rarely occur
  }
  
  // Write our string (response) to the stream, sending it back down the TCP connection line to be received by client. This is the foundation for sending HTML, which Browsers can interpret and read via HTTP response headers
}