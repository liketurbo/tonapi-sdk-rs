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
pub struct NftItem {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "index")]
    pub index: i64,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::AccountAddress>>,
    #[serde(rename = "collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Box<crate::models::NftItemCollection>>,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "metadata")]
    pub metadata: ::std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "sale", skip_serializing_if = "Option::is_none")]
    pub sale: Option<Box<crate::models::Sale>>,
    #[serde(rename = "previews", skip_serializing_if = "Option::is_none")]
    pub previews: Option<Vec<crate::models::ImagePreview>>,
    #[serde(rename = "dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<String>,
    #[serde(rename = "approved_by")]
    pub approved_by: Vec<ApprovedBy>,
}

impl NftItem {
    pub fn new(address: String, index: i64, verified: bool, metadata: ::std::collections::HashMap<String, serde_json::Value>, approved_by: Vec<ApprovedBy>) -> NftItem {
        NftItem {
            address,
            index,
            owner: None,
            collection: None,
            verified,
            metadata,
            sale: None,
            previews: None,
            dns: None,
            approved_by,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApprovedBy {
    #[serde(rename = "getgems")]
    Getgems,
    #[serde(rename = "tonkeeper")]
    Tonkeeper,
    #[serde(rename = "ton.diamonds")]
    TonPeriodDiamonds,
}

impl Default for ApprovedBy {
    fn default() -> ApprovedBy {
        Self::Getgems
    }
}
