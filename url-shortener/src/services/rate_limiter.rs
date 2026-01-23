use governor::{
    Quota, RateLimiter as GovRateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use std::{collections::HashMap, net::IpAddr, num::NonZeroU32, sync::Mutex};

pub struct RateLimiter {
    quota: Quota,
    limiters: Mutex<HashMap<IpAddr, GovRateLimiter<NotKeyed, InMemoryState, DefaultClock>>>,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        let quota = Quota::per_minute(NonZeroU32::new(requests_per_minute).unwrap());
        Self {
            limiters: Mutex::new(HashMap::new()),
            quota,
        }
    }

    pub fn check(&self, ip: IpAddr) -> bool {
        let mut limiters = self.limiters.lock().unwrap();
        let limiter = limiters
            .entry(ip)
            .or_insert_with(|| GovRateLimiter::direct(self.quota));
        limiter.check().is_ok()
    }

    #[allow(dead_code)]
    pub fn tracked_ips(&self) -> usize {
        self.limiters.lock().unwrap().len()
    }

    #[allow(dead_code)]
    pub fn cleanup_stale_limiters(&self) {
        let mut limiters = self.limiters.lock().unwrap();
        limiters.retain(|_, limiter| limiter.check().is_ok());
    }
}
