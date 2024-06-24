use std::collections::HashMap;
use common::FORCE;
use anyhow::Result;
use axum::response::IntoResponse;
use response_utils::res::ResResult;
use serde::Serialize;

pub mod base;
pub mod sys;

fn handle_force(params: HashMap<String, String>) -> bool {
    // 判断是否真删除
    match params.get(FORCE) {
        None => {
            false
        }
        Some(data) => {
            if data == "true" {
                true
            } else {
                false
            }
        }
    }
}

fn handle_result<T>(data:Result<T>)-> impl IntoResponse
where  T: Serialize
{
    match data {
        Ok(data) => {
            ResResult::with_success(data)
        }
        Err(_err) => {
            ResResult::<()>::with_error(&_err.to_string())
        }
    }
}