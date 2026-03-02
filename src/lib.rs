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
        let command: Option<String> = Some("echo".to_string());
        let arguments: Vec<String> = vec!["testing zed debugger".to_string()];

        let connecection = TcpArguments {
            port: 0,
            host: 0,
            timeout: None,
        };

        Ok(DebugAdapterBinary {
            command,
            arguments,
            envs: vec![],
            cwd: None,
            connection: None,
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
        Ok(StartDebuggingRequestArgumentsRequest::Attach)
    }
}

zed::register_extension!(ZedCortexDebugRs);
