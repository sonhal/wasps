use axum::{
    routing::get,
    Router, Json,
};

use log::info;
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
    .route("/", get(|| async { "WASP - 0.0.1" }))
    .route("/wasps", get(json));


    let banner = r#"
    ___       __   ________  ________  ________   
    |\  \     |\  \|\   __  \|\   ____\|\   __  \  
    \ \  \    \ \  \ \  \|\  \ \  \___|\ \  \|\  \ 
     \ \  \  __\ \  \ \   __  \ \_____  \ \   ____\
      \ \  \|\__\_\  \ \  \ \  \|____|\  \ \  \___|
       \ \____________\ \__\ \__\____\_\  \ \__\   
        \|____________|\|__|\|__|\_________\|__|   
                                \|_________|       
                                              
    "#;

    info!("{}", banner);
    info!("hey");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}


