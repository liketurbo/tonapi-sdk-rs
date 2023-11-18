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
pub struct EmulateMessageToWalletRequest {
    #[serde(rename = "boc")]
    pub boc: String,
    /// additional per account configuration
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<crate::models::EmulateMessageToWalletRequestParamsInner>>,
}

impl EmulateMessageToWalletRequest {
    pub fn new(boc: String) -> EmulateMessageToWalletRequest {
        EmulateMessageToWalletRequest {
            boc,
            params: None,
        }
    }
}

