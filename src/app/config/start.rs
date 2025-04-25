use crate::app::config::router::init_router;
use crate::shared::config::nacos::init_nacos_service;
use crate::shared::config::redis::init_redis_conn;
use crate::shared::config::sea_orm::init_db_conn;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::conn::{QuinnListener, TcpListener};
use salvo::logging::Logger;
use salvo::prelude::*;
use salvo::{Listener, Router, Server};
#[inline]
#[allow(dead_code)]
async fn use_http1(router: Router) {
    let service = Service::new(router).hoop(Logger::new());
    let acceptor = TcpListener::new("0.0.0.0:9001").bind().await;
    tracing::info!("Listening on http://0.0.0.0:9001");
    Server::new(acceptor).serve(service).await;
}

#[inline]
#[allow(dead_code)]
async fn use_http3(router: Router) {
    let cert = include_bytes!("../../../cert/cert.pem").to_vec();
    let key = include_bytes!("../../../cert/key.pem").to_vec();
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    let listener = TcpListener::new(("0.0.0.0", 9001)).rustls(config.clone());
    let acceptor = QuinnListener::new(config, "0.0.0.0:9001")
        .join(listener)
        .bind()
        .await;
    tracing::info!("Listening on https://0.0.0.0:9004");
    Server::new(acceptor).serve(router).await;
}

#[inline]
pub async fn salvo_application_start() {
    tracing_subscriber::fmt().init();
    init_nacos_service().await;
    init_db_conn().await;
    init_redis_conn().await;
    // router
    let router = init_router().await;
    use_http1(router).await;
}
