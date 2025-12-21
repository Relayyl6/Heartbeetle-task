use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::{CreateJob, Job, JobStatus, JobQueue, JobPriority};

impl JobQueue {
    pub fn new(
        max_queue_size: usize
    ) -> Self {
        Self {
            jobs: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(Mutex::new(1)),
            max_queue_size,
        }
    }

    pub async fn add_job(
        &self,
        create_job: CreateJob
    ) -> Result<Job, String> {
        let mut jobs = self.jobs.lock().await;
        
        // Check queue size limit
        if jobs.len() >= self.max_queue_size {
            return Err("Queue is full".to_string());
        }

        let mut next_id = self.next_id.lock().await;
        let job_id = *next_id;
        *next_id += 1;

        let now = Utc::now();
        let expires_at = create_job.ttl_seconds.map(|ttl| now + chrono::Duration::seconds(ttl));

        let job = Job {
            job_id,
            status: JobStatus::Pending,
            payload: create_job.payload,
            result: None,
            priority: create_job.priority,
            retries: 0,
            max_retries: create_job.max_retries.unwrap_or(3),
            created_at: now,
            updated_at: now,
            expires_at,
        };

        jobs.push(job.clone());
        
        // Sort by priority - Higher priority first
        jobs.sort_by(|a, b| {
            let priority_a = a.priority.as_ref().unwrap_or(&JobPriority::Medium);
            let priority_b = b.priority.as_ref().unwrap_or(&JobPriority::Medium);
            priority_b.cmp(priority_a)  // Reverse order: High -> Medium -> Low
        });

        Ok(job)
    }

    pub async fn get_job(&self, job_id: u64) -> Option<Job> {
        let jobs = self.jobs.lock().await;
        jobs.iter().find(|j| j.job_id == job_id).cloned()
    }

    pub async fn get_all_jobs(&self) -> Vec<Job> {
        let jobs = self.jobs.lock().await;
        jobs.clone()
    }

    pub async fn get_next_pending_job(&self) -> Option<Job> {
        let mut jobs = self.jobs.lock().await;
        let now = Utc::now();

        // Remove expired jobs
        jobs.retain(|job| {
            if let Some(expires_at) = job.expires_at {
                expires_at > now
            } else {
                true
            }
        });

        // Find first pending job (already sorted by priority)
        if let Some(job) = jobs.iter_mut().find(|j| j.status == JobStatus::Pending) {
            job.status = JobStatus::Running;
            job.updated_at = Utc::now();
            return Some(job.clone());
        }

        None
    }

    pub async fn update_job_status(
        &self,
        job_id: u64,
        status: JobStatus,
        result: Option<String>,
    ) {
        let mut jobs = self.jobs.lock().await;
        if let Some(job) = jobs.iter_mut().find(|j| j.job_id == job_id) {
            job.status = status;
            job.result = result;
            job.updated_at = Utc::now();
        }
    }

    pub async fn retry_job(&self, job_id: u64) -> bool {
        let mut jobs = self.jobs.lock().await;
        if let Some(job) = jobs.iter_mut().find(|j| j.job_id == job_id) {
            if job.retries < job.max_retries {
                job.retries += 1;
                job.status = JobStatus::Pending;
                job.updated_at = Utc::now();
                return true;
            }
        }
        false
    }
}