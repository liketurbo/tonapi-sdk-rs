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
pub struct GetChartRates200Response {
    #[serde(rename = "points")]
    pub points: ::std::collections::HashMap<String, serde_json::Value>,
}

impl GetChartRates200Response {
    pub fn new(
        points: ::std::collections::HashMap<String, serde_json::Value>,
    ) -> GetChartRates200Response {
        GetChartRates200Response { points }
    }
}