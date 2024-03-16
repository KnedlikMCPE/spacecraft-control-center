use std::sync::Arc;
use krpc_client::Client;

pub trait Script {
    fn run(&self, client: Arc<Client>);
}
