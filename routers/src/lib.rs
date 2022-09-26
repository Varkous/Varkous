// use std::fs; // For tapping into host File System. Needed to open, read and write files (such as HTML documents)
use std::str; // The legendary String, a struct of characters (individual letters, 1 byte each)
use std::time::{SystemTime}; // Just to query time of day to store in request object
use request_response::*;
use httparse::Request;
use std::fs;

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

    let method = request.method.unwrap().to_owned();
    let url =  request.path.unwrap().to_owned();
    let path = format!("{method} {url}");

    let mut files: Vec<String> = Vec::new();
    for route in route_map {
      // println!("Route: {}, URL: {}", route.0, method + " " + &url);

      if path == route.0 {
        println!("Found: {}", url);
        return route.1(request);
      } else {
        for file in route.3 {
          files.push(fs::read_dir(file).unwrap());
        }
        
      }
    };
    // let paths = ;

    for ath in paths {
        println!("Name: {}", ath.unwrap().path().display())
    }
 
    println!("Other file: {}", path);
    return Routes::error(&request);

  }
  // ===============================================
  pub fn error(request: &Request) -> Response {
    // Default returns an error
    println!("404 Page not found...");
    // return Response::send(Response::Buffer("video.mp4"));
    return Response::send(request, "video.mp4");

  }

}
// ===============================================
