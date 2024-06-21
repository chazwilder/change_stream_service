use log4rs;
use cs_lib::domain::change_streams::change_stream;
use tokio;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("C:\\Users\\cwilder\\Desktop\\dev\\TPT\\change_stream_handler\\tpt_change_stream\\log4rs.yaml", Default::default())
    .expect("Failed to initialize logger from config file");
    let health_route = warp::path("heartbeat").map(|| "OK");
    let server = tokio::spawn(warp::serve(health_route).run(([0, 0, 0, 0], 3032)));

    if let Err(e) = change_stream().await {
        eprintln!("Change stream error: {:?}", e);
        std::process::exit(1);
    }
    tokio::signal::ctrl_c().await?;
    server.abort();
    println!("Shutting down");
    Ok(())
}
