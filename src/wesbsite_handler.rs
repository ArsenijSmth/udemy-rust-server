use super::server::Handler;
use super::http::{Request, Response, StatusCode, Method};
use std::fs;

pub struct WebsiteHandler {
    publict_path: String
}

impl WebsiteHandler {
    pub fn new(publict_path: String) -> Self {
        Self { publict_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.publict_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.publict_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Attck attemptred: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}