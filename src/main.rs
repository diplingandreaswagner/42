use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

// Message struct
#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    user: String,
    text: String,
}

// Chat server actor
struct ChatServer {
    clients: Mutex<Vec<actix_web_actors::ws::WebsocketContext<ChatSession>>>, // Use the full path to avoid ambiguity
}

impl Actor for ChatServer {
    type Context = actix::Context<Self>;
}

impl ChatServer {
    fn new() -> Self {
        ChatServer {
            clients: Mutex::new(Vec::new()),
        }
    }
}

// WebSocket session
struct ChatSession {
    server: web::Data<actix::Addr<ChatServer>>,
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let msg: ChatMessage = serde_json::from_str(&text).unwrap();
                println!("Received: {:?}", msg);
                // Broadcast to all clients
                for client in self.server.clients.lock().unwrap().iter() {
                    client.text(text.clone());
                }
            }
            _ => (),
        }
    }
}

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<actix::Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(ChatSession { server: srv }, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = ChatServer::new().start();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .route("/ws", web::get().to(chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}