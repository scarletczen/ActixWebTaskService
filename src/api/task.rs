use crate::model::task::Task;
use crate::model::task::TaskState;
use crate::repository::ddb::DDBRepository;

use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use serde::{Serialize, Deserialize};

use derive_more::{Display};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier{
    task_global_id: String,
}

#[derive(Deserialize)]
pub struct TaskCompletionRequest{
    result_file:String
}

#[derive(Deserialize)]
pub struct SubmitTaskRequest{
    user_id:String,
    task_type:String,
    source_file:String
}

pub enum TaskError{
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
     HttpResponse::build(self.status_code())
     .insert_header(ContentType::json())
     .body(self.to_string())   
    }
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST
            
        }
    }
}


#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIdentifier>, ddb_repo: Data<DDBRepository>) -> Result<Json<Task>,TaskError>{
    let task = ddb_repo.get_task(
        task_identifier.into_inner().task_global_id
    ).await;
    match task {
        Some(task)=>Ok(Json(task)),
        None=>Err(TaskError::TaskNotFound)
    }
}
