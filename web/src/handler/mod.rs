use std::collections::HashMap;
use common::FORCE;

pub mod base;
pub mod sys;

fn handler_force(params: HashMap<String, String>) -> bool {
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