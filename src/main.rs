#![deny(warnings)]

use futures::{FutureExt, StreamExt};
use warp::Filter;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};

#[tokio::main]
async fn main() {
    let ports: u16 = std::env::var("PORTS").unwrap_or("1".to_owned()).parse().expect("PORTS needs to be an unsigned integer");
    
    pretty_env_logger::init();

    let routes = warp::path("echo")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(|websocket| {
                // Just echo all messages back...
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });


    let mut futures = Vec::new();
    for i in 0..ports {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030 + i);
        futures.push(warp::serve(routes.clone()).bind(socket));
    }
    futures::future::join_all(futures).await;
}