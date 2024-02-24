use std::net::SocketAddr;
use std::sync::{Arc};
use axum::{body};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade, Query};
use axum::response::{Response};
use futures_util::{SinkExt, StreamExt};
use hyper::StatusCode;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use crate::dto::prelude::PublicLoginDTO;
use crate::manager::websocket_manager::WebSocketManager;
use crate::message::socket_message::{MessageType, SocketMessage};
use crate::shared_state::public_router_state::{PublicRouterState};

impl PublicRouterState {

    // #[debug_handler]
    pub(crate) async fn register_public_user(
        Query(params): Query<PublicLoginDTO>,
        ws: WebSocketUpgrade,
        State(state): State<Arc<PublicRouterState>>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> Response {

        let ip = addr.to_string()
            .split(':')
            .collect::<Vec<_>>()
            .get(0).unwrap().to_string();

        let name = params.name;

        if name.len() < 3 || name.len() > 30 || !name.chars().all(char::is_alphanumeric) {

            return Response::builder().status(StatusCode::UNAUTHORIZED).body(body::boxed(body::Empty::new())).unwrap();
        }

        println!("Incoming WS connect: {}@{}", name, ip);

        let state_borrow = Arc::clone(&state);

        // Channel for linking the registration manager to the callback executed
        // upon WS upgrade that created the WS send/recv tasks
        let (mngr_send, mngr_recv) = oneshot::channel::<mpsc::Receiver<SocketMessage>>();

        let reg_msg = SocketMessage {
            messageType: MessageType::Join,
            ipaddr: ip.clone(),
            name: name.clone(),
            text: "".to_string(),
            datetime: None,
            batch: None,
            interval: None,
            return_channel: Some(mngr_send),
        };

        match state_borrow.user_recv_channel.send(reg_msg).await {

            Ok(()) => PublicRouterState::open_socket(ws, Arc::clone(&state), ip, name, mngr_recv),

            Err(e) => {

                eprintln!("Error on communicating with registration manager task:\n{}", e.to_string());
                Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(body::boxed(body::Empty::new())).unwrap()
            }
        }
    }

    fn open_socket(
        ws: WebSocketUpgrade,
        state: Arc<PublicRouterState>,
        ip: String,
        name: String,
        mngr_recv: oneshot::Receiver<mpsc::Receiver<SocketMessage>>

    ) -> Response {

        ws.on_upgrade(|socket| async move {

            // WS ends for text messages
            let (mut send, recv) = socket.split();

            match mngr_recv.await {

                Ok(tx) => {

                    // Start the task that receives messages from the new user via the websocket
                    tokio::spawn(WebSocketManager::start_websocket_recv_task(ip.clone(), name.clone(), recv, state));

                    // Start the task that sends messages to the new user via the websocket
                    tokio::spawn(WebSocketManager::start_websocket_send_task(ip, name, send, tx));
                }

                Err(_) => {
                    send.close().await.unwrap();
                    drop(recv);
                }
            }
        })
    }
}
