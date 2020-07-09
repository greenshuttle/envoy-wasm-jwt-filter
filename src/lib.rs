use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::Action;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct JwtAuthorizer {
    context_id: i32,
}

impl Context for JwtAuthorizer {}

impl HttpContext for JwtAuthorizer {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            println!("In WASM : #{} -> {}: {}", self.context_id, name, value);
        }
        let token = &String::from("jwt_token");
        match self.get_http_request_header("Authorization") {
            Some(Authorization) if Authorization.eq(token) => {
                self.resume_http_request();
                Action::Continue
            }
            _ => {
                self.send_http_response(
                    403,
                    vec![("Powered By", "proxy-wasm")],
                    Some(b"Access Forbidden.\n"),
                );
                Action::Pause
            }
        }
    }
}
