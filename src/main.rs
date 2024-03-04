use crate::database::db::DBhandler;
use actix_rt::spawn;
use actix_web::{get, middleware, rt::Runtime, web, App, HttpResponse, HttpServer};
use tokio::time::Duration;
//use futures::channel::mpsc;
use tokio::sync::mpsc; // use tokio's mpsc channel

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;

//mod cli;
//use cli::print_banner;
mod chains;
pub mod database;
mod error;
mod scenarios;
mod tests;
mod tx_format;
mod jobs;

use jobs::types::{Command, ThreadInfo};
use jobs::jobs::{dummy_thread};

// get the slashes
mod routes;
use routes::{
    broadcast_tx, dot_openchannels, get_url, info, save_url, start_job, xcm_asset_transfer,
};





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //  print_banner();
    // Explicitly specify the type for the channel
    // let (_panic_sender, mut _panic_receiver): (mpsc::Sender<&str>, mpsc::Receiver<&str>) = mpsc::channel(1);
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    let db_handler = DBhandler {};
    //  let tx2 = tx.clone();
    let tx3 = tx.clone();

    let http_handle = actix_rt::spawn(async move {
        println!("Running web service on port 8081");
        HttpServer::new(move || {
            App::new()
                //   .app_data(thread_info_clone.clone()) // Pass shared data to the app
                .wrap(middleware::Compress::default())
                //   .service(status)
                //    .service(status3)
                //   .service(thread_status)
                //  .service(status2)
                .service(xcm_asset_transfer)
                .service(get_url)
                .app_data(web::Data::new(db_handler.clone()))
                .service(save_url) // Explicitly specify the handler for the route
                .service(broadcast_tx)
                .service(dot_openchannels)
                .service(start_job)
                .service(info)
        })
        .bind(("127.0.0.1", 8081))
        .expect("could not bind")
        .workers(4)
        .run()
        .await
    });

    let ready = Arc::new(tokio::sync::Mutex::new(()));
    let ready_clone = ready.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await; // Introduce a delay
        tx.send(Command::Start {
            job: "sending from second handle".to_string(),
        })
        .await;
        let _ = ready.lock().await; // Notify task_manager that it is ready
    });
    //spanning thread
    let dummy_thread_handle = tokio::spawn(async move {
        dummy_thread(tx3, ready_clone).await;
    });

    let task_manager_handle = actix_rt::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Status { job } => {
                    println!("Got the status of the job");
                }
                Stop { job } => {
                    println!("Received job stop signal");
                }
                Start { job } => {
                    let outme = format!("Starting job: {:?}", job);
                    println!("{}", outme);
                }
            }
        }
    });

    // Wait for all threads to finish

    tokio::try_join!(dummy_thread_handle, task_manager_handle)?;
    tokio::try_join!(http_handle)?;
    Ok(())
}