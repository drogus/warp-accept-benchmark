use std::{time::{Duration, Instant}, sync::Arc};
use tokio::sync::RwLock;
use tokio_tungstenite::connect_async;
use clap::{Arg, App};

#[tokio::main]
async fn main() {
    let matches = App::new("Stress test")
    .arg(Arg::with_name("ports")
        .short("p")
        .long("ports")
        .value_name("PORTS")
        .help("Sets the number of ports to use")
        .takes_value(true)
        .required(false))
    .get_matches();

    let ports: u32 = matches.value_of("ports").unwrap_or("1").parse().expect("PORTS needs to be an unsigned integer");

    let counter = Arc::new(RwLock::new(0usize));

    let count: u32 = 50000;
    for i in 0..count {
        let counter_clone = counter.clone();
        tokio::spawn(async move {
            let port: u32 = 3030 + (i % ports);

            match connect_async(&format!("ws://127.0.0.1:{}/echo", port)).await {
                Ok((mut ws_stream, _)) => {
                    {
                        *counter_clone.write().await += 1;
                    }

                    // keep the connection open
                    tokio::time::delay_for(Duration::from_secs(1000)).await;
                },
                Err(e) => {
                    println!("error when connecting: {:?}", e);
                }
            }
        });
    }

    let now = Instant::now();

    loop {
        let locked = counter.read().await;
        println!("connections: {:?}, elapsed: {}ms", *locked, now.elapsed().as_millis());
        drop(locked);
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}