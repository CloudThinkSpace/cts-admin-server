use serde::Deserialize;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleMenuDto {
    pub menu_ids: Vec<String>,
    pub role_id: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleApiDto {
    pub api_ids: Vec<String>,
    pub role_id: String,
}
