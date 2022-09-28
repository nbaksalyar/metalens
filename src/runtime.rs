use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use futures::StreamExt;
use petgraph::algo::toposort;
use redbpf::load::{Loaded, Loader, LoaderError};
use tokio::time;
use tracing::info;

use crate::{ws::MsgChannelTx, ProgGraph};

pub type EventDeserializer = fn(Box<[u8]>) -> serde_json::Value;

#[derive(Debug)]
pub enum RuntimeError {
    // Toposort failed, cycles in the program graph.
    ProgContainsCycles,
    // Program load failed.
    LoadError(LoaderError),
    /// Expected property has not been found.
    ExpectedProperty(&'static str),
    /// I/O error
    IoError(std::io::Error),
    /// Error with a string description
    // FIXME
    Other(String),
}

impl From<std::io::Error> for RuntimeError {
    fn from(e: std::io::Error) -> Self {
        RuntimeError::IoError(e)
    }
}

pub struct LoadedState {
    deserializers: HashMap<String, EventDeserializer>,
    pub prog: Loaded,
    pub drop_state: Arc<Mutex<bool>>,
}

impl LoadedState {
    pub fn new(prog: Loaded) -> Self {
        Self {
            prog,
            deserializers: HashMap::new(),
            drop_state: Arc::new(Mutex::new(false)),
        }
    }

    pub fn register_deserializer(&mut self, map_name: String, deser: EventDeserializer) {
        self.deserializers.insert(map_name, deser);
    }
}

impl Drop for LoadedState {
    fn drop(&mut self) {
        // signal that this program has been dropped
        let mut drop_state = self.drop_state.lock().unwrap();
        *drop_state = true;
    }
}

pub fn load_bpf_prog(
    nodes: &ProgGraph,
    bpf_prog: &[u8],
    out_stream: MsgChannelTx,
) -> Result<LoadedState, RuntimeError> {
    let mut prog_state = LoadedState::new(Loader::load(bpf_prog).map_err(RuntimeError::LoadError)?);

    let mut exec_order = toposort(&nodes, None)
        .map_err(|_| RuntimeError::ProgContainsCycles)?
        .into_iter();

    while let Some(node_idx) = exec_order.next() {
        let node = &nodes[node_idx];
        node.load(&mut prog_state, out_stream.clone())?;
    }

    Ok(prog_state)
}

pub async fn terminal_runtime(mut prog_state: LoadedState) {
    let counters_map = prog_state.prog.map("counters").unwrap().clone();

    let mut counters_reader_interval = time::interval(Duration::from_secs(1));
    let timers_task = tokio::spawn(async move {
        let counters_array = redbpf::Array::<u32>::new(&counters_map).unwrap();

        loop {
            counters_reader_interval.tick().await;
            info!("counter = {}", counters_array.get(0).unwrap());
        }
    });

    while let Some((map_name, events)) = prog_state.prog.events.next().await {
        dbg!(&map_name);

        for event in events {
            let pid: Result<[u8; 8], _> = (&*event).try_into();
            eprintln!("PID {}", u64::from_ne_bytes(pid.unwrap()));
        }
    }

    let _ = timers_task.await;
}
