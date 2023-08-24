//! Understand why macos is failing
use failure_server::ToxicTcpProxy;
use noxious_client::{StreamDirection, Toxic, ToxicKind};

const STATIC_HTTP_SERVER_LISTEN: &str = "127.0.0.1:10101";
const TCP_PROXY_LISTEN: &str = "127.0.0.1:10102";
const TCP_PROXY_CONFIG_API_LISTEN: &str = "127.0.0.1:8472";

#[tokio::main]
async fn main() {
    let mut toxic_tcp_proxy = ToxicTcpProxy::new(
        "toxictuf".to_string(),
        TCP_PROXY_LISTEN,
        STATIC_HTTP_SERVER_LISTEN,
        TCP_PROXY_CONFIG_API_LISTEN,
    )
    .expect("Couldn't ToxicTcpProxy::new()")
    .with_toxic(Toxic {
        name: "slowclose".to_string(),
        kind: ToxicKind::SlowClose { delay: 500 },
        toxicity: 0.75,
        direction: StreamDirection::Downstream,
    })
    .with_toxic(Toxic {
        name: "timeout".to_string(),
        kind: ToxicKind::Timeout { timeout: 100 },
        toxicity: 0.5,
        direction: StreamDirection::Downstream,
    });

    toxic_tcp_proxy.start().await.expect("Couldn't start toxic tcp proxy");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await
}
