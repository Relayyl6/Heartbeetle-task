use actix_web::{web, HttpResponse, Responder};
use crate::models::{JobQueue, CreateJob, StatusJobQuery};

pub async fn create_job(
    queue: web::Data<JobQueue>,
    req: web::Json<CreateJob>,
) -> impl Responder {
    match queue.add_job(req.into_inner()).await {
        Ok(job) => HttpResponse::Created().json(serde_json::json!({
            "message": "Job created successfully",
            "job_id": job.job_id,
            "status": job.status,
        })),
        Err(e) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }))
    }
}

pub async fn get_job(
    queue: web::Data<JobQueue>,
    path: web::Path<u64>,
) -> impl Responder {
    let job_id = path.into_inner();

    match queue.get_job(job_id).await {
        Some(job) => HttpResponse::Ok().json(job),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("Job with id {} not found", job_id)
        }))
    }
}

pub async fn list_jobs(
    queue: web::Data<JobQueue>,
) -> impl Responder {
    let jobs = queue.get_all_jobs().await;
    HttpResponse::Ok().json(jobs)
}

// Bonus: Get jobs by status
pub async fn list_jobs_by_status(
    queue: web::Data<JobQueue>,
    query: web::Query<StatusJobQuery>,
) -> impl Responder {
    let all_jobs = queue.get_all_jobs().await;
    let filtered: Vec<_> = all_jobs
        .into_iter()
        .filter(|j| j.status == query.status)
        .collect();
    
    HttpResponse::Ok().json(filtered)
}

