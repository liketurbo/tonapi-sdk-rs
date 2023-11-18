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
pub struct FoundAccountsAddressesInner {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "preview")]
    pub preview: String,
}

impl FoundAccountsAddressesInner {
    pub fn new(address: String, name: String, preview: String) -> FoundAccountsAddressesInner {
        FoundAccountsAddressesInner {
            address,
            name,
            preview,
        }
    }
}

