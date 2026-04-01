mod mcp_tools;
use non_exhaustive::non_exhaustive;
use axum::Router;
use rmcp::{
    ErrorData as McpError,
    handler::server::{ServerHandler, router::tool::ToolRouter},
    model::{CallToolResult, ListToolsResult, ServerInfo, ServerCapabilities},
    transport::{
        StreamableHttpServerConfig, StreamableHttpService,
        streamable_http_server::session::local::LocalSessionManager,
    },
};

pub(crate) struct Server {
    tool_router: ToolRouter<Self>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        non_exhaustive! { ServerInfo {
            //server_info: rmcp::model::Implementation::from_build_env(),
            //..Default::default()
            capabilities: ServerCapabilities::builder().enable_resources().build(),
            ..Default::default()
        }
        }
    }

    fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
        let tool_call_context = rmcp::handler::server::tool::ToolCallContext::new(self, request, context);
        async move { self.tool_router.call(tool_call_context).await }
    }

    fn list_tools(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<rmcp::service::RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        async move {
            Ok(ListToolsResult {
                tools: self.tool_router.list_all(),
                next_cursor: None,
                meta: None,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service: StreamableHttpService<Server, LocalSessionManager> = StreamableHttpService::new(
        || Ok(Server::default()),
        Default::default(),

        non_exhaustive! {StreamableHttpServerConfig {
            stateful_mode: true,
            sse_keep_alive: None,
        }},

    );

    let app = Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    axum::serve(listener, app).await?;
    Ok(())
}
