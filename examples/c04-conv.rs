use async_openai::types::{ChatCompletionToolChoiceOption, CreateChatCompletionRequest};
use rpc_router::{router_builder, RpcParams};
use rust_xp_ai_function::{chat::{self, first_choice}, conv, gpts, oa_client::new_oa_client, tools::AiTools};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, RpcParams)]
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Init AI Client
    let oa_client = new_oa_client()?;
    let chat_client = oa_client.chat();
    let model = gpts::MODEL;

    // -- User question
    let question = "What is the weather in the California's best city and Paris";

    // -- Build messages
    let messages = vec![chat::user_msg(question)?];

    // -- Build tools
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

    // -- Init rpc_router
    let rpc_router = router_builder![get_weather].build();

    let ai_tools = AiTools::new(rpc_router, vec![tool_weather]);

    // -- Execute question with conv
    let response = conv::send_user_msg(oa_client, ai_tools, question).await?;

    println!("\nFinal asnwer:\n\n{response}");

    Ok(())
}
