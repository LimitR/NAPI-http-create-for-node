#[macro_use]
extern crate napi;
extern crate num_cpus;

use std::collections::HashMap;
use std::env;
use std::io::Bytes;
use napi_derive::napi;
use napi::bindgen_prelude::*;
use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use serde_json::value::RawValue;


#[macro_use]
extern crate serde_json;


#[napi]
fn get_cwd<T: Fn(String) -> Result<()>>(callback: T) -> ()  {
    return callback("Hello".to_string()).unwrap()
}


#[napi]
async fn run(host: String, port: i32) -> Result<()>{

    server(host, port);
    Ok(())
}


#[actix_web::main]
async fn server(data: String, host: i32) -> std::io::Result<()> {
    println!("Server is running on http://{}:{}", &data, &host);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_hello))
            .route("/post", web::post().to(post_hello))
    })
    .bind((&data as &str, host as u16))?
    .run()
    .await
}

async fn get_hello()-> impl Responder {
    HttpResponse::Ok().body("Win")
}


async fn post_hello(data: String) -> impl Responder {
        let mut r: serde_json::Value = serde_json::from_str(&data).unwrap();
        let result = r[6]["friends"][0]["name"].as_str().unwrap().to_owned();
    HttpResponse::Ok().body(&result)
}
