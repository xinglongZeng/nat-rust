use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,guard,Result};
use actix_web::middleware::Logger;
use env_logger::Env;
use serde::{Serialize,Deserialize};
use log::{ debug};
use std::fmt::{Debug};
use actix::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<>{
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    println!("Hello, world!");
}
