use core::*;
use routers::*;
use httparse::Request;
/* =========================================== */

trait Handlers {
  #![allow(unused_variables)]
    // ===============================================
    fn home(request: &Request) -> Response {
      Self::authenticate(request);
      println!("We're home! {:?}", request.path);
      return Response::send(request, "home.html", 200);
    }
    // ===============================================
    fn contact(request: &Request) -> Response {
      println!("Contact me at varkous@protonmail.com");
      return Response::send(request, "contact.html", 200);
    }
    // ===============================================
    fn about(request: &Request) -> Response {
      println!("I'm a computer programmer, specifically a freelance web developer and software engineer.");
      return Response::send(request, "about.html", 200);
    }
    // ===============================================
    fn projects(request: &Request) -> Response {
      println!("The website you're viewing now, Simulacrum, Vigil, JAMP, FairShare are among them");
      return Response::send(request, "projects.html", 200);
    }
    // ===============================================
    fn workorder(request: &Request) -> Response {
      println!("If you want a particular application or website developed for you, submit the order here.");
      return Response::send(request, "workorder.html", 200);
    }
    // ===============================================
    fn mpst(request: &Request) -> Response {
      println!("If you want a particular application or website developed for you, submit the order here.");
      return Response::send(request, "mpst.html", 200);
    }
    fn wow(request: &Request) -> Response {
      return Response::send(request, "wow.html", 200);
    }
    // ===============================================
    fn authenticate(request: &Request) {
      return println!("User authenticated...");
    }
    // ===============================================
    fn favicon(request: &Request) -> Response{
      println!("Favy...");
      return Response::send(request, "favicon.ico", 200);
    }
    // ===============================================
    fn error(request: &Request) -> Response {
      println!("404 Page not found...");
      return Response::send(request, "error.html", 404);
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
      RouteHandle("GET /favicon", &Routes::favicon),
      RouteHandle("GET /workorder", &Routes::workorder),
      RouteHandle("GET /mpst", &Routes::mpst),
      RouteHandle("GET /wowcharandom", &Routes::wow),
    ];  

    server_listen("192.168.3.253", "6001",  6, route_map);
  
}
/* ========================================== */


