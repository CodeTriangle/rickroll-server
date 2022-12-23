use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use warp::Filter;
use warp::http::Uri;
use warp::hyper::StatusCode;
use warp::path::FullPath;

#[tokio::main]
async fn main() {
    println!("Creating routes...");

    let counter = Arc::new(Mutex::new(HashMap::new()));

    let catchall = warp::get()
        .and(warp::path::full())
        .map(|path: FullPath| {
            println!("Unauthorized path: '{}'", path.as_str());
            warp::reply::with_status("You shouldn't be here...", StatusCode::NOT_FOUND)
        });

    let rr_counter = counter.clone();

    let rickroll = warp::get()
        .and(warp::path("rr"))
        .and(warp::path::param::<String>())
        .and(warp::addr::remote())
        .map(move |ident: String, maybe_sa: Option<SocketAddr>| {
            println!("Someone is here on path '{}'", ident);
            if let Some(sa) = maybe_sa {
                println!("Their IP is: {}", sa.ip());
            }

            let count = *rr_counter.lock().unwrap().entry(ident.as_str().to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);

            println!("That makes {count} viewers");

            warp::redirect(Uri::from_static("https://www.youtube.com/watch?v=dQw4w9WgXcQ"))
        });
    
    let static_files = warp::get()
        .and(warp::path("static"))
        .and(warp::fs::dir("static"));

    let stats = warp::get()
        .and(warp::path("stats"))
        .and(warp::fs::file("pages/stats.html"));

    let api = warp::get()
        .and(warp::path("api"));

    let sa_counter = counter.clone();

    let stats_api = api.and(warp::path("stats"))
        .map(move || {
            warp::reply::json(&*sa_counter.lock().unwrap())
        });

    println!("Routes created. Serving.");

    let routes = rickroll
        .or(static_files)
        .or(stats)
        .or(stats_api)
        .or(catchall)
        .map(|ex| warp::reply::with_header(ex, "cache-control", "max-age=0"));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
