pub struct Race {
    pub time: u32,
    pub distance_to_beat: u64
}

impl Race {
    pub fn solve(&self) -> u32 {
        let time_float = f64::from(self.time);
        let distance_float = self.distance_to_beat as f64;
        let delta : f64 = time_float * time_float - 4.0 * distance_float;
        if delta > 0.0 {
            let delta_sqrt = f64::from(delta).sqrt();
            let x1 = (time_float + delta_sqrt) / 2.0;
            let x2 = (time_float - delta_sqrt) / 2.0;
            let lower_bound = (x2.floor() as u32 + 1 ).max(0);
            let higher_bound = (x1.ceil() as u32 - 1).min(self.time);
            assert!(lower_bound <= higher_bound);
            higher_bound - lower_bound + 1
        } else if delta == 0.0 {
            0
        } else {
            panic!("No solution")
        }
        
    }
}