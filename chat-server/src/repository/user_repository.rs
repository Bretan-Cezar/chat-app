use std::sync::Arc;
use sea_orm::{ActiveModelTrait, QueryOrder, QuerySelect};
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use crate::dto::private_login_dto::PrivateLoginDTO;
use crate::entity::prelude::{PrivateUser, PublicUser};
use crate::entity::{private_user, public_user};

pub struct UserRepository;

impl UserRepository {

    pub async fn add_public_user(
        db: Arc<DatabaseConnection>,
        ip: String,
        name: String,
        datetime: DateTime,
    ) {

        let r = public_user::ActiveModel {

            id: Default::default(),
            ipaddr: Set(ip.clone()),
            name: Set(name.clone()),
            join_datetime: Set(datetime),
            close_datetime: Set(None)

        }.save(db.as_ref()).await;

        match r {
            Ok(usr) => {
                println!("[UserRepo] User id={} joined: {}@{}",
                         usr.id.unwrap(),
                         usr.name.unwrap(),
                         usr.ipaddr.unwrap());
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }

    pub async fn close_public_user(
        db: Arc<DatabaseConnection>,
        ip: String,
        name: String,
        close_datetime: DateTime,
    ) {

        let select: public_user::Model = PublicUser::find()
            .filter(public_user::Column::Ipaddr.eq(ip.clone()))
            .filter(public_user::Column::Name.eq(name.clone()))
            .filter(public_user::Column::CloseDatetime.is_null())
            .order_by_desc(public_user::Column::JoinDatetime)
            .limit(1)
            .one(db.as_ref())
            .await
            .expect("[UserRepo] database error")
            .expect(format!(
            "[UserRepo] ERROR: could not find a joined user with ip={}, name={}", ip.clone(), name.clone()).as_str());


        let mut user: public_user::ActiveModel = select.into();

        user.close_datetime = Set(Some(close_datetime));

        user.update(db.as_ref()).await
            .expect(format!("[UserRepo] ERROR: could not update close_datetime for user with ip={}, name={}", ip, name).as_str());
    }

    pub async fn find_private_user(
        db: Arc<DatabaseConnection>,
        name: String
    ) -> Option<private_user::Model> {

        PrivateUser::find()
            .filter(public_user::Column::Name.eq(name))
            .one(db.as_ref())
            .await
            .expect("[UserRepo] database error")
    }
}