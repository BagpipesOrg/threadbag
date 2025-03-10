#![allow(unreachable_code)]
// HTTP Endpoint routes
use crate::database::db::{string_to_logtype, DBhandle, Loghandler}; //DBhandler
use crate::database::decode::decompress_string;
use crate::database::types::{
    BroadcastInput, BroadcastStatus, GenericOut, GetUrlResponse, LogsOut, ScenarioInfo,
    ScenarioInfoOut, ScenarioLog, TxInfo, TxQueue, UrlResponse, Urldata,
};
use crate::jobs::threads::{ThreadManager, ThreadStatus}; // ThreadInfo
use crate::scenarios::scenario_parse::{
    multi_scenario_info, scenario_information, verify_scenario_id,
};
use crate::scenarios::scenario_types::{Graph, ScenarioSummary};

use crate::Command;
use actix_web::{get, post, web, HttpResponse};
use std::sync::Arc;

#[get("/")]
pub async fn info() -> HttpResponse {
    HttpResponse::Ok().body("Threadbag Documentation: https://docs.bagpipes.io/docs/api/threadbag")
}

// open channels, list open ingoing and outgoing hrmp channels for paraid
#[post("/polkadot/openchannels")]
pub async fn dot_openchannels() -> HttpResponse {
    HttpResponse::Ok().body("Todo!")
}

#[get("/version")]
pub async fn get_version() -> HttpResponse {
    let version = env!("CARGO_PKG_VERSION");
    HttpResponse::Ok().body(version)
}

// broadcast input: {chain: 'hydradx', tx: ''}
#[post("/broadcast")]
pub async fn broadcast_tx(_data: web::Json<BroadcastInput>) -> web::Json<BroadcastStatus> {
    web::Json(BroadcastStatus {
        status: "fail".to_string(),
        hash: "not found".to_string(),
    })
}

#[post("/saveUrl")]
pub async fn save_url(data: web::Json<Urldata>, db: web::Data<DBhandle>) -> web::Json<UrlResponse> {
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
    db: web::Data<DBhandle>,
) -> web::Json<GetUrlResponse> {
    let fluff = format!("Todo {name}!");
    println!("{:?}", fluff);

    match db.get_entry(name.to_string()) {
        Ok(out) => {
            let url = out.longurl;
            //       println!("Output: {:?}", out);
            return web::Json(GetUrlResponse {
                success: true,
                longUrl: url,
            });
            // return HttpResponse::Ok().body("Found entry!");
        }
        Err(_err) => web::Json(GetUrlResponse {
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

/// curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWg"}' http://localhost:8081/job/stop -v
/// wanted input:  {         
///    pub id: String,        
///}        
/// stop
#[post("/job/stop")]
pub async fn stop_job(
    data: web::Json<ScenarioInfo>, // JobStart
    datan: web::Data<Arc<ThreadManager>>,
    _tx: web::Data<tokio::sync::mpsc::Sender<Command>>,
    _db: web::Data<DBhandle>,
) -> web::Json<GenericOut> {
    println!("JobStart called");
    let my_data: ScenarioInfo = data.into_inner();
    println!("data collected");
    let scenario_id = my_data.id;
    println!("start_job id: {:?}", scenario_id);
    // let my_delay = my_data.delay;
    // validate input

    let _thread_info = datan.stop_thread(&scenario_id);

    // send job start command
    println!("route job sending start command");

    web::Json(GenericOut {
        success: true,
        result: "Job stopped".to_string(),
    })
}

/// create a webhook that listens for data
#[post("/webhook/create")]
pub async fn creates_webhook() -> web::Json<GenericOut> {
    //todo!("create webhook and return endpoint ")
    web::Json(GenericOut {
        success: false,
        result: "webhooks is still todo".to_string(),
    })
}

/// subscribe to webhook
#[get("/webhook/sub")]
pub async fn sub_webhook() -> web::Json<GenericOut> {
    //  todo!("subscribe  webhook")
    web::Json(GenericOut {
        success: false,
        result: "webhooks is still todo".to_string(),
    })
}

/// curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWg"}' http://localhost:8081/job/start -v
/// wanted input:  {         
///    pub scenario_id: String,        
///    pub delay: u64,        
///}        
/// start a scenario worker
#[post("/job/start")]
pub async fn start_job(
    data: web::Json<ScenarioInfo>, // job_start
    tx: web::Data<tokio::sync::mpsc::Sender<Command>>,
    _db: web::Data<DBhandle>,
) -> web::Json<GenericOut> {
    println!("JobStart called");
    let my_data: ScenarioInfo = data.into_inner();
    println!("data collected");
    let scenario_id = my_data.id;
    println!("start_job id: {:?}", scenario_id);
    match verify_scenario_id(scenario_id.clone()) {
        true => {}
        _ => {
            return web::Json(GenericOut {
                success: false,
                result: "Invalid scenario id".to_string(),
            });
        }
    };
    // let my_delay = my_data.delay;
    // validate input

    // send job start command
    println!("route job sending start command");

    if let Err(_error) = tx
        .send(Command::Start {
            //job: "sending from second handle".to_string(),
            scenario_id,
            delay: 12u64, // 12 hours
        })
        .await
    {
        return web::Json(GenericOut {
            success: false,
            result: "could not start job".to_string(),
        });
    };

    web::Json(GenericOut {
        success: true,
        result: "Job started".to_string(),
    })
}

/// curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWg"}' http://localhost:8081/job/start -v
/// Threadbags mempool
#[post("/scenario/tx")]
pub async fn scenario_transactions(
    data: web::Json<ScenarioInfo>,
    db: web::Data<Loghandler>,
) -> web::Json<TxQueue> {
    let scenario_id = data.into_inner().id;

    let output: Vec<TxInfo> = match db.into_inner().get_transactions(scenario_id) {
        Ok(value) => value.into_iter().collect(),
        _ => Vec::new(),
    };

    web::Json(TxQueue { mempool: output })
}

/*
curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWvg"}' http://localhost:8081/scenario/info -v
{"success":true,"result":[{"source_chain":"polkadot","source_address":"5GdvmQtUwByTt6Vkx41vtWvg5guyaH3BL2yn6iamg1RViiKD","dest_chain":"assetHub","dest_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","assetid":"0","amount":"1","txtype":"swap","tx":"not set"},{"source_chain":"assetHub","source_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","dest_chain":"hydraDx","dest_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","assetid":"3","amount":"2","txtype":"swap","tx":"not set"},{"source_chain":"hydraDx","source_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","dest_chain":"hydraDx","dest_address":"5D7RT7vqgZKUoKxrPMihNeXBzhrmWjd5meprfUFhtrULJ4ng","assetid":"5","amount":"2","txtype":"swap","tx":"not set"}]}
*/

#[post("/scenario/info")]
pub async fn scenario_info(
    data: web::Json<ScenarioInfo>,
    db: web::Data<DBhandle>,
) -> web::Json<ScenarioInfoOut> {
    println!("scenario info got input: {:?}", data);
    let name = data.into_inner().id;
    // geturl
    match db.get_entry(name.to_string()) {
        Ok(out) => {
            println!("Output: {:?}", out);

            // decode blob
            let decoded = decompress_string(out.longurl)
                .await
                .expect("Failed to decompress string, invalid value");
            println!("decoded ok");
            println!("Decoded as: {}", decoded);
            // Decoded diagram data json
            let graph: Graph =
                serde_json::from_str(decoded.as_str()).expect("Failed to parse JSON");

            println!("decoded okay");
            // parse scenario
            println!("parsing scenario_information");
            let _output_string =
                scenario_information(graph.clone()).expect("could not parse scenario");
            println!("parsing scenario_information ok");
            println!("parsing multi_scenario_info");
            let o2: Vec<ScenarioSummary> = multi_scenario_info(graph.clone());
            println!("parsing multi_scenario_info ok");
            println!("parsing multi_scenario_info: {:?}", o2);
            return web::Json(ScenarioInfoOut {
                success: true,
                result: Some(o2),
            });
        }
        Err(_err) => {
            return web::Json(ScenarioInfoOut {
                success: false,
                result: None,
            })
        }
    };

    web::Json(ScenarioInfoOut {
        success: false,
        result: None,
        // result: Vec::new(),
    })
    //HttpResponse::Ok().body("wip")
}

// scenario workers
#[get("/scenario/all_workers")]
pub async fn list_all_threads(data: web::Data<Arc<ThreadManager>>) -> HttpResponse {
    let active_threads = data.get_active_threads();
    println!("listning threads!");
    HttpResponse::Ok().json(active_threads)
}

// curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWvg"}' http://localhost:8081/scenario/info -v
/// query single scenario worker
#[post("/scenario/worker/")]
pub async fn list_single_thread(
    postdata: web::Json<ScenarioInfo>,
    data: web::Data<Arc<ThreadManager>>,
) -> web::Json<ThreadStatus> {
    let scenario_id = postdata.into_inner().id;
    let thread_info = data.get_thread_status(scenario_id);

    web::Json(thread_info)
}

/// curl -X POST -H "Content-Type: application/json" -d '{"id": "LSm-41cJY", "log_type": "query"}' http://localhost:8081/scenario/get_filter_logs
#[post("/scenario/get_filter_logs")]
pub async fn get_filtered_logs(
    postdata: web::Json<ScenarioLog>,
    l_db: web::Data<Loghandler>,
) -> web::Json<LogsOut> {
    let inne = postdata.into_inner();
    let scenario_id = inne.clone().id;
    let log_type_string = inne.log_type;

    let log_type = match string_to_logtype(&log_type_string) {
        Some(val) => val,
        _ => {
            return web::Json(LogsOut {
                success: false,
                result: Vec::new(),
            })
        }
    };
    let output: Vec<String> = match l_db.into_inner().get_log_entries(scenario_id, log_type) {
        Ok(value) => value.into_iter().map(|entry| entry.msg).collect(),
        _ => Vec::new(),
    };
    web::Json(LogsOut {
        success: true,
        result: output, // can get the dates as well if want to
    })
}

// curl -X POST -H "Content-Type: application/json" -d '{"id": "H!Xz6LWvg"}' http://localhost:8081/scenario/worker/logs -v
/// get execution logs | get the history of the executed scenario
/// Returns a list of logs for the entry in a Vec<String>
#[post("/scenario/worker/logs")]
pub async fn get_logs(
    postdata: web::Json<ScenarioInfo>,
    l_db: web::Data<Loghandler>,
) -> web::Json<LogsOut> {
    println!("displaying logs");
    println!("quering logs");
    // todo validate scenario id
    let scenario_id = postdata.into_inner().id;

    let output: Vec<String> = match l_db.into_inner().get_entry(scenario_id) {
        Ok(value) => value.into_iter().map(|entry| entry.msg).collect(),
        _ => Vec::new(),
    };
    println!("returning query logs");
    web::Json(LogsOut {
        success: true,
        result: output, // can get the dates as well if want to
    })
}

// test a http action
#[post("/action/http/dry_run")]
pub async fn dry_run_http() -> web::Json<GenericOut> {
    web::Json(GenericOut {
        success: false,
        result: "not found".to_string(),
    })
}
