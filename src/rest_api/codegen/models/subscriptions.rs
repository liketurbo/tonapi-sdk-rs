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
pub struct Subscriptions {
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<crate::rest_api::codegen::models::Subscription>,
}

impl Subscriptions {
    pub fn new(
        subscriptions: Vec<crate::rest_api::codegen::models::Subscription>,
    ) -> Subscriptions {
        Subscriptions { subscriptions }
    }
}