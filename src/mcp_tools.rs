use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars,
    tool, tool_router,
};
use serde::{Deserialize, Serialize};

use crate::Server;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ShowMessageArgs {
    message: String,
}

#[derive(Debug, Serialize)]
struct ShowBigMessageRequest {
    message: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SetColorArgs {
    color: String,
}

#[derive(Debug, Serialize)]
struct SetLedColorRequest {
    color: String,
}

#[tool_router] //this macro is defined in rmcp library
impl Server {
    pub(crate) fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(name = "showMessage", description = "Show a message in a screen, in a very prominent way. It's part of aa notification system for developers.")]
    async fn show_message(
        &self,
        Parameters(ShowMessageArgs { message }): Parameters<ShowMessageArgs>,
    ) -> Result<CallToolResult, McpError> {
        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:3000/showBigMessage")
            .json(&ShowBigMessageRequest { message })
            .send()
            .await
            .map_err(|err| McpError::internal_error(err.to_string(), None))?;

        if !response.status().is_success() {
            return Err(McpError::internal_error(
                format!("showBigMessage returned {}", response.status()),
                None,
            ));
        }

        Ok(CallToolResult::success(vec![Content::text("Ok")]))
    }

    #[tool(name = "setColor", description = "Set a color to signal state of an application. It's part of aa notification system for developers.")]
    async fn set_color(
        &self,
        Parameters(SetColorArgs { color }): Parameters<SetColorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let client = reqwest::Client::new();
        let response = client
            .post("http://localhost:3000/setLedColor")
            .json(&SetLedColorRequest { color })
            .send()
            .await
            .map_err(|err| McpError::internal_error(err.to_string(), None))?;

        if !response.status().is_success() {
            return Err(McpError::internal_error(
                format!("setLedColor returned {}", response.status()),
                None,
            ));
        }

        Ok(CallToolResult::success(vec![Content::text("Ok")]))
    }
}
