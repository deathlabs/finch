use std::io::Read;
use std::net::Ipv4Addr;

use chrono::{SecondsFormat, Utc};
use serde_json::{Value, json};
use tiny_http::{Header, Method, Request, Response, ResponseBox, Server};

use crate::schema;

const PROTOCOL_VERSION: &str = "2025-11-25";
const SCHEMA_URI: &str = "finch://schema/mappings";
const SCHEMA_MIME_TYPE: &str = "application/schema+json";

const MAX_BODY_BYTES: u64 = 64 * 1024;

pub(crate) fn run(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server: Server = Server::http((Ipv4Addr::LOCALHOST, port)).inspect_err(|error| {
        log(json!({
            "level": "error",
            "event": "mcp_start_failed",
            "port": port,
            "error": error.to_string(),
        }));
    })?;
    let address: std::net::SocketAddr = server.server_addr().to_ip().expect("TCP listener");
    log(json!({
        "level": "info",
        "event": "mcp_listening",
        "url": format!("http://{address}/mcp"),
    }));
    for mut request in server.incoming_requests() {
        let mut record: Value = json!({
            "level": "info",
            "event": "mcp_request",
            "http_method": request.method().as_str(),
            "url": request.url(),
        });
        let response: ResponseBox = build_response(&mut request, address.port(), &mut record);
        record["http_status"] = json!(response.status_code().0);
        if let Err(error) = request.respond(response) {
            record["level"] = json!("error");
            record["send_error"] = json!(error.to_string());
        }
        log(record);
    }
    return Ok(());
}

fn log(mut record: Value) {
    record["timestamp"] = json!(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true));
    eprintln!("{record}");
    return;
}

fn build_response(request: &mut Request, port: u16, record: &mut Value) -> ResponseBox {
    let loopback_host: String = format!("127.0.0.1:{port}");
    let localhost_host: String = format!("localhost:{port}");
    let loopback_origin: String = format!("http://{loopback_host}");
    let localhost_origin: String = format!("http://{localhost_host}");
    for header in request.headers() {
        let value: &str = header.value.as_str();
        if (header.field.equiv("Host") && value != loopback_host && value != localhost_host)
            || (header.field.equiv("Origin")
                && value != loopback_origin
                && value != localhost_origin)
        {
            return Response::empty(403).boxed();
        }
        if header.field.equiv("MCP-Protocol-Version") && !is_supported_version(value) {
            return Response::empty(400).boxed();
        }
    }
    if request.url() != "/mcp" {
        return Response::empty(404).boxed();
    }
    if request.method() != &Method::Post {
        return Response::empty(405)
            .with_header(header("Allow", "POST"))
            .boxed();
    }
    if !request.headers().iter().any(|header| {
        return header.field.equiv("Content-Type")
            && header
                .value
                .as_str()
                .split(';')
                .next()
                .is_some_and(|media_type| {
                    return media_type.trim().eq_ignore_ascii_case("application/json");
                });
    }) {
        return Response::empty(415).boxed();
    }

    let mut body: Vec<u8> = Vec::new();
    if request
        .as_reader()
        .take(MAX_BODY_BYTES + 1)
        .read_to_end(&mut body)
        .is_err()
    {
        return Response::empty(400).boxed();
    }
    if body.len() as u64 > MAX_BODY_BYTES {
        return Response::empty(413).boxed();
    }
    let response: Option<Value> = match serde_json::from_slice(&body) {
        Ok(message) => {
            let response: Option<Value> = handle(&message);
            record["request"] = message;
            response
        }
        Err(_) => Some(error(&Value::Null, -32700, "Expected a JSON message.")),
    };
    if let Some(response) = response {
        if let Some(code) = response.pointer("/error/code") {
            record["rpc_error_code"] = code.clone();
        }
        return Response::from_string(response.to_string())
            .with_header(header("Content-Type", "application/json"))
            .boxed();
    }
    return Response::empty(202).boxed();
}

fn header(name: &str, value: &str) -> Header {
    return Header::from_bytes(name, value).expect("valid static HTTP header");
}

fn is_supported_version(version: &str) -> bool {
    return matches!(version, PROTOCOL_VERSION | "2025-06-18" | "2025-03-26");
}

fn handle(message: &Value) -> Option<Value> {
    let id: Option<&Value> = message.get("id");
    let method: Option<&str> = message.get("method").and_then(Value::as_str);
    let params: &Value = &message["params"];

    if message["jsonrpc"] != "2.0"
        || method.is_none()
        || id.is_some_and(|id| !(id.is_string() || id.is_i64() || id.is_u64()))
    {
        return Some(error(
            &Value::Null,
            -32600,
            "Expected a JSON-RPC 2.0 request or notification.",
        ));
    }

    let id: &Value = id?;

    if message.get("params").is_some() && !params.is_object() {
        return Some(error(id, -32602, "Expected a parameters object."));
    }

    let result: Value = match method {
        Some("initialize") => {
            let Some(version) = params["protocolVersion"].as_str() else {
                return Some(error(id, -32602, "Expected a protocolVersion string."));
            };
            if !params["capabilities"].is_object()
                || !params["clientInfo"]["name"].is_string()
                || !params["clientInfo"]["version"].is_string()
            {
                return Some(error(
                    id,
                    -32602,
                    "Expected capabilities and clientInfo with a name and version.",
                ));
            }
            let version: &str = if is_supported_version(version) {
                version
            } else {
                PROTOCOL_VERSION
            };
            json!({
                "protocolVersion": version,
                "capabilities": {"resources": {}},
                "serverInfo": {"name": "finch", "version": env!("CARGO_PKG_VERSION")}
            })
        }
        Some("ping") => json!({}),
        Some("resources/list") => json!({
            "resources": [{
                "uri": SCHEMA_URI,
                "name": "mappings-schema",
                "description": "JSON Schema for a Finch mappings file.",
                "mimeType": SCHEMA_MIME_TYPE
            }]
        }),
        Some("resources/templates/list") => json!({"resourceTemplates": []}),
        Some("resources/read") => {
            let Some(uri) = params["uri"].as_str() else {
                return Some(error(id, -32602, "Expected a resource URI string."));
            };
            if uri != SCHEMA_URI {
                return Some(error(id, -32002, "Resource not found."));
            }
            json!({
                "contents": [{
                    "uri": SCHEMA_URI,
                    "mimeType": SCHEMA_MIME_TYPE,
                    "text": schema::mappings().as_value().to_string()
                }]
            })
        }
        _ => return Some(error(id, -32601, "Method not found.")),
    };
    return Some(json!({"jsonrpc": "2.0", "id": id, "result": result}));
}

fn error(id: &Value, code: i32, message: &str) -> Value {
    return json!({"jsonrpc": "2.0", "id": id, "error": {"code": code, "message": message}});
}
