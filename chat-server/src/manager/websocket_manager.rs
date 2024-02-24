use std::sync::{Arc};
use tokio::sync::mpsc;
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use crate::message::socket_message::{MessageType, SocketMessage};
use crate::shared_state::public_router_state::{PublicRouterState};
use serde_json;
use chrono::naive::NaiveDateTime;

pub struct WebSocketManager;

impl WebSocketManager {

    pub async fn start_websocket_send_task(
        ip: String,
        name: String,
        mut ws: SplitSink<WebSocket, Message>,
        mut msg_recv: mpsc::Receiver<SocketMessage>
    ) {

        loop {

            match msg_recv.recv().await {

                None => {}

                Some(msg) => {

                    match msg.messageType {

                        MessageType::Text | MessageType::Join | MessageType::IntervalBatch | MessageType::FixedBatch => {
                            WebSocketManager::serialize_and_send_json_to_ws(msg, &mut ws).await;
                        }

                        MessageType::Close => {

                            if msg.ipaddr == ip && msg.name == name {
                                println!("[{}@{}] WS closed. Closing send task...", msg.name, msg.ipaddr);
                                break;
                            }
                            else {
                                WebSocketManager::serialize_and_send_json_to_ws(msg, &mut ws).await;
                            }
                        }
                    }
                }
            }
        }
    }

    async fn serialize_and_send_json_to_ws(msg: SocketMessage, ws: &mut SplitSink<WebSocket, Message>) {

        let json = serde_json::to_string(&msg)
            .expect(format!("Error on {} message serialization", &msg.messageType).as_str());

        match ws.send(Message::from(json)).await {
            Ok(()) => {}

            Err(e) => {
                eprintln!("Error on WS {} message send: {:?}", msg.messageType, e);
            }
        }
    }

    pub async fn start_websocket_recv_task(
        ip: String,
        name: String,
        mut ws: SplitStream<WebSocket>,
        state: Arc<PublicRouterState>
    ) {

        let msg_mngr_sink = state.user_recv_channel.clone();

        loop {

            match ws.next().await {

                None => {}

                Some(Ok(Message::Text(text))) => {

                    if text.len() <= 131072 {

                        let msg = SocketMessage {
                            ipaddr: ip.clone(),
                            name: name.clone(),
                            text,
                            messageType: MessageType::Text,
                            datetime: None,
                            batch: None,
                            interval: None,
                            return_channel: None,
                        };

                        match msg_mngr_sink.send(msg).await {
                            Ok(()) => {
                                tokio::time::sleep(state.ws_tick).await;
                            }
                            Err(err) => {
                                eprintln!("Error on sending to message manager:\n{:?}", err.0);
                                break;
                            }
                        };
                    }
                    else {
                        tokio::time::sleep(state.ws_tick).await;
                    }
                }

                Some(Ok(Message::Close(_))) => {

                    let msg = SocketMessage {
                        ipaddr: ip.clone(),
                        name: name.clone(),
                        text: String::new(),
                        messageType: MessageType::Close,
                        datetime: None,
                        batch: None,
                        interval: None,
                        return_channel: None,
                    };

                    match msg_mngr_sink.send(msg).await {
                        Ok(()) => {}
                        Err(err) => {
                            eprintln!("Error on sending to message manager:\n{:?}", err.0);
                            break;
                        }
                    };

                    println!("[{}] WS closed. Closing recv task...", ip);
                    break;
                }

                Some(Ok(Message::Binary(bin))) => {

                    if bin.len() == 17 && bin.get(0).unwrap().eq(&255) {

                        let mut start_arr = [0u8; 8];
                        start_arr.copy_from_slice(&bin[1..=8]);

                        let mut end_arr = [0u8; 8];
                        end_arr.copy_from_slice(&bin[9..]);

                        let start_ts = i64::from_be_bytes(start_arr);
                        let end_ts = i64::from_be_bytes(end_arr);

                        if start_ts < end_ts && start_ts >= 0 && end_ts >= 0 {

                            if end_ts - start_ts <= 86400 {

                                let start_datetime = NaiveDateTime::from_timestamp_opt(start_ts, 0).unwrap();
                                let end_datetime = NaiveDateTime::from_timestamp_opt(end_ts, 0).unwrap();

                                let msg = SocketMessage {
                                    messageType: MessageType::IntervalBatch,
                                    ipaddr: ip.clone(),
                                    name: name.clone(),
                                    text: "".to_string(),
                                    datetime: None,
                                    batch: None,
                                    interval: Some((start_datetime, end_datetime)),
                                    return_channel: None,
                                };

                                match msg_mngr_sink.send(msg).await {
                                    Ok(()) => {}
                                    Err(err) => {
                                        eprintln!("Error on sending to message manager: {:?}", err);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    else if bin.len() == 9 && bin.get(0).unwrap().eq(&127) {

                        let mut end_arr = [0u8; 8];
                        end_arr.copy_from_slice(&bin[1..=8]);

                        let end_ts = i64::from_be_bytes(end_arr);

                        if end_ts >= 0 {

                            let end_datetime = NaiveDateTime::from_timestamp_millis(end_ts).unwrap();

                            println!("{:?}", end_datetime);

                            let msg = SocketMessage {

                                messageType: MessageType::FixedBatch,
                                ipaddr: ip.clone(),
                                name: name.clone(),
                                text: "".to_string(),
                                datetime: Some(end_datetime),
                                batch: None,
                                interval: None,
                                return_channel: None,
                            };

                            match msg_mngr_sink.send(msg).await {
                                Ok(()) => {}
                                Err(err) => {
                                    eprintln!("Error on sending to message manager: {:?}", err);
                                    break;
                                }
                            }
                        }
                    }

                    tokio::time::sleep(state.ws_tick).await;
                }

                Some(Ok(Message::Ping(_ping))) => {
                    unimplemented!()
                }

                Some(Ok(Message::Pong(_pong))) => {
                    unimplemented!()
                }

                Some(Err(e)) => {

                    eprintln!("Error on WS message recv: {:?}", e);
                    break;
                }
            }
        }
    }
}