use std::sync::Arc;

use crate::{
    models::server::Pool,
    routes::{concept::concepts_router, user::users_router, auth::auth_router},
    services::{response::handle_rejection, server::serve_start},
    utils::port::port,
};
use warp::Filter;

pub async fn app(pool: &Arc<Pool>) {
    let port = port().unwrap();
    let started = warp::path("api")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(serve_start);
    // let fallback = warp::any().map(|| "Ninguna pagina!");
    // let download_route = warp::path("files").and(warp::fs::dir("./files/"));
    let apis = started
        .or(concepts_router(pool))
        .or(users_router(pool))
        .or(auth_router(pool))
        .recover(handle_rejection);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type","authorization"])
        .allow_methods(vec!["POST","GET","PUT","DELETE"]);

        let routes = apis.with(cors);

    println!("Starting server on port: {}", port);
    
    let (_addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], port), async {
            tokio::signal::ctrl_c()
                .await
                .expect("Http server: Failed to listen to CTRL+C");
            println!("Shutting down HTTP Server!");
        });

    println!("Server listening => {}",_addr);
        
    server.await;
}
