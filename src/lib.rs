#[macro_use]
extern crate napi;
extern crate num_cpus;

use napi_derive::napi;
use napi::bindgen_prelude::*;
use actix_web::{body, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use napi::Env;

static mut ROUTE: Vec<String> = Vec::new();
static mut SERVICE: Vec<String> = Vec::new();

#[napi]
fn addRoute(route: String, service: String){
    unsafe{
        ROUTE.push(route);
        SERVICE.push(service);
    }
}

#[napi]
fn run(host: String, port: i32) -> Result<()>{
    server(host, port);
    Ok(())
}


#[actix_web::main]
async fn server(host: String, port: i32) -> std::io::Result<()> {
    println!("Server is running on http://{}:{}", &host, &port);
        HttpServer::new(|| {
            unsafe{
            App::new()
                .route(&ROUTE[0], web::get().to(|| {
                    HttpResponse::Ok().body(&SERVICE[0])
                }))
                .route(&ROUTE[1], web::get().to(|| {
                    HttpResponse::Ok().body(&SERVICE[1])
                }))
                .route(&ROUTE[2], web::get().to(|| {
                    HttpResponse::Ok().body(&SERVICE[2])
                }))
            }
        })
        .bind((&host as &str, port as u16))?
        .run()
        .await
}