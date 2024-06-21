use log4rs;
use cs_lib::domain::change_streams::change_stream;

#[tokio::main]
async fn main() {
    log4rs::init_file("C:\\Users\\cwilder\\Desktop\\dev\\TPT\\change_stream_handler\\tpt_change_stream\\log4rs.yaml", Default::default())
    .expect("Failed to initialize logger from config file");
    change_stream().await.expect("TODO: panic message");
}
