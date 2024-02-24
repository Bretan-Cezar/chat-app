use std::collections::HashMap;
use std::sync::{Arc};
use sea_orm::DatabaseConnection;
use tokio::sync::{mpsc, oneshot, RwLock};
use crate::message::socket_message::{MessageType, SocketMessage};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use crate::repository::public_message_repository::PublicMessageRepository;
use crate::repository::user_repository::UserRepository;

pub struct PublicMessageManager;

impl PublicMessageManager {

    pub async fn start_task(
        user_send_channels: Arc<RwLock<HashMap<(String, String), mpsc::Sender<SocketMessage>>>>,
        db_conn: Arc<DatabaseConnection>,
        mut text_recv: mpsc::Receiver<SocketMessage>
    ) -> ! {

        loop {
            
            match text_recv.recv().await {

                Some(msg) => {
                    
                    match msg.messageType {

                        MessageType::Text => {

                            let now = Utc::now().naive_utc();

                            // Maybe spawn this?
                            PublicMessageManager::broadcast_text(
                                &user_send_channels,
                                msg.ipaddr.clone(),
                                msg.name.clone(),
                                msg.text.clone(),
                                now.clone()
                            ).await;

                            tokio::spawn(PublicMessageRepository::add_public_message(
                                Arc::clone(&db_conn),
                                msg.ipaddr,
                                msg.name,
                                "general".to_string(),
                                now,
                                msg.text,
                            ));
                        }

                        MessageType::Close => {

                            PublicMessageManager::broadcast_close(
                                &user_send_channels,
                                msg.ipaddr.clone(),
                                msg.name.clone()
                            ).await;

                            tokio::spawn(UserRepository::close_public_user(
                                Arc::clone(&db_conn),
                                msg.ipaddr,
                                msg.name,
                                Utc::now().naive_utc(),
                            ));
                        }

                        MessageType::Join => {

                            PublicMessageManager::register(
                                &user_send_channels,
                                Arc::clone(&db_conn),
                                msg.ipaddr.clone(),
                                msg.name.clone(),
                                msg.return_channel.unwrap()
                            ).await;

                            tokio::spawn(PublicMessageManager::broadcast_join(
                                Arc::clone(&user_send_channels),
                                msg.ipaddr,
                                msg.name
                            ));
                        }

                        MessageType::IntervalBatch => {

                            let interval = msg.interval.unwrap();

                            tokio::spawn(PublicMessageManager::send_message_batch_within_period(
                                Arc::clone(&user_send_channels),
                                Arc::clone(&db_conn),
                                "general".to_string(),
                                msg.ipaddr,
                                msg.name,
                                interval.0,
                                interval.1
                            ));
                        }

                        MessageType::FixedBatch => {

                            let no_messages = 20;

                            tokio::spawn(PublicMessageManager::send_message_batch_until(
                                Arc::clone(&user_send_channels),
                                Arc::clone(&db_conn),
                                "general".to_string(),
                                msg.ipaddr,
                                msg.name,
                                no_messages,
                                msg.datetime.unwrap()
                            ));
                        }
                    }
                }
                None => {}
            }
        }
    }

    async fn broadcast_text(
        map: &RwLock<HashMap<(String, String), mpsc::Sender<SocketMessage>>>,
        msg_ip: String,
        msg_name: String,
        msg_text: String,
        msg_datetime: NaiveDateTime
    ) {

        let map = map.read().await;

        for ((ip, name), dest) in map.iter() {

            if ip.clone() != msg_ip || name.clone() != msg_name {

                dest.send(SocketMessage {
                    messageType: MessageType::Text,
                    ipaddr: "".to_string(),
                    name: msg_name.clone(),
                    text: msg_text.clone(),
                    datetime: Some(msg_datetime),
                    //timestamp: Some(msg_datetime.timestamp_millis()),
                    batch: None,
                    interval: None,
                    return_channel: None,
                })
                    .await
                    .expect(format!("[PublicMessageManager] ERROR: could not send Text message to WS send task, ip {}", ip).as_str());
            }
        }
    }

    async fn broadcast_close(
        map: &RwLock<HashMap<(String, String), mpsc::Sender<SocketMessage>>>,
        msg_ip: String,
        msg_name: String
    ) {

        let mut map = map.write().await;

        let key = (msg_ip.clone(), msg_name.clone());

        for ((ip, _), dest) in map.iter() {

            dest.send(SocketMessage {
                messageType: MessageType::Close,
                ipaddr: msg_ip.clone(),
                name: msg_name.clone(),
                text: "".to_string(),
                datetime: None,
                //timestamp: None,
                batch: None,
                interval: None,
                return_channel: None,
            })
                .await
                .expect(format!("[PublicMessageManager] ERROR: could not send Closed message to WS send task, ip {}", ip).as_str());
        }

        map.remove(&key);
    }


    async fn register(
        map: &RwLock<HashMap<(String, String), mpsc::Sender<SocketMessage>>>,
        db_conn: Arc<DatabaseConnection>,
        ip: String,
        name: String,
        return_channel: oneshot::Sender<mpsc::Receiver<SocketMessage>>
    ) {

        let mut map = map.write().await;

        let key = (ip.clone(), name.clone());

        // If the user is not the map, it's not registered
        if !map.contains_key(&key) {

            // Channel for the TextMessageManager task to send TextMessageDTOs to a WS send task
            let (sx, tx) = mpsc::channel::<SocketMessage>(1);

            // Insert the registered user along with the send end of the channel in the map
            map.insert(key, sx);

            // Insert the newly registered user into the DB
            tokio::spawn(UserRepository::add_public_user(db_conn, ip, name, Utc::now().naive_utc()));

            match return_channel.send(tx) {
                Ok(()) => {}
                Err(_) => {
                    eprintln!("[PublicMessageManager] ERROR: could not send Receiver end");
                }
            }
        }
        else {
            eprintln!("[PublicMessageManager] ERROR: Conflict on registration")
        }
    }


    async fn broadcast_join(
        map: Arc<RwLock<HashMap<(String, String), mpsc::Sender<SocketMessage>>>>,
        msg_ip: String,
        msg_name: String,
    ) {

        let map = map.read().await;

        for ((ip, name), dest) in map.iter() {

            if ip.clone() != msg_ip || name.clone() != msg_name {

                dest.send(SocketMessage {
                    messageType: MessageType::Join,
                    ipaddr: "".to_string(),
                    name: msg_name.clone(),
                    text: "".to_string(),
                    datetime: None,
                    //timestamp: None,
                    batch: None,
                    interval: None,
                    return_channel: None,
                })
                    .await
                    .expect(format!("[PublicMessageManager] ERROR: could not send Join message to WS send task, ip {}", ip).as_str());
            }
        }
    }

    async fn send_message_batch_within_period(
        map: Arc<RwLock<HashMap<(String, String), mpsc::Sender<SocketMessage>>>>,
        db_conn: Arc<DatabaseConnection>,
        room_name: String,
        target_ip: String,
        target_name: String,
        start_datetime: NaiveDateTime,
        end_datetime: NaiveDateTime
    ) {

        let map = map.read().await;

        let target = map.get(&(target_ip, target_name)).unwrap();

        let batch = PublicMessageRepository::get_messages_within_period(
            db_conn,
            room_name,
            start_datetime,
            end_datetime,
        ).await;

        let r = target.send(SocketMessage {
            messageType: MessageType::IntervalBatch,
            ipaddr: "".to_string(),
            name: "".to_string(),
            text: "".to_string(),
            datetime: None,
            //timestamp: None,
            batch: Some(batch),
            interval: Some((start_datetime, end_datetime)),
            return_channel: None,
        }).await;

        match r {
            Ok(()) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }

    async fn send_message_batch_until(
        map: Arc<RwLock<HashMap<(String, String), mpsc::Sender<SocketMessage>>>>,
        db_conn: Arc<DatabaseConnection>,
        room_name: String,
        target_ip: String,
        target_name: String,
        no_messages: u64,
        end_datetime: NaiveDateTime,
    ) {
        let map = map.read().await;

        let target = map.get(&(target_ip, target_name)).unwrap();

        let batch = PublicMessageRepository::get_messages_until(
            db_conn,
            room_name,
            no_messages,
            end_datetime,
        ).await;

        let r = target.send(SocketMessage {
            messageType: MessageType::FixedBatch,
            ipaddr: "".to_string(),
            name: "".to_string(),
            text: "".to_string(),
            datetime: None,
            //timestamp: None,
            batch: Some(batch),
            interval: None,
            return_channel: None,
        }).await;

        match r {
            Ok(()) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }

    }
}
