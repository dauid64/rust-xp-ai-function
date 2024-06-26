use rpc_router::{router_builder, RpcParams};
use rust_xp_ai_function::{chat, conv, oa_client::new_oa_client, tools::{new_ai_tools, AiTools}};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Init AI Client
    let oa_client = new_oa_client()?;

    // -- Get the AI Tools
    let ai_tools = new_ai_tools(None)?;

    // -- User question
    let questions = &[
        "What is the weather in the California's best city and Paris",
        "Why is the sky red? (be concise)",
        "what is the weather in italy's capital"
    ];

    // -- Execute question with conv
    for &question in questions {
        let response = conv::send_user_msg(oa_client.clone(), ai_tools.clone(), question).await?;

        println!(
            r#"
            == Question: {question}
            {response}
            "#
        );
    }
    

    Ok(())
}
