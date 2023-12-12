use serde::{Deserialize, Serialize};

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
pub struct BlockLimits {
    #[serde(rename = "bytes")]
    pub bytes: Box<crate::rest_api::codegen::models::BlockParamLimits>,
    #[serde(rename = "gas")]
    pub gas: Box<crate::rest_api::codegen::models::BlockParamLimits>,
    #[serde(rename = "lt_delta")]
    pub lt_delta: Box<crate::rest_api::codegen::models::BlockParamLimits>,
}

impl BlockLimits {
    pub fn new(
        bytes: crate::rest_api::codegen::models::BlockParamLimits,
        gas: crate::rest_api::codegen::models::BlockParamLimits,
        lt_delta: crate::rest_api::codegen::models::BlockParamLimits,
    ) -> BlockLimits {
        BlockLimits {
            bytes: Box::new(bytes),
            gas: Box::new(gas),
            lt_delta: Box::new(lt_delta),
        }
    }
}