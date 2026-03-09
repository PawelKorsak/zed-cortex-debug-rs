use core::time;

use zed_extension_api::{
    self as zed, DebugAdapterBinary, DebugConfig, DebugRequest, DebugScenario, DebugTaskDefinition,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, TcpArguments, Worktree,
    register_extension, serde_json,
};

use serde_json::Value;

struct ZedCortexDebugRs {}

impl zed::Extension for ZedCortexDebugRs {
    fn new() -> Self
    where
        Self: Sized,
    {
        println!("Making new instance of zed-cortex-debug-rs");
        Self {}
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        println!("Getting dap...");
        println!("Adapter Name: {adapter_name}");
        println!("Config label: {}", config.label);
        println!("Config adapter: {}", config.adapter);
        println!("TCP connection details {:?}", config.tcp_connection);
        println!(
            "User provided debug adapter path: {:?}",
            user_provided_debug_adapter_path
        );
        println!("Worktree: {:?}", worktree);

        let command: Option<String> = Some("echo".to_string());
        let arguments: Vec<String> = vec!["testing zed debugger".to_string()];

        let port = match config.tcp_connection {
            Some(conn) => conn.port.ok_or_else(|| "port not provided")?,
            None => return Err("tcp_connection not provided".to_string()),
        };
        let host = match config.tcp_connection {
            Some(conn) => conn.host.ok_or_else(|| "host not provided")?,
            None => return Err("tcp_connection not provided".to_string()),
        };
        let timeout = match config.tcp_connection {
            Some(conn) => conn.timeout,
            None => None,
        };

        let connection = Some(TcpArguments {
            port,
            host,
            timeout,
        });

        Ok(DebugAdapterBinary {
            command,
            arguments,
            envs: vec![],
            cwd: None,
            connection,
            request_args: StartDebuggingRequestArguments {
                // We just pass along the configuration
                configuration: config.config,
                request: StartDebuggingRequestArgumentsRequest::Launch,
            },
        })
    }
    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        println!("Determining DAP request...");
        Ok(StartDebuggingRequestArgumentsRequest::Attach)
    }
}

zed::register_extension!(ZedCortexDebugRs);
