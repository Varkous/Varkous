use core::*;
use routers::*;
use request_response::*;
use httparse::Request;

/* =========================================== */


trait Handlers {
  #![allow(unused_variables)]
    // ===============================================
    fn home(request: &Request) -> Response {
      Routes::authenticate(request);
      println!("We're home! {:?}", request.path);
      return Response::send(request, "home.html");
    }
    // ===============================================
    fn contact(request: &Request) -> Response {
      println!("Contact me at varkous@protonmail.com");
      return Response::send(request, "contact.html");
    }
    // ===============================================
    fn about(request: &Request) -> Response {
      println!("I'm a computer programmer, specifically a freelance web developer and software engineer.");
      return Response::send(request, "about.html");
    }
    // ===============================================
    fn projects(request: &Request) -> Response {
      println!("The website you're viewing now, Simulacrum, Vigil, JAMP, FairShare are among them");
      return Response::send(request, "projects.html");
    }
    // ===============================================
    fn workorder(request: &Request) -> Response {
      println!("If you want a particular application or website developed for you, submit the order here.");
      return Response::send(request, "workorder.html");
    }
    // ===============================================
    fn vigil(request: &Request) -> Response {
      return Response::send(request, "vigil.html");
    }
    // ===============================================
    fn simulacrum(request: &Request) -> Response {
      return Response::send(request, "simulacrum.html");
    } 
    // ===============================================
    fn wow(request: &Request) -> Response {
      return Response::send(request, "wow.html");
    }
    // ===============================================
    fn favicon(request: &Request) -> Response {
      println!("Favicon...");
      return Response::send(request, "favicon.ico");
    }
    // ===============================================
    fn error(request: &Request) -> Response {
      println!("404 Page not found...");
      return Response::send(request, "error.html");
    }
    // ===============================================
    fn authenticate(request: &Request) {
      return println!("User authenticated...");
    }
    // ===============================================
}
impl Handlers for Routes {}

fn main() {

    let route_map = vec![
      RouteHandle("GET /", &Routes::home),
      RouteHandle("GET /contact", &Routes::contact),
      RouteHandle("GET /about", &Routes::about),
      RouteHandle("GET /projects", &Routes::projects),
      RouteHandle("GET /workorder", &Routes::workorder),
      RouteHandle("GET /favicon.ico", &Routes::favicon),
      RouteHandle("GET /vigil", &Routes::vigil),
      RouteHandle("GET /simulacrum", &Routes::simulacrum),
      RouteHandle("GET /wow", &Routes::wow),
    ];  
    // Routes::file_paths(vec!["views", "styles", "resources"]);
    server_listen("127.0.0.1", "7878",  6, route_map);
  
}
/* ========================================== */


