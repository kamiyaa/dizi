use actix::{Actor, StreamHandler};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

#[derive(Clone, Debug)]
struct WebSocketData {}

impl Actor for WebSocketData {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketData {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[post("/api/ws/connect")]
pub async fn connect(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocketData {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[post("/api/ws/disconnect")]
pub async fn disconnect(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocketData {}, &req, stream);
    println!("{:?}", resp);
    resp
}
