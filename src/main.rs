use rmcp::{
    ErrorData as McpError,
    ServiceExt,
    handler::server::{
        ServerHandler,
        router::tool::ToolRouter,
        wrapper::Parameters,
    },
    model::{CallToolResult, Content, ListToolsResult, ServerInfo},
    schemars,
    tool, tool_router,
    transport::io::stdio,
};
use serde::Deserialize;

struct Server {
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ShowMessageArgs {
    message: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SetColorArgs {
    color: String,
}

#[tool_router] //this macro is defined in rmcp library
impl Server {
    #[tool(name = "showMessage", description = "Show a message in a screen, in a very prominent way")]
    async fn show_message(
        &self,
        Parameters(ShowMessageArgs { message }): Parameters<ShowMessageArgs>,
    ) -> Result<CallToolResult, McpError> {
        let _ = message;
        //This is the actual line that should be filled
        Ok(CallToolResult::success(vec![Content::text("not implemented")]))
    }

    #[tool(name = "setColor", description = "Set a color")]
    async fn set_color(
        &self,
        Parameters(SetColorArgs { color }): Parameters<SetColorArgs>,
    ) -> Result<CallToolResult, McpError> {
        let _ = color;
        Ok(CallToolResult::success(vec![Content::text("not implemented")]))
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: rmcp::model::Implementation::from_build_env(),
            ..Default::default()
        }
    }

    fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
        let tool_call_context = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        async move { self.tool_router.call(tool_call_context).await }
    }

    fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParam>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        async move {
            Ok(ListToolsResult {
                tools: self.tool_router.list_all(),
                next_cursor: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::default();
    let transport = stdio();

    let _running = server.serve(transport).await?;

    std::future::pending::<()>().await;
    Ok(())
}
