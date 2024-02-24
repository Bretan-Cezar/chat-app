//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "public_message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub room_id: i64,
    pub datetime: DateTime,
    #[sea_orm(column_type = "Text")]
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::public_room::Entity",
        from = "Column::RoomId",
        to = "super::public_room::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PublicRoom,
    #[sea_orm(
        belongs_to = "super::public_user::Entity",
        from = "Column::UserId",
        to = "super::public_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PublicUser,
}

impl Related<super::public_room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PublicRoom.def()
    }
}

impl Related<super::public_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PublicUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}