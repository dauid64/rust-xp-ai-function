use async_openai::types::ChatCompletionTool;
use rpc_router::{router_builder, RouterBuilder, RpcParams};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::chat;

pub(super) fn router_builder() -> RouterBuilder {
    router_builder![get_weather]
}

pub(super) fn chat_tools() -> crate::Result<Vec<ChatCompletionTool>> {
    let tool_weather = chat::tool_fn(
        "get_weather", 
        "get the weather for a city", 
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "country": {
                    "type": "string",
                    "description": "The full country name of the city"
                },
                "unit": {
                    "type": "string", "enum": ["celsius", "fahrenheit"],
                    "description": "Unit respecting the country of the city"
                }
            },
            "required": ["location", "country", "unit"],
        }),
    )?;
    
    Ok(vec![tool_weather])
}

#[allow(unused)]
#[derive(Debug, Deserialize, RpcParams, schemars::JsonSchema)]
struct GetWeatherParams {
    location: String,
    country: String,
    unit: String,
}

#[derive(Serialize)]
struct Weather {
    temperature: f64,
    unit: String,
    humindity_rh: f32,
}

async fn get_weather(params: GetWeatherParams) -> Result<Weather, String> {
    Ok(Weather {
        temperature: 30.,
        unit: params.unit,
        humindity_rh: 0.3,
    })
}