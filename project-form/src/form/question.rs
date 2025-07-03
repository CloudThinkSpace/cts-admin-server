use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub r#type: String,
    pub name: String,
    pub title: String,
    pub unit: Option<String>,
    pub cached: Option<bool>,
    pub required: Option<bool>,
    pub description: Option<String>,
    pub error: Option<String>,
    pub default_value: Option<String>,
    pub items: Option<Vec<Item>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub code: String,
}