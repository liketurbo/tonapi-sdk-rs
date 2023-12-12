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
pub struct GetAllRawShardsInfo200Response {
    #[serde(rename = "id")]
    pub id: Box<crate::rest_api::codegen::models::BlockRaw>,
    #[serde(rename = "proof")]
    pub proof: String,
    #[serde(rename = "data")]
    pub data: String,
}

impl GetAllRawShardsInfo200Response {
    pub fn new(
        id: crate::rest_api::codegen::models::BlockRaw,
        proof: String,
        data: String,
    ) -> GetAllRawShardsInfo200Response {
        GetAllRawShardsInfo200Response {
            id: Box::new(id),
            proof,
            data,
        }
    }
}