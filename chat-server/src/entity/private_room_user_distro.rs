//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "private_room_user_distro")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub room_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::private_room::Entity",
        from = "Column::RoomId",
        to = "super::private_room::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PrivateRoom,
    #[sea_orm(
        belongs_to = "super::private_user::Entity",
        from = "Column::UserId",
        to = "super::private_user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PrivateUser,
}

impl Related<super::private_room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PrivateRoom.def()
    }
}

impl Related<super::private_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PrivateUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}