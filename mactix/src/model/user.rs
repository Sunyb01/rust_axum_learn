//! entity
//!

use chrono::{self, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[allow(non_snake_case)]
pub struct DingDepartment {
    pub id: i64,
    pub department_id: i64,
    pub department_name: String,
    pub parent_department_id: i64,
    pub sub_department_id: Option<String>,
    pub gmt_create: NaiveDateTime,
    pub gmt_modify: NaiveDateTime,
    pub is_deleted: i8,
}
