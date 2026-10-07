use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format_json(input: &str) -> Result<String, JsValue> {
    // checking if this is json. it better fucking be
    let value: serde_json::Value = serde_json::from_str(input)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    // adding some spaces; civilization, apparently.
    serde_json::to_string_pretty(&value)
        .map_err(|err| JsValue::from_str(&err.to_string()))
}
