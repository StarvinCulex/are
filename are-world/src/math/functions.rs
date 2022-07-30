pub mod gauss {
    use std::f32::consts::TAU;

    #[inline]
    pub fn density(x: f32, mu: f32, sigma: f32) -> f32 {
        (-0.5 * ((x - mu) / sigma).powi(2)).exp() / sigma / TAU.sqrt()
    }
}
