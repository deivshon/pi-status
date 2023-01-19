use serde::Serialize;

use crate::StatusFields;

#[derive(Serialize)]
pub struct DummyStruct {
    pub x: i128,
    pub y: i64
}

pub fn get() -> StatusFields {
    return crate::StatusFields::Dummy(None)
}
