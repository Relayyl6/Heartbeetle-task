use crate::models::{JobQueue, JobStatus};
use rand::Rng;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct Worker {
    id: usize,
    queue: Arc<JobQueue>,
}

impl Worker {
    pub fn new(id: usize, queue: Arc<JobQueue>) -> Self {
        Self { id, queue }
    }

    pub async fn start(self) {
        println!("Worker {} started", self.id);

        loop {
            // Check for next job
            if let Some(job) = self.queue.get_next_pending_job().await {
                println!("Worker {} picked up job {}", self.id, job.job_id);

                // Process the job
                let result = self.process_job(&job.payload).await;

                match result {
                    Ok(output) => {
                        println!("Worker {} completed job {}", self.id, job.job_id);
                        self.queue
                            .update_job_status(job.job_id, JobStatus::Completed, Some(output))
                            .await;
                    }
                    Err(error) => {
                        eprintln!("Worker {} failed job {}: {}", self.id, job.job_id, error);
                        
                        // Try to retry
                        let retried = self.queue.retry_job(job.job_id).await;
                        
                        if retried {
                            println!("Job {} queued for retry", job.job_id);
                        } else {
                            // Max retries reached
                            self.queue
                                .update_job_status(
                                    job.job_id,
                                    JobStatus::Failed,
                                    Some(format!("Failed after {} retries: {}", job.max_retries, error)),
                                )
                                .await;
                        }
                    }
                }
            } else {
                // No jobs available, sleep briefly
                sleep(Duration::from_millis(100)).await;
            }
        }
    }

    async fn process_job(&self, payload: &str) -> Result<String, String> {
        // Simulate long-running task
        let sleep_duration = {
            let mut rng = rand::rng();
            rng.random_range(2..=5)  // ✅ RNG dropped here
        };
        println!(
            "Worker {} processing payload '{}' (will take {}s)",
            self.id, payload, sleep_duration
        );

        sleep(Duration::from_secs(sleep_duration)).await;

        // Parse payload and execute
        if payload.starts_with("generate_report_for_user:") {
            let user_id = payload
                .split(':')
                .nth(1)
                .ok_or("Invalid payload format")?;
            
            Ok(format!("Report generated for user {}", user_id))
        } else if payload.starts_with("send_email:") {
            let email = payload
                .split(':')
                .nth(1)
                .ok_or("Invalid payload format")?;
            
            Ok(format!("Email sent to {}", email))
        } else if payload == "fail" {
            // Simulate failure for testing retries
            Err("Simulated failure".to_string())
        } else {
            Ok(format!("Processed: {}", payload))
        }
    }
}

// Spawn multiple workers (Bonus)
pub fn spawn_workers(num_workers: usize, queue: Arc<JobQueue>) {
    for id in 0..num_workers {
        let worker = Worker::new(id, queue.clone());
        tokio::spawn(async move {
            worker.start().await;
        });
    }
}