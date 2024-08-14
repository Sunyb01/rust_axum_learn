use actix_web::web::Data;

use crate::model::DingDepartment;
use crate::AppState;

pub async fn get_ding_depts(data: Data<AppState>, offset: i32, limit: i32) -> Vec<DingDepartment> {
    let depts: Vec<DingDepartment> = sqlx::query_as!(
        DingDepartment,
        r#"SELECT id, department_id, department_name, parent_department_id, sub_department_id, gmt_create, gmt_modify, is_deleted FROM ding_department where is_deleted = 0 ORDER by id LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    depts
}
