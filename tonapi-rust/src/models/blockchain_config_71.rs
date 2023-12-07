/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockchainConfig71 : Bridge parameters for wrapping TON in other networks.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig71 {
    #[serde(rename = "oracle_bridge_params")]
    pub oracle_bridge_params: Box<crate::models::OracleBridgeParams>,
}

impl BlockchainConfig71 {
    /// Bridge parameters for wrapping TON in other networks.
    pub fn new(oracle_bridge_params: crate::models::OracleBridgeParams) -> BlockchainConfig71 {
        BlockchainConfig71 {
            oracle_bridge_params: Box::new(oracle_bridge_params),
        }
    }
}

