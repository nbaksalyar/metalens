//! WebSocket server.

use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::{future, pin_mut, SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::{Message as WsMessage, Result as WsResult};
use tracing::{info, warn};

use crate::codegen::CodegenError;
use crate::runtime;
use crate::{codegen::CodegenResult, dsl::Elem, ProgGraph};

pub type MsgChannelTx = UnboundedSender<Message>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub action: String,
    pub payload: String,
}

pub async fn start() {
    let addr = "0.0.0.0:8080".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) -> WsResult<()> {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (outgoing, mut incoming) = ws_stream.split();

    let (tx, rx) = unbounded();
    let tx2 = tx.clone();

    let mut loaded_prog = None;

    let handle_incoming = tokio::spawn(async move {
        while let Some(Ok(ws_msg)) = incoming.next().await {
            let msg: Message =
                serde_json::from_str(ws_msg.to_text().expect("expected a text message"))
                    .expect("invalid json");

            match msg.action.as_str() {
                "compile" => {
                    let res = compile(&msg.payload);
                    match res {
                        Ok((codegen, prog_graph)) => {
                            dbg!("codegen successful");

                            {
                                // drop previously loaded prog before attaching a new one
                                let _ = loaded_prog.take();
                            }

                            // execute the prog
                            let prog =
                                runtime::load_bpf_prog(&prog_graph, &codegen.bpf, tx2.clone());

                            if let Err(e) = prog {
                                warn!("failed to execute a prog: {:?}", e);
                                continue;
                            }

                            loaded_prog.replace(prog);

                            let mut tx3 = tx2.clone();

                            tokio::spawn(async move {
                                tx3.send(Message {
                                    action: "asm".to_owned(),
                                    payload: codegen.asm,
                                })
                                .await
                                .expect("failed to send msg");
                            });
                        }
                        Err(e) => {
                            warn!("compilation failed: {:?}", e);
                        }
                    }
                }
                _ => {
                    warn!("Unknown action; message: {:?}", msg);
                }
            }
        }
    });

    let receive_from_others = rx
        .map(|msg| {
            Ok(WsMessage::text(
                serde_json::to_string(&msg).expect("invalid json msg"),
            ))
        })
        .forward(outgoing);

    pin_mut!(handle_incoming, receive_from_others);
    future::select(handle_incoming, receive_from_others).await;

    Ok(())
}

fn compile(prog: &str) -> Result<(CodegenResult, ProgGraph), CodegenError> {
    let nodes: Vec<Elem> = serde_json::from_str(prog).expect("could not parse the program");
    let prog = crate::dsl::construct_prog(nodes)?;
    Ok((crate::codegen::generate(&prog)?, prog))
}
