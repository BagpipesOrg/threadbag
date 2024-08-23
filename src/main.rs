use crate::database::db::{DBhandler, Loghandler};

use actix_web::{middleware, web, App, HttpServer};
//use tokio::time::Duration;
//use futures::channel::mpsc;
use actix_cors::Cors;
use tokio::sync::mpsc; // use tokio's mpsc channel
                       //use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

//mod cli;
//use cli::print_banner;
mod chains;
pub mod database;
mod error;
mod jobs;
mod scenarios;
mod tests;
mod tx_format;
mod web_server;

use jobs::jobs::{dummy_thread, start_job_worker};
use jobs::threads::ThreadManager;
use jobs::types::Command;

// get the slashes
mod routes;
use routes::{
    broadcast_tx, dot_openchannels, get_filtered_logs, get_logs, get_url, info, list_single_thread,
    save_url, scenario_info, scenario_transactions, start_job, stop_job, xcm_asset_transfer,
};

// cors settings to allow any origin
pub fn cors_middleware() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
    //     .supports_credentials()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // logger
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    //  print_banner();
    // Explicitly specify the type for the channel
    // let (_panic_sender, mut _panic_receiver): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel(1);
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    // sled db handler
    let db_handler = DBhandler {};

    // thread manager | latest
    let thread_manager = Arc::new(ThreadManager::new());
    // spawn test thread
    thread_manager.spawn("Thread 1".to_string(), async {
        sleep(Duration::from_secs(5 * 1111)).await
    });

    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    //tracing::subscriber::set_global_default(subscriber)?;

    //  let tx2 = tx.clone();
    let tx3 = tx.clone();
    let tx3_clone = tx3.clone(); // Clone tx3 before moving it into the closure
    let l_db: Loghandler = Loghandler::new();
    let http_handle = actix_rt::spawn(async move {
        println!("Running web service on port 8081");
        HttpServer::new(move || {
            App::new()
                //   .app_data(thread_info_clone.clone()) // Pass shared data to the app
                .wrap(middleware::Compress::default())
                .wrap(cors_middleware())
                .app_data(web::Data::new(tx3_clone.clone()))
                .app_data(web::Data::new(Arc::clone(&thread_manager)))
                .app_data(web::Data::new(db_handler.clone()))
                .app_data(web::Data::new(l_db.clone()))
                .service(xcm_asset_transfer)
                .service(get_url)
                .service(scenario_info)
                .service(save_url)
                .service(get_logs)
                .service(get_filtered_logs)
                .service(broadcast_tx)
                .service(dot_openchannels)
                .service(scenario_transactions) // mempool
                .service(start_job)
                .service(info)
                .service(stop_job)
                .service(list_single_thread)
        })
        .bind(("127.0.0.1", 8081))
        .expect("could not bind")
        .workers(4)
        .run()
        .await
    });

    // old remove
    let ready = Arc::new(tokio::sync::Mutex::new(()));
    let ready_clone = ready.clone();
    actix_rt::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Introduce a delay
        let _ = tx
            .send(Command::Start {
                scenario_id: "sending from second handle".to_string(),
                delay: 100u64,
            })
            .await;
        let _ = ready.lock().await; // Notify task_manager that it is ready
    });
    //spanning thread
    let dummy_thread_handle = actix_rt::spawn(async move {
        dummy_thread(tx3, ready_clone).await;
    });

    let task_manager_handle = actix_rt::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Status { scenario_id: _ } => {
                    println!("Got the status of the job");
                }
                Stop { scenario_id: _ } => {
                    println!("Received job stop signal");
                }
                Start { scenario_id, delay } => {
                    let outme = format!("Starting job: {:?}", scenario_id);
                    println!("Start job called");
                    // start_job_worker start_job_worker
                    println!("Starting worker thread");
                    let _worker_thread = actix_rt::spawn(async move {
                        let _ = match start_job_worker(scenario_id, delay).await {
                            Err(error) => println!("start job error: {:?}", error),
                            _ => {}
                        };
                    });
                    println!("worker thread");
                    println!("output: {}", outme);
                }
            }

            //    sleep(Duration::from_secs(60 * 60 * 10)).await;
        }
    });

    // Wait for all threads to finish

    tokio::try_join!(dummy_thread_handle, task_manager_handle)?;
    let _ = match tokio::try_join!(http_handle) {
        Err(_error) => {}
        _ => {}
    };
    Ok(())
}
