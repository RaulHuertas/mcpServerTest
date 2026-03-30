use axum::Router;
use rmcp::{
    ErrorData as McpError,
    handler::server::{
        ServerHandler,
        router::tool::ToolRouter,
        wrapper::Parameters,
    },
    model::{CallToolResult, Content, ListToolsResult, ServerInfo},
    schemars,
    tool, tool_router,
    transport::{
        StreamableHttpServerConfig, StreamableHttpService,
        streamable_http_server::session::local::LocalSessionManager,
    },
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
    #[tool(name = "showMessage", description = "Show a message in a screen, in a very prominent way. It's part of aa notification system for developers.")]
    async fn show_message(
        &self,
        Parameters(ShowMessageArgs { message }): Parameters<ShowMessageArgs>,
    ) -> Result<CallToolResult, McpError> {
        //let _ = message;
        //This is the actual line that should be filled
        println!("YOU ARE DISPLAYING A MESSAGE  {}", message);
        Ok(CallToolResult::success(vec![Content::text("Ok")]))
    }

    #[tool(name = "setColor", description = "Set a color to signal state of an application. It's part of aa notification system for developers.")]
    async fn set_color(
        &self,
        Parameters(SetColorArgs { color }): Parameters<SetColorArgs>,
    ) -> Result<CallToolResult, McpError> {
        //let _ = color;
        println!("YOU ARE DISPLAYING A COLOR  {}", color);
        Ok(CallToolResult::success(vec![Content::text("Ok")]))

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
    let service: StreamableHttpService<Server, LocalSessionManager> = StreamableHttpService::new(
        || Ok(Server::default()),
        Default::default(),
        StreamableHttpServerConfig {
            stateful_mode: true,
            sse_keep_alive: None,
        },
    );

    let app = Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
