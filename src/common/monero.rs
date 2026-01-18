use rmcp::{
    ServerHandler,
    model::{ServerCapabilities, ServerInfo},
    schemars, tool,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MarblesSendRequest {
    #[schemars(description = "the left hand side number")]
    target_address: String,
    amount: i32, // XMR
}

#[derive(Debug, Clone)]
pub struct MarblesService {
    address: String,
}

#[tool(tool_box)]
impl MarblesService {
    pub fn new() -> Self {
        Self {
            address: format!("my_marbles_wallet"),
        }
    }

    #[tool(description = "Send marbles")]
    fn send_marbles(
        &self,
        #[tool(aggr)] MarblesSendRequest {
            target_address,
            amount,
        }: MarblesSendRequest,
    ) -> String {
        let message = format!(
            "Sending {} marbles from {} to {}",
            amount, self.address, target_address
        );
        tracing::info!("{}", message);
        message
    }
}

#[tool(tool_box)]
impl ServerHandler for MarblesService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("marbles thing".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
