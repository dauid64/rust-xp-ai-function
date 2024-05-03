// region --- Modules

mod ai_tools;
mod weather;

// -- Flatten
pub use ai_tools::*;
use rpc_router::{ResourcesBuilder, RouterBuilder};

use crate::{chat, Result};

// endregion: --- Modules

pub fn new_ai_tools(resources: Option<ResourcesBuilder>) -> Result<AiTools> {
    let router = RouterBuilder::default()
        .extend_resources(resources)
        .extend(weather::router_builder())
        .build();

    let mut chat_tools = Vec::new();
    chat_tools.extend(weather::chat_tools()?);

    Ok(AiTools::new(router, chat_tools))
}