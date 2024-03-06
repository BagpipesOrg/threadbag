// HTTP Endpoint routes
use crate::database::db::DBhandler;
use crate::database::decode::decompress_string;
use crate::database::types::{
    job_start, GenericOut, GetUrlResponse, ScenarioInfo, UrlResponse, Urldata,
};
use crate::jobs::threads::{ThreadManager, thread_status}; // ThreadInfo
use crate::scenarios::scenario_parse::scenario_information;
use crate::scenarios::scenario_types::Graph;
use actix_web::{get, post, web, HttpResponse, Result};
use std::sync::Arc;



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

    web::Json(UrlResponse {
        success: true,
        shortUrl: shortid.to_owned(),
    })
}

#[get("/getUrl/{name}")]
pub async fn get_url(
    name: web::Path<String>,
    db: web::Data<DBhandler>,
) -> web::Json<GetUrlResponse> {
    let fluff = format!("Todo {name}!");
    println!("{:?}", fluff);

    match db.get_entry(name.to_string()) {
        Ok(out) => {
            println!("Output: {:?}", out);
            return web::Json(GetUrlResponse {
                success: true,
                longUrl: out.to_owned(),
            });
            // return HttpResponse::Ok().body("Found entry!");
        }
        Err(err) => web::Json(GetUrlResponse {
            success: false,
            longUrl: "not found".to_string(),
        }),
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

#[post("/job/start")]
pub async fn start_job(
    data: web::Json<job_start>,
    db: web::Data<DBhandler>,
) -> web::Json<GenericOut> {
    let my_data: job_start = data.into_inner();

    return web::Json(GenericOut {
        success: true,
        result: "Job started".to_string(),
    });
}

#[post("/scenario/info/")]
pub async fn scenario_info(
    data: web::Json<ScenarioInfo>,
    db: web::Data<DBhandler>,
) -> web::Json<GenericOut> {
    println!("scenario info got input: {:?}", data);
    let name = data.into_inner().id;
    // geturl
    match db.get_entry(name.to_string()) {
        Ok(out) => {
            println!("Output: {:?}", out);

            // decode blob
            let decoded = decompress_string(out)
                .await
                .expect("Failed to decompress string, invalid value");

            // Decoded diagram data json
            let graph: Graph =
                serde_json::from_str(decoded.as_str()).expect("Failed to parse JSON");

            println!("decoded okay");
            // parse scenario
            let output_string = scenario_information(graph).expect("could not parse scenario");

            return web::Json(GenericOut {
                success: true,
                result: output_string,
            });
        }
        Err(err) => {
            return web::Json(GenericOut {
                success: false,
                result: "not found".to_string(),
            })
        }
    };

    return web::Json(GenericOut {
        success: false,
        result: "not found".to_string(),
    });
    //HttpResponse::Ok().body("wip")
}

// scenario workers
#[get("/scenario/all_workers")]
pub async fn list_all_threads(data: web::Data<Arc<ThreadManager>>) -> HttpResponse {
    let active_threads = data.get_active_threads();
    println!("listning threads!");
    HttpResponse::Ok().json(active_threads)
}

/// query single scenario worker
#[post("/scenario/worker/")]
pub async fn list_single_thread(
    postdata: web::Json<ScenarioInfo>,
    data: web::Data<Arc<ThreadManager>>,
) -> web::Json<thread_status> {

    let scenario_id = postdata.into_inner().id; 
    let thread_info = data.get_thread_status(scenario_id);

    return web::Json(thread_info);
}
