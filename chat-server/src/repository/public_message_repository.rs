use std::sync::Arc;
use sea_orm::{ActiveModelTrait, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use sqlx::types::chrono::NaiveDateTime;
use crate::entity::prelude::{PublicMessage, PublicRoom, PublicUser};
use crate::entity::{public_message, public_room, public_user};
use crate::message::socket_message::{MessageType, SocketMessage};

pub struct PublicMessageRepository;

impl PublicMessageRepository {

    pub async fn add_public_message(
        db: Arc<DatabaseConnection>,
        ip: String,
        name: String,
        room_name: String,
        datetime: DateTime,
        text: String,
    ) {

        let user_id: i64 = PublicUser::find()
            .filter(public_user::Column::Ipaddr.eq(ip.clone()))
            .filter(public_user::Column::Name.eq(name.clone()))
            .filter(public_user::Column::CloseDatetime.is_null())
            .order_by_desc(public_user::Column::JoinDatetime)
            .limit(1)
            .one(db.as_ref())
            .await.expect(format!(
            "[MessageRepo] ERROR: could not find a joined user with ip={}, name={}", ip.clone(), name.clone()).as_str())
            .unwrap().id;

        let room_id: i64 = PublicRoom::find()
            .filter(public_room::Column::Name.eq(room_name.clone()))
            .one(db.as_ref())
            .await.expect(format!(
            "[MessageRepo] ERROR: could not find a public room with name={}", room_name).as_str())
            .unwrap().id;

        let message = public_message::ActiveModel {
            id: Default::default(),
            user_id: Set(user_id),
            room_id: Set(room_id),
            datetime: Set(datetime),
            text: Set(text),
        }.save(db.as_ref()).await;

        match message {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }


    pub async fn get_messages_until(
        db: Arc<DatabaseConnection>,
        room_name: String,
        no_messages: u64,
        end_datetime: NaiveDateTime
    ) -> Vec<SocketMessage> {

        let room_id: i64 = PublicRoom::find()
            .filter(public_room::Column::Name.eq(room_name.clone()))
            .one(db.as_ref())
            .await.expect(format!(
            "[MessageRepo] ERROR: could not find a public room with name={}", room_name).as_str())
            .unwrap().id;

        let messages = PublicMessage::find()
            .filter(public_message::Column::RoomId.eq(room_id))
            .filter(public_message::Column::Datetime.lt(end_datetime))
            .order_by_desc(public_message::Column::Datetime)
            .limit(no_messages)
            .find_with_related(PublicUser)
            .all(db.as_ref())
            .await.unwrap();

        messages.iter().map(|m| {
            SocketMessage {
                messageType: MessageType::Text,
                ipaddr: "".to_string(),
                name: m.1.get(0).unwrap().name.clone(),
                text: m.0.text.clone(),
                datetime: Some(m.0.datetime.clone()),
                batch: None,
                interval: None,
                return_channel: None,
            }
        }).collect()
    }


    pub async fn get_messages_within_period(
        db: Arc<DatabaseConnection>,
        room_name: String,
        start_datetime: NaiveDateTime,
        end_datetime: NaiveDateTime,
    ) -> Vec<SocketMessage> {

        let room_id: i64 = PublicRoom::find()
            .filter(public_room::Column::Name.eq(room_name.clone()))
            .one(db.as_ref())
            .await.expect(format!(
            "[MessageRepo] ERROR: could not find a public room with name={}", room_name).as_str())
            .unwrap().id;

        let messages = PublicMessage::find()
            .filter(public_message::Column::RoomId.eq(room_id))
            .filter(public_message::Column::Datetime.between(start_datetime, end_datetime))
            .find_with_related(PublicUser)
            .all(db.as_ref())
            .await.unwrap();

        messages.iter().map(|m| {
            SocketMessage {
                messageType: MessageType::Text,
                ipaddr: "".to_string(),
                name: m.1.get(0).unwrap().name.clone(),
                text: m.0.text.clone(),
                datetime: Some(m.0.datetime.clone()),
                batch: None,
                interval: None,
                return_channel: None,
            }
        }).collect()
    }
}
