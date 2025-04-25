use salvo::Router;
use salvo::prelude::*;
use tokio::time::Duration;
use tower::limit::RateLimitLayer;

pub async fn init_router() -> Router {
    //一秒仅允许通过3000个请求。如果有多余的请求，则拦截
    let limit = RateLimitLayer::new(3000, Duration::from_secs(1)).compat();

    Router::new().hoop(limit)
}
