/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JettonVerificationType {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist,
    #[serde(rename = "none")]
    None,

}

impl ToString for JettonVerificationType {
    fn to_string(&self) -> String {
        match self {
            Self::Whitelist => String::from("whitelist"),
            Self::Blacklist => String::from("blacklist"),
            Self::None => String::from("none"),
        }
    }
}

impl Default for JettonVerificationType {
    fn default() -> JettonVerificationType {
        Self::Whitelist
    }
}



