use chrono::NaiveDateTime;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Null;

impl Display for Null {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NULL")
    }
}

pub trait DbType: Display + Debug {
    fn display(&self) -> String;
    fn source(&self) -> String {
        format!("{self}")
    }
}

impl DbType for String {
    fn display(&self) -> String {
        format!("'{self}'")
    }
}

impl DbType for bool {
    fn display(&self) -> String {
        format!("{self}")
    }
}

impl DbType for i32 {
    fn display(&self) -> String {
        format!("{self}")
    }
}
impl DbType for f64 {
    fn display(&self) -> String {
        format!("{self}")
    }
}
impl DbType for i64 {
    fn display(&self) -> String {
        format!("{self}")
    }
}

impl DbType for f32 {
    fn display(&self) -> String {
        format!("{self}")
    }
}

impl DbType for NaiveDateTime {
    fn display(&self) -> String {
        format!("'{self}'")
    }
}

impl DbType for Null {
    fn display(&self) -> String {
        "NUlL".to_string()
    }
}

