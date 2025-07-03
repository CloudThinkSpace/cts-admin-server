use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Validations {
    pub expression_validations:Vec<ExpressionValidation>
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionValidation {
    pub expression:String,
    pub message:String
}