use serde::{Deserialize, Serialize};
use crate::form::question::Question;
use crate::form::validation::Validations;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormTemplate {
    pub form: Form,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub name: String,
    pub title: String,
    pub description: String,
    pub version: String,
    pub questions: Vec<Question>,
    pub validations: Option<Validations>,
}
