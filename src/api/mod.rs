use log::info;

pub struct ApiServer {
    port: u16,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        info!("API Server configured on port: {}", port);
        ApiServer { port }
    }
    
    /// Start the API server
    pub async fn start(&self) {
        info!("Starting API server on port {}", self.port);
        // TODO: Implement JSON-RPC and REST endpoints
    }
}