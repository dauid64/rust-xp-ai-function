use std::sync::Arc;

use rpc_router::RpcResource;


#[derive(Default, Clone, RpcResource)]
pub struct ModelManager {
    db_client: Arc<Vec<String>>,
}