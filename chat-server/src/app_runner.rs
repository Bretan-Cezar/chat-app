use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc};
use std::time::Duration;
use axum::error_handling::HandleErrorLayer;
use axum::{BoxError, Router};
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::get;
use tokio::sync::{mpsc, RwLock};
use sea_orm::Database;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use crate::config::Config;
use crate::manager::public_message_manager::PublicMessageManager;
use crate::message::socket_message::SocketMessage;
use crate::shared_state::public_router_state::PublicRouterState;

pub struct AppRunner;

impl AppRunner {

    pub async fn run(config: Config) {

        let db_conn = Arc::new(Database::connect(config.database_url)
            .await
            .expect("Error on DB connection"));


        // Channel for the WS recv tasks to send TextMessageDTOs to the TextMessageManager task
        let (mp_sx, mp_rx) = mpsc::channel::<SocketMessage>(5120);


        let router_state = Arc::new(PublicRouterState {
            user_recv_channel: Arc::new(mp_sx),
            ws_tick: Duration::from_millis(config.ws_tick),
        });


        // Start text message manager task
        tokio::spawn(PublicMessageManager::start_task(
            Arc::new(RwLock::new(HashMap::new())),
            db_conn,
            mp_rx)
        );


        let public_router = Router::new()
            .route("/login-public", get(PublicRouterState::register_public_user))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(|err: BoxError| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled error: {}", err),
                        )
                    }))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(1, Duration::from_millis(config.reg_tick)))
            )
            .with_state(Arc::clone(&router_state))
            .into_make_service_with_connect_info::<SocketAddr>();

        let private_router = Router::new()
            .route("/login-private")
            .layer();


        let app = Router::new()
            .merge(public_router);


        axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
            .serve(app)
            .await
            .unwrap();
    }
}