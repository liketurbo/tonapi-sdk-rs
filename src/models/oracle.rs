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
pub struct Oracle {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "secp_pubkey")]
    pub secp_pubkey: String,
}

impl Oracle {
    pub fn new(address: String, secp_pubkey: String) -> Oracle {
        Oracle {
            address,
            secp_pubkey,
        }
    }
}

