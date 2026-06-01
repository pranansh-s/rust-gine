use std::time::{Duration, Instant};

pub struct Timer {
    duration: Duration,
    remaining: Duration,
    repeating: bool,
    finished: bool,
}

impl Timer {
    pub fn new(duration: Duration, repeating: bool) -> Self {
        Self {
            duration,
            remaining: duration,
            repeating,
            finished: false,
        }
    }

    pub fn tick(&mut self, delta: Duration) -> bool {
        if self.finished && !self.repeating {
            return false;
        }

        if self.remaining <= delta {
            if self.repeating {
                let overflow = delta - self.remaining;
                self.remaining = self.duration.saturating_sub(overflow);
            } else {
                self.remaining = Duration::ZERO;
                self.finished = true;
            }
            true
        } else {
            self.remaining -= delta;
            self.finished = false;
            false
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn reset(&mut self) {
        self.remaining = self.duration;
        self.finished = false;
    }
}

pub struct TimeSystem {
    start_time: Instant,
    last_frame_time: Instant,
    delta_time: Duration,
    elapsed_time: Duration,
    fixed_timestep: Duration,
    fixed_accumulator: Duration,
    scheduled_tasks: Vec<(Timer, Box<dyn FnMut() + Send + Sync>)>,
}

impl TimeSystem {
    pub fn new(fixed_fps: f32) -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_frame_time: now,
            delta_time: Duration::ZERO,
            elapsed_time: Duration::ZERO,
            fixed_timestep: Duration::from_secs_f32(1.0 / fixed_fps),
            fixed_accumulator: Duration::ZERO,
            scheduled_tasks: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame_time);
        self.last_frame_time = now;
        self.elapsed_time = now.duration_since(self.start_time);
        
        self.fixed_accumulator += self.delta_time;
        
        let delta = self.delta_time;
        for (timer, callback) in &mut self.scheduled_tasks {
            if timer.tick(delta) {
                callback();
            }
        }
        
        self.scheduled_tasks.retain(|(timer, _)| !timer.is_finished() || timer.repeating);
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_time.as_secs_f64()
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed_time.as_secs_f32()
    }

    pub fn check_fixed_update(&mut self) -> bool {
        if self.fixed_accumulator >= self.fixed_timestep {
            self.fixed_accumulator -= self.fixed_timestep;
            true
        } else {
            false
        }
    }

    pub fn fixed_delta_seconds(&self) -> f32 {
        self.fixed_timestep.as_secs_f32()
    }

    pub fn schedule_once<F>(&mut self, delay: Duration, callback: F)
    where
        F: FnMut() + Send + Sync + 'static,
    {
        self.scheduled_tasks.push((Timer::new(delay, false), Box::new(callback)));
    }

    pub fn schedule_repeating<F>(&mut self, interval: Duration, callback: F)
    where
        F: FnMut() + Send + Sync + 'static,
    {
        self.scheduled_tasks.push((Timer::new(interval, true), Box::new(callback)));
    }
}
