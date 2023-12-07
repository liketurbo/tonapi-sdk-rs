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
pub struct BlockchainBlockShards {
    #[serde(rename = "shards")]
    pub shards: Vec<crate::models::BlockchainBlockShardsShardsInner>,
}

impl BlockchainBlockShards {
    pub fn new(shards: Vec<crate::models::BlockchainBlockShardsShardsInner>) -> BlockchainBlockShards {
        BlockchainBlockShards {
            shards,
        }
    }
}

