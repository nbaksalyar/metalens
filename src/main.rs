mod codegen;
mod dsl;
mod formulas;
mod nodes;
mod runtime;
mod ws;

use nodes::Node;
use petgraph::graph::DiGraph;
use tracing_subscriber;

pub type ProgGraph = DiGraph<Node, ()>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // start a websocket server
    let handle = tokio::spawn(ws::start());

    // let args: Vec<String> = std::env::args().collect();
    // demo_load_from_file(&args[1]).await.unwrap();

    let _ = handle.await;

    Ok(())
}

/*
async fn demo_load_from_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::BufReader;
    use dsl::Elem;

    // Construct a prog from DSL
    let f = File::open(path)?;
    let freader = BufReader::new(f);

    let nodes: Vec<Elem> = serde_json::from_reader(freader).expect("could not read json"); // TODO fix panic
    let prog = dsl::construct_prog(nodes);
    let res = codegen::generate(&prog);

    // load the prog
    let prog = runtime::load_bpf_prog(&prog, &res.bpf).await;
    runtime::terminal_runtime(prog).await;

    Ok(())
}
 */
