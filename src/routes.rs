// HTTP Endpoint routes
use crate::database::db::DBhandler;
use crate::database::types::{ScenarioInfo, GetUrlResponse, UrlResponse, Urldata};
use actix_web::{get, post, web, HttpResponse, Result};

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
pub async fn save_url(
    data: web::Json<Urldata>,
    db: web::Data<DBhandler>,
) -> web::Json<UrlResponse> {
    println!("saveurl: {:?}", data);
    let shortid = db.saveurl(data.into_inner()).expect("Could not save to db");
    println!("Data saved!");
    println!("Short id generated: {:?}", shortid);

    // DBhandler::
    // Ok(HttpResponse::Ok().json(UrlResponse{ success: true, shortUrl: shortid}))
    web::Json(UrlResponse {
        success: true,
        shortUrl: shortid.to_owned(),
    })
}

#[get("/getUrl/{name}")]
pub async fn get_url(name: web::Path<String>, db: web::Data<DBhandler>) -> web::Json<GetUrlResponse> {
    let fluff = format!("Todo {name}!");
    println!("{:?}", fluff);

    match db.get_entry(name.to_string()) {
        Ok(out) => {
            println!("Output: {:?}", out);
            return web::Json(GetUrlResponse {
                success: true,
                longUrl: out.to_owned(),
            })
           // return HttpResponse::Ok().body("Found entry!");
        }
        Err(err) => {
            web::Json(GetUrlResponse {
                success: false,
                longUrl: "not found".to_string(),
            })        }
    };

    web::Json(GetUrlResponse {
        success: false,
        longUrl: "not found".to_string(),
    }) 
}

#[post("/xcm-asset-transfer")]
pub async fn xcm_asset_transfer() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

#[post("/scenario/info/")]
pub async fn scenario_info(data: web::Json<ScenarioInfo>) -> HttpResponse {
    // geturl

    // decode blob

    // parse scenario

    HttpResponse::Ok().body("wip")
}
