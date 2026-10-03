use std::collections::VecDeque;
use std::f32::consts::TAU;

#[derive(Clone, Copy, PartialEq)]
pub enum Scene {
    Orbit,
    Wave,
}

pub fn position(index: usize, count: usize, time: f32, scene: Scene) -> (f32, f32, f32) {
    let n = index as f32;
    let q = n / count.max(1) as f32;
    match scene {
        Scene::Orbit => {
            let radius = q.sqrt() * 0.43;
            let angle = n * 2.399_963_1 + time * (0.16 + (1.0 - q) * 0.22);
            let pulse = 1.0 + 0.055 * (time * 1.4 + radius * 18.0).sin();
            (
                0.5 + angle.cos() * radius * pulse,
                0.5 + angle.sin() * radius * pulse * 0.78,
                (angle.sin() + 1.0) * 0.5,
            )
        }
        Scene::Wave => {
            let columns = (count as f32 * 1.8).sqrt().ceil() as usize;
            let rows = count.div_ceil(columns);
            let x = (index % columns) as f32 / (columns - 1).max(1) as f32;
            let y = (index / columns) as f32 / (rows - 1).max(1) as f32;
            let z = (x * TAU * 1.5 + time).sin() * (y * TAU + time * 0.6).cos();
            (0.07 + x * 0.86, 0.15 + y * 0.7 + z * 0.08, (z + 1.0) * 0.5)
        }
    }
}

#[derive(Default)]
pub struct FrameStats {
    pub samples: VecDeque<f32>,
}
impl FrameStats {
    pub fn push(&mut self, ms: f32) {
        if !ms.is_finite() || ms <= 0.0 {
            return;
        }
        self.samples.push_back(ms);
        if self.samples.len() > 180 {
            self.samples.pop_front();
        }
    }
    pub fn summary(&self) -> (f32, f32, f32) {
        if self.samples.is_empty() {
            return (0.0, 0.0, 0.0);
        }
        let mean = self.samples.iter().sum::<f32>() / self.samples.len() as f32;
        let mut sorted: Vec<f32> = self.samples.iter().copied().collect();
        sorted.sort_by(f32::total_cmp);
        let p95 = sorted[(sorted.len() as f32 * 0.95).ceil() as usize - 1];
        (1000.0 / mean, mean, p95)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timing_reports_real_slow_frames() {
        let mut stats = FrameStats::default();
        for _ in 0..95 {
            stats.push(10.0);
        }
        for _ in 0..5 {
            stats.push(100.0);
        }
        let (fps, mean, p95) = stats.summary();
        assert!((mean - 14.5).abs() < 0.001);
        assert!((fps - 1000.0 / 14.5).abs() < 0.001);
        assert_eq!(p95, 10.0);
        stats.push(200.0);
        assert_eq!(stats.summary().2, 100.0);
    }
    #[test]
    fn bounded_history_and_invalid_samples() {
        let mut stats = FrameStats::default();
        stats.push(f32::NAN);
        stats.push(0.0);
        assert!(stats.samples.is_empty());
        for i in 1..=200 {
            stats.push(i as f32);
        }
        assert_eq!(stats.samples.len(), 180);
        assert_eq!(stats.samples[0], 21.0);
    }
    #[test]
    fn scenes_fit_normalized_canvas_at_all_loads() {
        for scene in [Scene::Orbit, Scene::Wave] {
            for count in [1000, 10000, 30000] {
                for i in 0..count {
                    let (x, y, z) = position(i, count, 31.7, scene);
                    assert!((0.0..=1.0).contains(&x));
                    assert!((0.0..=1.0).contains(&y));
                    assert!((0.0..=1.0).contains(&z));
                }
            }
        }
    }
}
