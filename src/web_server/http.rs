#![allow(dead_code)]

use crate::jobs::types::Command;
use crate::routes::{
    broadcast_tx, dot_openchannels, get_logs, get_url, info, list_single_thread, save_url,
    scenario_info, scenario_transactions, start_job, stop_job, xcm_asset_transfer,
};
use crate::DBhandler;
use crate::{cors_middleware, Loghandler, ThreadManager};
use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use std::net::TcpListener;
use std::sync::Arc;
use tokio::sync::mpsc;

// spawn the webserver on an avaliable port and return the binded address
pub fn spawn_web_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = run_webserver(listener).expect("could not run web server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

pub fn quick_server() -> Result<(Server, u16), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = run_webserver(listener).expect("could not run web server");
    Ok((server, port)) // return the server object and port
}

// Run a http server instance that we can spawn
pub fn run_webserver(listener: TcpListener) -> Result<Server, std::io::Error> {
    let (tx, _rx) = mpsc::channel::<Command>(32);
    // sled db handler
    let db_handler = DBhandler {};

    let thread_manager = Arc::new(ThreadManager::new());
    let tx3 = tx.clone();
    let tx3_clone = tx3.clone(); // Clone tx3 before moving it into the closure
    let l_db: Loghandler = Loghandler::new();

    let server = HttpServer::new(move || {
        App::new()
            // .app_data(thread_info_clone.clone()) // Pass shared data to the app
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
            .service(broadcast_tx)
            .service(dot_openchannels)
            .service(scenario_transactions) // mempool
            .service(start_job)
            .service(info)
            .service(stop_job)
            .service(list_single_thread)
    })
    .listen(listener)? //("127.0.0.1", 8081))?
    // .workers(4)
    .run();

    Ok(server)
}

/*

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

*/
//s
