use std::fs; // For tapping into host File System. Needed to open, read and write files (such as HTML documents)
use std::str; // The legendary String, a struct of characters (individual letters, 1 byte each)
use std::time::{SystemTime}; // Just to query time of day to store in request object

// ===============================================
// Must be derived for copying and cloning in order to reference the routes in different parts of the program.


// The RouteHandle tuple-struct contains two public types (public to be accessed in iterations). A string reference with a terminal lifetime that should represent the request Method and URL sent, and a custom function which should always return a resizeable Vector with string parameter (the Response item).
#[derive(Copy, Clone)]
pub struct RouteHandle (pub &'static str, pub &'static dyn Fn(&Request) -> Response);

// Must be implemented with Copy, Clone, Send and Sync for transfer between threads, and the arguments given static lifetimes to last until thread handles them.
unsafe impl Send for RouteHandle {}
unsafe impl Sync for RouteHandle {}
// ===============================================

// ===============================================
pub struct Routes;

impl Routes {
  // Every time a request is made, the request is mapped against the route handlers provided by the programmer, and returns the handler the route is assigned to
  pub fn map(request: &Request, route_map: Vec<RouteHandle>) -> Response {

    let url = &request.url;
    for route in route_map {
      println!("Route: {}, URL: {}", route.0, url);

      if url == route.0 {
        return route.1(request);
      }
    };
    return Routes::error(&request);

  }
  // ===============================================
  pub fn error(request: &Request) -> Response {
    // Default returns an error
    println!("404 Page not found...");
    // return Response::send(Response::Buffer("video.mp4"));
    return Response::send("video.mp4");

  }
}
// ===============================================
#[derive(Debug)]
// ===============================================
pub struct Request {
  pub method: String,
  // body: Vec<String>,
  // cookies: Vec<String>,
  // fresh: bool,
  pub source_ip: String,
  // pub referrer: String,
  pub connection: String,
  // params: Vec<String>,
  pub url: String,
  pub time: SystemTime,
  pub protocol: String,
  // query: Vec<String>,
}
// =========================================

// Lifetime "a", the response will live as long as its fields. And will also be a "static" as shown below. 
pub enum Response {
  Buffer(Vec<u8>),
  File(String),
  // Text(&'a str),
  Empty,
}
// =========================================
impl Response {
// Static means the response will last for the entire lifelife of the program. This is not a problem, since it is passed to a thread, then absolved upon reaching the requester

  // fn parse(res: Self) -> Vec<u8> {
    // let views = "views/";
    // let resources = "resources/";
    // let file = fs::read(format!("{}{}", views, res)).unwrap();
    // match res => Vec<u8> {
    //   Self::Buffer(data) => fs::read(format!("{}{}", views, res)).unwrap(),
    //   => vec!()
    // }

    // let file = fs::read(format!("{}{}", views, res)).unwrap();
    // return file
    // data.as_bytes().to_vec()
    // match res {
    //   Self::Buffer(data) => fs::read(format!("{}{}", views, res)).unwrap(),
    //   Self::Text(data) => {
    //   data.as_bytes().to_vec()
    // },
    //   return res
    // return 
    // match res {
    //   Self::Buffer(data) => data,
    //   Self::Text(data) => {
    //     data.as_bytes().to_vec()
    //   },
      // Self::File(data) => {

      //   let file = fs::read(data);
      //   let views = "views/";
      //   let resources = "resources/";
      //   // print!("Okeeeyyy: {:?}", file);

      //   match file {
      //     Ok(file) => file,
      //     Err(error) => {
      //       // println!("Error? {}{}", views, data);

      //       let file = fs::read(format!("{}{}", views, data)).expect("File not found!");
      //       file
      //     },
      //   }
      // },
      // _ => vec!()
    // }
  // }
 // ========================================================
  pub fn send(path: &str) -> Response {
      // let file = fs::read(format!("{}{}", views, res)).unwrap();
      let views = "views/";
      let resources = "resources/";
      
      let file = fs::read_to_string(format!("{}{}", views, path));
      match file {
        Err(err) => {
          let file = fs::read_to_string(format!("{}{}", resources, path));
          match file {
            Err(err) => {
              Response::Buffer(vec!())
            },
            Ok(data) => {
              return Self::parse(data);
            },
            _ => Response::Buffer(vec!())
          }
      
        },
        Ok(data) => {
          return Self::parse(data);
        },
        _ => Response::Buffer(vec!())
      }
  
      // let mut res_bytes = Response::parse(res);


      // return Response::Buffer(file.unwrap())
  }
  // ========================================================
  pub fn parse (file: String) -> Response {
    let content_length = &file.len();
    let http_status = "HTTP/1.1 200 OK";
    let response_head = format!("{http_status}\r\nContent-Length: {content_length}\r\n\r\n{file}");

    // for i in 0..response_head.as_bytes().len() {
    //   file.insert(0 + i, response_head.as_bytes()[i]);
    // }
    // return Response::File(file);
    return Response::File(response_head);

  }
// -------------
}
// ===============================================
