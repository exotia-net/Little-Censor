mod server;
mod handlers;

use std::{fs::File, io::Read};

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use env_logger;

use rustrict::{Trie, Type};

use serde::Deserialize;
use server::WebSocket;

async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocket::new(), &req, stream)
}

// # Maybe
// pub trait VulgarType {
//     fn to_vulgar_type(&self) -> Type;
// }

// impl VulgarType for i32 {
//     fn to_vulgar_type(&self) -> Type {
//         match self {
//             0 => Type::PROFANE, // Bad words.
//             1 => Type::OFFENSIVE, // Offensive words.
//             2 => Type::SEXUAL, // Sexual words.
//             3 => Type::MEAN, // Mean words.
//             4 => Type::EVASIVE, // Words intended to evade detection.
//             5 => Type::SPAM, // Spam/gibberish/SHOUTING.
//             6 => Type::SAFE, // One of a very small number of safe phases. Recommended to enforce this on users who repeatedly evade the filter.
//             7 => Type::MILD, // Not that bad.
//             8 => Type::MODERATE, // Bad.
//             9 => Type::SEVERE, // Cover your eyes!
//             10 => Type::MILD_OR_HIGHER, // Any level; Type::MILD, Type::MODERATE, or Type::SEVERE.
//             11 => Type::MODERATE_OR_HIGHER, // Any level in excess of Type::MILD.
//             12 => Type::INAPPROPRIATE, // The default Type, meaning profane, offensive, sexual, or severely mean.
//             _ => Type::ANY, // Any type of detection (except SAFE).
//         }
//     }
// }

#[derive(Deserialize)]
struct Config {
    addr: String,
    port: u16,
    vulgars: Vec<String>,
}

impl Config {
    fn new() -> Self {
        Self {
            addr: "127.0.0.1".to_owned(),
            port: 3000,
            vulgars: vec![]
        }
    }
}

fn load_config() -> Result<Config, std::io::Error> {
    let mut file: File = File::open("config.json")?;
    let mut data: String = String::new();
    file.read_to_string(&mut data)?;
    let json: Config = serde_json::from_str(&data)?;
    Ok(json)
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = match load_config() {
        Ok(v) => v,
        Err(e) => {
            log::error!("{}", e);
            Config::new()
        },
    };

    unsafe {
        for vulgar in config.vulgars {
            Trie::customize_default().set(&vulgar, Type::INAPPROPRIATE);
        }
    }

    log::info!("Starting HTTP server...");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws").route(web::get().to(websocket_handler)))
    })
    .bind((config.addr, config.port))?
    .run()
    .await
}
