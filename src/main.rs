use crane_webserver::*;

fn main() {
    let server = WebServer::bind("127.0.0.1:8888", |path, query| match path.as_str() {
        "/" => root(),
        "/todos" => todos(),
        _ => ResponseBuilder::new().status(HttpStatus::Not_Found).build(),
    })
    .unwrap();

    server.start();
}
fn todos() -> Response {
    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .body(r#"[{ "name": "do hw", "checked": "false"}]"#)
        .build()
}
fn root() -> Response {
    ResponseBuilder::new()
        .status(HttpStatus::OK)
        .header("Content-Type", "text/html")
        .body("<h1>Hello World</h1>")
        .build()
}
