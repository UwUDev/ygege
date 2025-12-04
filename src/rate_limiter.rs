use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};

#[derive(Clone)]
pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    request_times: Arc<Mutex<VecDeque<Instant>>>,
    max_requests: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_duration: Duration, max_concurrent: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            request_times: Arc::new(Mutex::new(VecDeque::new())),
            max_requests,
            window_duration,
        }
    }

    pub fn default() -> Self {
        Self::new(5, Duration::from_secs(10), 5)
    }

    pub async fn acquire(&self) -> RateLimitGuard {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();

        loop {
            let mut times = self.request_times.lock().await;
            let now = Instant::now();

            while let Some(&oldest) = times.front() {
                if now.duration_since(oldest) > self.window_duration {
                    times.pop_front();
                } else {
                    break;
                }
            }

            if times.len() < self.max_requests {
                times.push_back(now);
                drop(times);
                break;
            } else {
                let oldest = *times.front().unwrap();
                let elapsed = now.duration_since(oldest);
                let wait_time = self.window_duration.saturating_sub(elapsed);

                if wait_time > Duration::from_millis(0) {
                    debug!(
                        "Rate limit reached ({} requests in {:?}), sleeping for {:?}",
                        times.len(),
                        self.window_duration,
                        wait_time
                    );
                    drop(times);
                    tokio::time::sleep(wait_time).await;
                } else {
                    times.pop_front();
                    times.push_back(now);
                    drop(times);
                    break;
                }
            }
        }

        RateLimitGuard { _permit: permit }
    }
}

pub struct RateLimitGuard {
    _permit: tokio::sync::OwnedSemaphorePermit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_concurrent() {
        let limiter = RateLimiter::new(10, Duration::from_secs(1), 3);

        let mut handles = vec![];
        for i in 0..5 {
            let limiter = limiter.clone();
            let handle = tokio::spawn(async move {
                let _guard = limiter.acquire().await;
                println!("Task {} acquired", i);
                tokio::time::sleep(Duration::from_millis(100)).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
