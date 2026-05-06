use std::{
    collections::{HashMap, VecDeque},
    sync::Mutex,
    time::{Duration, Instant},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum McpRateLimitKey {
    InvalidAuthIp(String),
    Token(String),
}

#[derive(Debug)]
pub struct McpRateLimiter {
    limit: usize,
    window: Duration,
    buckets: Mutex<HashMap<McpRateLimitKey, VecDeque<Instant>>>,
}

impl McpRateLimiter {
    pub fn new(limit: usize, window: Duration) -> Self {
        Self {
            limit,
            window,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub fn per_minute(limit: usize) -> Self {
        Self::new(limit, Duration::from_secs(60))
    }

    pub fn check(&self, key: McpRateLimitKey, now: Instant) -> Result<(), Duration> {
        let mut buckets = self
            .buckets
            .lock()
            .expect("mcp rate limiter mutex poisoned");
        let bucket = buckets.entry(key).or_default();
        while bucket
            .front()
            .is_some_and(|oldest| now.duration_since(*oldest) >= self.window)
        {
            bucket.pop_front();
        }

        if bucket.len() >= self.limit {
            let oldest = bucket
                .front()
                .copied()
                .expect("limited bucket should contain at least one timestamp");
            let elapsed = now.duration_since(oldest);
            return Err(self.window.saturating_sub(elapsed));
        }

        bucket.push_back(now);
        Ok(())
    }

    pub fn is_limited(&self, key: &McpRateLimitKey, now: Instant) -> Option<Duration> {
        let mut buckets = self
            .buckets
            .lock()
            .expect("mcp rate limiter mutex poisoned");
        let bucket = buckets.get_mut(key)?;
        while bucket
            .front()
            .is_some_and(|oldest| now.duration_since(*oldest) >= self.window)
        {
            bucket.pop_front();
        }

        if bucket.len() >= self.limit {
            let oldest = bucket
                .front()
                .copied()
                .expect("limited bucket should contain at least one timestamp");
            let elapsed = now.duration_since(oldest);
            return Some(self.window.saturating_sub(elapsed));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn limiter_rejects_after_limit_until_window_expires() {
        let limiter = McpRateLimiter::new(2, Duration::from_secs(60));
        let key = McpRateLimitKey::InvalidAuthIp("203.0.113.10".to_string());
        let start = Instant::now();

        assert!(limiter.check(key.clone(), start).is_ok());
        assert!(
            limiter
                .check(key.clone(), start + Duration::from_secs(1))
                .is_ok()
        );

        let retry_after = limiter
            .check(key.clone(), start + Duration::from_secs(2))
            .expect_err("third request in the same window should be limited");
        assert_eq!(retry_after, Duration::from_secs(58));

        assert!(limiter.check(key, start + Duration::from_secs(61)).is_ok());
    }

    #[test]
    fn is_limited_does_not_consume_capacity() {
        let limiter = McpRateLimiter::new(2, Duration::from_secs(60));
        let key = McpRateLimitKey::InvalidAuthIp("203.0.113.10".to_string());
        let start = Instant::now();

        assert!(limiter.is_limited(&key, start).is_none());
        assert!(limiter.check(key.clone(), start).is_ok());
        assert!(
            limiter
                .is_limited(&key, start + Duration::from_secs(1))
                .is_none()
        );
        assert!(
            limiter
                .check(key.clone(), start + Duration::from_secs(2))
                .is_ok()
        );
        assert_eq!(
            limiter.is_limited(&key, start + Duration::from_secs(3)),
            Some(Duration::from_secs(57))
        );
    }
}
