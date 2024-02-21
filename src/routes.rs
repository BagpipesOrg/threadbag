// HTTP Endpoint routes
use actix_web::{get, web, post, HttpResponse};


#[get("/")]
pub async fn info() -> HttpResponse {
    HttpResponse::Ok().body("Documentation: https://xcmsend.github.io/api/index.html")
}

// open channels, list open ingoing and outgoing hrmp channels for paraid
#[post("/polkadot/openchannels")]
pub async fn dot_openchannels() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}



// broadcast input: {chain: 'hydradx', tx: ''}
#[post("/broadcast")]
pub async fn broadcast_tx() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

#[post("/saveUrl")]
pub async fn save_url() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}


#[get("/getUrl/{name}")]
pub async fn get_url(name: web::Path<String>) -> HttpResponse {
    let fluff = format!("Todo {name}!");

    HttpResponse::Ok().body("Todo!")
}

#[post("/xcm-asset-transfer")]
pub async fn xcm_asset_transfer() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}
