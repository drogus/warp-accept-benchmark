#![deny(warnings)]

use futures::{FutureExt, StreamExt};
use warp::Filter;
use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use tokio::net::TcpListener;
use net2::{unix::UnixTcpBuilderExt, TcpBuilder};

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


    let reuse_port = std::env::var("REUSE_PORT").unwrap_or("false".to_owned());
    if reuse_port == "true" {
        let mut handles = Vec::new();
        for _ in 0..ports {
            let routes_clone = routes.clone();
            let handle = tokio::task::spawn(async move {
                let tcp = TcpBuilder::new_v4().unwrap();
                let listener = tcp.reuse_address(true).unwrap()
                    .reuse_port(true).unwrap()
                    .bind("0.0.0.0:3030").unwrap()
                    .listen(10000).unwrap();
    
                    listener.set_nonblocking(true).unwrap();
    
                let mut listener = TcpListener::from_std(listener).unwrap();
                let stream = listener.incoming();
                warp::serve(routes_clone.clone()).run_incoming(stream).await
            });
    
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    } else {
        let mut futures = Vec::new();
        for i in 0..ports {
            let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3030 + i);
            futures.push(warp::serve(routes.clone()).bind(socket));
        }
        futures::future::join_all(futures).await;
    }
}