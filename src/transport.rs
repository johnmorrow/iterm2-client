use crate::auth::Credentials;
use crate::error::Result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::StreamExt;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::http::header::HeaderValue;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub type WsStream<S> = WebSocketStream<S>;
pub type WsSink<S> = SplitSink<WsStream<S>, tokio_tungstenite::tungstenite::Message>;
pub type WsSource<S> = SplitStream<WsStream<S>>;

const SUBPROTOCOL: &str = "api.iterm2.com";
const TCP_URL: &str = "ws://localhost:1912";
const LIBRARY_VERSION: &str = "0.1.0";

fn unix_socket_path() -> std::path::PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    std::path::PathBuf::from(home)
        .join("Library/Application Support/iTerm2/private/socket")
}

pub async fn connect_tcp(
    credentials: &Credentials,
    app_name: &str,
) -> Result<(WsSink<MaybeTlsStream<tokio::net::TcpStream>>, WsSource<MaybeTlsStream<tokio::net::TcpStream>>)> {
    let mut request = TCP_URL.into_client_request()?;
    apply_headers(request.headers_mut(), credentials, app_name);
    let config = ws_config();
    let (ws_stream, _response) =
        tokio_tungstenite::connect_async_with_config(request, Some(config), false).await?;
    Ok(ws_stream.split())
}

pub async fn connect_unix(
    credentials: &Credentials,
    app_name: &str,
) -> Result<(WsSink<tokio::net::UnixStream>, WsSource<tokio::net::UnixStream>)> {
    let path = unix_socket_path();
    let stream = tokio::net::UnixStream::connect(&path).await?;
    connect_with_stream(stream, credentials, app_name).await
}

pub async fn connect_with_stream<S>(
    stream: S,
    credentials: &Credentials,
    app_name: &str,
) -> Result<(WsSink<S>, WsSource<S>)>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let mut request = TCP_URL.into_client_request()?;
    apply_headers(request.headers_mut(), credentials, app_name);
    let config = ws_config();
    let (ws_stream, _response) =
        tokio_tungstenite::client_async_with_config(request, stream, Some(config)).await?;
    Ok(ws_stream.split())
}

pub async fn connect(
    credentials: &Credentials,
    app_name: &str,
) -> Result<(
    WsSink<MaybeTlsStream<tokio::net::TcpStream>>,
    WsSource<MaybeTlsStream<tokio::net::TcpStream>>,
)> {
    // Try Unix socket first, fall back to TCP
    let path = unix_socket_path();
    if path.exists() {
        match tokio::net::UnixStream::connect(&path).await {
            Ok(stream) => {
                // We can't return a different type here, so we just fall through to TCP
                // The unix connect function returns a different type, so callers who want
                // unix should use connect_unix directly.
                let _ = stream;
            }
            Err(_) => {}
        }
    }
    connect_tcp(credentials, app_name).await
}

fn apply_headers(
    headers: &mut tokio_tungstenite::tungstenite::http::HeaderMap,
    credentials: &Credentials,
    app_name: &str,
) {
    headers.insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_static(SUBPROTOCOL),
    );
    headers.insert(
        "Origin",
        HeaderValue::from_static("ws://localhost"),
    );
    headers.insert(
        "x-iterm2-library-version",
        HeaderValue::from_str(&format!("rust {LIBRARY_VERSION}")).unwrap(),
    );
    headers.insert(
        "x-iterm2-cookie",
        HeaderValue::from_str(&credentials.cookie).unwrap(),
    );
    headers.insert(
        "x-iterm2-key",
        HeaderValue::from_str(&credentials.key).unwrap(),
    );
    headers.insert(
        "x-iterm2-advisory-name",
        HeaderValue::from_str(app_name).unwrap(),
    );
}

fn ws_config() -> WebSocketConfig {
    let mut config = WebSocketConfig::default();
    config.max_frame_size = Some(16 * 1024 * 1024);
    config.max_message_size = Some(64 * 1024 * 1024);
    config
}
