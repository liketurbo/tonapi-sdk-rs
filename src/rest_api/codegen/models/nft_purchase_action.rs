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
pub struct NftPurchaseAction {
    #[serde(rename = "auction_type")]
    pub auction_type: AuctionType,
    #[serde(rename = "amount")]
    pub amount: Box<crate::rest_api::codegen::models::Price>,
    #[serde(rename = "nft")]
    pub nft: Box<crate::rest_api::codegen::models::NftItem>,
    #[serde(rename = "seller")]
    pub seller: Box<crate::rest_api::codegen::models::AccountAddress>,
    #[serde(rename = "buyer")]
    pub buyer: Box<crate::rest_api::codegen::models::AccountAddress>,
}

impl NftPurchaseAction {
    pub fn new(
        auction_type: AuctionType,
        amount: crate::rest_api::codegen::models::Price,
        nft: crate::rest_api::codegen::models::NftItem,
        seller: crate::rest_api::codegen::models::AccountAddress,
        buyer: crate::rest_api::codegen::models::AccountAddress,
    ) -> NftPurchaseAction {
        NftPurchaseAction {
            auction_type,
            amount: Box::new(amount),
            nft: Box::new(nft),
            seller: Box::new(seller),
            buyer: Box::new(buyer),
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuctionType {
    #[serde(rename = "DNS.tg")]
    DnsPeriodTg,
    #[serde(rename = "getgems")]
    Getgems,
    #[serde(rename = "basic")]
    Basic,
}

impl Default for AuctionType {
    fn default() -> AuctionType {
        Self::DnsPeriodTg
    }
}