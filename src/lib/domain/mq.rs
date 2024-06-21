use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, ExchangeKind, BasicProperties};
use mongodb::{change_stream::event::{ChangeStreamEvent, OperationType}, bson::Document};
use serde_json::json;
use dotenvy::dotenv;
use std::env;
use log::info;

pub async fn change_stream_to_rabbitmq(event: ChangeStreamEvent<Document>) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    channel
        .exchange_declare(
            "new_order",
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                durable: true,
                ..ExchangeDeclareOptions::default()
            },
            FieldTable::default(),
        )
        .await?;

    info!("Connected to RabbitMQ, exchange declared: new_order");

    match event.operation_type {
        OperationType::Insert => {
            let event_json = serde_json::to_string(&event)?;

            channel
                .basic_publish(
                    "new_order",
                    "",
                    BasicPublishOptions::default(),
                    event_json.as_bytes(),
                    BasicProperties::default(),
                )
                .await?;

            info!("Published event to RabbitMQ exchange 'new_order': {:?}", event);
        }
        _ => {}
    }
    Ok(())
}