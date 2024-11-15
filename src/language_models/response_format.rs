use schemars::{schema_for, JsonSchema};
use serde::de::DeserializeOwned;

pub trait GenerateSchema {
    fn generate_schema<T: DeserializeOwned + JsonSchema>(
        description: Option<String>,
        name: String,
        strict: Option<bool>,
    ) -> async_openai::types::ResponseFormatJsonSchema;
}

impl GenerateSchema for async_openai::types::ResponseFormatJsonSchema {
    fn generate_schema<T: DeserializeOwned + JsonSchema>(
        description: Option<String>,
        name: String,
        strict: Option<bool>,
    ) -> Self {
        let schema = schema_for!(T).schema; // Use schema_for! instead of schema_for_value!
        Self {
            description,
            name,
            schema: Some(serde_json::to_value(schema).unwrap_or_default()),
            strict,
        }
    }
}
