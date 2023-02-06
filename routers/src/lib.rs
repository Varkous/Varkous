use std::{str, fs, env::current_dir}; // The legendary String, a struct of characters (individual letters, 1 byte each), and <fs> for tapping into host File System. Needed to open, read and write files (such as HTML documents)
use httparse::{Request, Header, Status};
use std::process::{Command, Stdio};

// ========================================================
pub fn forge_request <'buf, 'headers> (request_buffer: &'buf [u8; 1024], headers: &'headers mut [Header<'buf>; 16]) -> Request<'headers, 'buf> {

  let mut request = Request::new(headers);
  let parsed = request.parse(request_buffer);
  match parsed {
    Ok(req) => req,
    Err(err) => {
      println!("Error in request? {}", err);
      Status::Partial
    }
  };
  // request.parse(request_buffer).unwrap();
  return request;


}

// The RouteHandle tuple-struct contains two public types (public to be accessed in iterations). A string reference with a terminal lifetime that should represent the request Method and URL sent, and a custom function which should always return a resizeable Vector with string parameter (the Response item).
#[derive(Copy, Clone)]
pub struct RouteHandle (pub &'static str, pub &'static dyn Fn(&Request) -> Response);

// Must be implemented with Copy, Clone, Send and Sync for transfer between threads, and the arguments given static lifetimes to last until thread handles them.
unsafe impl Send for RouteHandle {}
unsafe impl Sync for RouteHandle {}
// ===============================================

pub struct Routes;

impl Routes {
  /* ----------------------------------------------------------------------------------------------------- */
  // Every time a request is made, the request is mapped against the route handlers provided by the programmer, and returns the handler the route is assigned to
  pub fn map(request: &Request, route_map: Vec<RouteHandle>) -> Response {

    println!("Request: {:?}", request);

    let method = match request.method {
      Some(str) => str,
      None => "Error"
    };
    
    if method == "Error" {
      return Response::Error(404)
    }

    let url =  request.path.unwrap();
    let path = format!("{method} {url}");

    for route in route_map {
    // Iterate over route paths. Index .0 is the url path, index.1 is the file returned by that url/path
  
      if path == route.0 {
        println!("Found: {}", url);
        return route.1(request);
      }
    };
   
    return Response::send(request, &url, 200);
  }

}
// ===============================================

pub enum Response {
  Buffer(Vec<u8>),
  Error(i32),
}

// ===============================================
impl Response {
/* ----------------------------------------------------------------------------------------------------- */
  pub fn get_file (mut url: &str) -> String {

    // Remove initial forward flash and replace %20 space chars with actual Spaces for correct linux-file searching
    let mut url_chars = url.chars();
    if url_chars.next().unwrap() == '/' {
      url = url_chars.as_str();
    }

    let mut base_directory = current_dir().unwrap().display().to_string();
    base_directory.push_str(url);

    let filename: String = replace_all(url.to_string(), "%20", " ");

      let find_file = Command::new("/usr/bin/find")
      .args(["-name", &filename])
      .output()
      .expect("Failed to execute");

    
      let printed_file_name = match str::from_utf8(&find_file.stdout) {
        Ok(text) => text.to_string().replace("./", "").replace("\n", ""),
        Err(_) => panic!("got non UTF-8 data from stdout"),
      };

    // Not an actual file, just its name, as a string
    return printed_file_name
  }


/* ----------------------------------------------------------------------------------------------------- */
  pub fn send(request: &Request, filepath: &str, http_status: i32) -> Response {

    let filename = Response::get_file(filepath);
    if filename.len() < 1 {
      return Response::Error(404);
    }

    let file = fs::read(filename);

    return match file {
      // The if-statement above will catch 99% of missed files. However, if the file read itself fails (somehow file was deleted/moved mid-request), then we need this match to return error
      Ok(mut buffer) => {
        let content_length = buffer.len();
        let status_confirm = match http_status {
          404 => "NOT FOUND",
          _ => "OK"
        };
  
        let mut response: Vec<u8> = format!("HTTP/1.1 {http_status} {status_confirm} \r\n Content-Length: {content_length}\r\n\r\n").into_bytes();
        // let mut response: Vec<u8> = format!("HTTP/1.1 200 OK \r\n Content-Length: {content_length}\r\n\r\n").into_bytes();

        response.append(&mut buffer);
        return Response::Buffer(response);
      },
      Err(why) => {
        println!("File read failed: {}{}", filepath, why);
        Response::Error(404)
      }
    }
  }

}

/* ----------------------------------------------------------------------------------------------------- */
// Replaces all instances of the given "replace" string with the "with" string. 
pub fn replace_all (mut text: String, replace: &str, with: &str) -> String {

  while text.contains(replace) {
    text = text.replace(replace, with);
  }

  return text.to_string()
}