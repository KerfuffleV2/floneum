mod host;
mod plugin;
pub use plugin::*;
mod embedding_db;
mod llm;
mod node;
mod page;
mod proxies;
mod structure;

wasmtime::component::bindgen!({
    path: "../wit",
    async: true,
    world: "both",
});
