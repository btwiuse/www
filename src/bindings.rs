pub use ::http::{Method, Request, Response};

// Load bindings from WIT file.
// wit_bindgen_rust::import!({paths: ["../../wit/core/http.wit"]});
wit_bindgen::generate!({
    path: "./",
    exports: {
        "foo:bar/ihttp": Runner,
    }
});

struct Runner;

use crate::bindings::exports::foo::bar::ihttp::Guest;
impl Guest for Runner {
    fn run() -> u8 {
        0u8
    }
}
// wit_bindgen::generate!({path: "./wasi-http/wit/proxy.wit"});

/*
use self::httpworld::{
    send_http_request as host_send_http_request, HttpMethod, HttpRequest, HttpResponse,
};

pub use self::httpworld::HttpRequestError;

impl From<HttpResponse> for Response<Vec<u8>> {
    fn from(value: HttpResponse) -> Self {
        let mut builder = Response::builder().status(value.status);

        for (key, value) in value.headers.iter() {
            builder = builder.header(key, value);
        }

        match value.body {
            Some(data) => builder.body(data).unwrap(),
            None => builder.body(Vec::new()).unwrap(),
        }
    }
}

pub fn send_http_request<T>(req: Request<T>) -> Result<Response<Vec<u8>>, HttpRequestError>
where
    T: Into<Vec<u8>>,
{
    let method = match *req.method() {
        Method::GET => HttpMethod::Get,
        Method::POST => HttpMethod::Post,
        Method::PUT => HttpMethod::Put,
        Method::PATCH => HttpMethod::Patch,
        Method::DELETE => HttpMethod::Delete,
        Method::OPTIONS => HttpMethod::Options,
        Method::HEAD => HttpMethod::Head,
        _ => HttpMethod::Get,
    };

    let mut parsed_headers: Vec<(String, String)> = Vec::new();

    for (key, value) in req.headers().iter() {
        if let Ok(value) = value.to_str() {
            parsed_headers.push((key.to_string(), value.to_string()))
        }
    }

    let headers_slice: Vec<(&str, &str)> = parsed_headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<(&str, &str)>>();

    let uri = req.uri().to_string();
    let body: Vec<u8> = req.into_body().into();

    let request = HttpRequest {
        body: Some(body.as_slice()),
        headers: &headers_slice,
        method,
        params: &[],
        uri: &uri,
    };

    host_send_http_request(request).map(|http_req| http_req.into())
}
*/
