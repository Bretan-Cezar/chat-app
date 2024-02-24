use std::fmt::{Display};
use std::sync::{Arc};
use std::time::Duration;
use tokio::sync::mpsc;
use crate::message::socket_message::SocketMessage;

pub struct PublicRouterState {
    pub(crate) user_recv_channel: Arc<mpsc::Sender<SocketMessage>>,
    pub(crate) ws_tick: Duration,
}

