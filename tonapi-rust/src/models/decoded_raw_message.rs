/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecodedRawMessage {
    #[serde(rename = "message")]
    pub message: Box<crate::models::DecodedRawMessageMessage>,
    #[serde(rename = "mode")]
    pub mode: i32,
}

impl DecodedRawMessage {
    pub fn new(message: crate::models::DecodedRawMessageMessage, mode: i32) -> DecodedRawMessage {
        DecodedRawMessage {
            message: Box::new(message),
            mode,
        }
    }
}

