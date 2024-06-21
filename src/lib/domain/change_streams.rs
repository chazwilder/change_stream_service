use futures::StreamExt;
use log::info;
use crate::db::mongodb_rch::get_db;
use crate::domain::mq::change_stream_to_rabbitmq;


pub async fn change_stream() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_db().await?;
    let mut change_stream = client.watch(None, None).await?;
    while let Some(event) = change_stream.next().await.transpose()? {
        info!("Event: {:?}", event);
        change_stream_to_rabbitmq(event).await?;
    }
    Ok(())
}
