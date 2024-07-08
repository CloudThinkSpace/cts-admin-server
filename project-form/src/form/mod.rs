use std::fmt::{Display, Formatter};

pub mod question;
pub mod form_json;
pub mod form_util;
pub mod validation;

pub enum FormCommonField {
    Id,
    Code,
    Lon,
    Lat,
    Status,
    UserId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

impl Display for FormCommonField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormCommonField::Id => {
                write!(f, "id")
            }
            FormCommonField::Code => {
                write!(f, "code")
            }
            FormCommonField::Lon => {
                write!(f, "lon")
            }
            FormCommonField::Lat => {
                write!(f, "lat")
            }
            FormCommonField::Status => {
                write!(f, "status")
            }
            FormCommonField::UserId => {
                write!(f, "user_id")
            }
            FormCommonField::CreatedAt => {
                write!(f, "created_at")
            }
            FormCommonField::UpdatedAt => {
                write!(f, "updated_at")
            }
            FormCommonField::DeletedAt => {
                write!(f, "deleted_at")
            }
        }
    }
}

impl FormCommonField {
    pub fn contains(data: &str) -> bool {
        match data {
            "id" | "code" | "lon" | "lat" | "status" | "user_id" | "created_at" | "updated_at" | "deleted_at" => {
                true
            }
            &_ => {
                false
            }
        }
    }
}
