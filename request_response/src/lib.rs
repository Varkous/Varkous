use httparse::*;
use std::fs; // For tapping into host File System. Needed to open, read and write files (such as HTML documents)

// ===============================================
#[derive(Debug)]
// ===============================================
// pub struct Request {
//   // pub buffer: [u8; 1024],
//   pub method: String,
//   // body: Vec<String>,
//   // cookies: Vec<String>,
//   // fresh: bool,
//   pub source_ip: String,
//   // pub referrer: String,
//   pub connection: String,
//   // params: Vec<String>,
//   pub url: String,
//   pub time: SystemTime,
//   pub protocol: String,
//   // query: Vec<String>,
// }
// =========================================

pub enum Response {
  Stream(Vec<u8>),
  File(Vec<u8>),
  Webpage(String),
  // Text(&'a str),
  Empty,
}
// =========================================
impl Response {
// Static means the response will last for the entire lifelife of the program. This is not a problem, since it is passed to a thread, then absolved upon reaching the requester
 // ========================================================
  pub fn send(req: &Request, path: &str) -> Response {
      // let file = fs::read(format!("{}{}", views, res)).unwrap();
      let views = "views/";
      let resources = "resources/";
      
      let file = fs::read_to_string(format!("{}{}", views, path));
      match file {
        Err(err) => {

          let file = fs::read_to_string(format!("{}{}", resources, path));

          match file {
            Err(err) => Response::File(vec!()),
            Ok(data) => 
            return Self::parse(data),
            _ => Response::File(vec!())
          }
      
        },
        Ok(data) => return Self::parse(data),
        _ => Response::File(vec!())
      }
  }
  // ========================================================
  pub fn parse (file: String) -> Response {
    
    let content_length = file.len();
    let http_status = "HTTP/1.1 200 OK";
    let response_head = format!("{http_status}\r\nContent-Length: {content_length}\r\n\r\n{file}");

    // for i in 0..response_head.as_bytes().len() {
    //   file.insert(0 + i, response_head.as_bytes()[i]);
    // }
    // return Response::File(file);
    return Response::Webpage(response_head);

  }
// -------------
}

pub fn forge_request <'buf, 'headers> (request_buffer: &'buf [u8; 1024], headers: &'headers mut [Header<'buf>; 16]) -> Request<'headers, 'buf> {
  // pub fn forge_request <'buf, 'headers> (request_buffer: &'buf [u8; 1024], headers: &'headers mut [Header<'buf>; 16]) -> Request<'headers, 'buf> {

  let mut request = Request::new(headers);
  request.parse(request_buffer).unwrap();
  return request;

}