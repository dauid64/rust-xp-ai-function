use crate::{utils::XValue, Result};
use schemars::{schema_for, JsonSchema};
use serde_json::{json, Value};


#[derive(Debug)]
pub struct ToolSpec {
    pub fn_name: String,
    pub fn_description: String,
    pub params: Value,
}

pub fn tool_spec<T: JsonSchema>() -> Result<ToolSpec> {
    let root_schema = schema_for!(T);
    let mut json_schema: Value = serde_json::to_value(root_schema)?;

    let fn_name = json_schema.x_take("title")?;
    let fn_description = json_schema.x_take("description")?;

    println!("->> json_schema: \n{}", serde_json::to_string_pretty(&json_schema)?);

    let tool_spec = ToolSpec {
        fn_name,
        fn_description,
        params: json!({}),
    };

    println!("\n->> tool_spec: \n{tool_spec:?}");

    Ok(tool_spec)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use serde::{Deserialize, Serialize};

    use super::*;

    /// # get_weather
    /// get the weather for a city
    #[allow(unused)]
    #[derive(Debug, Deserialize, schemars::JsonSchema)]
    struct GetWeatherParams {
        /// The city and state, e.g. San Francisco, CA
        location: String,
        /// The full country name of the city
        country: String,
        /// Unit respecting the country of the city
        unit: TempUnit,
    }

    #[derive(Debug, Serialize, Deserialize, schemars::JsonSchema)]
    enum TempUnit {
        Celcius,
        Fahrenheit,
    }


    #[test]
    fn test_tool_spec() -> Result<()> {
        tool_spec::<GetWeatherParams>();
        
    
        Ok(())
    }
}

// endregion: --- Tests