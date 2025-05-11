fn main() {
    let seconds = 2_134_835_688;
    let duration = Duration::from(seconds);
    let output = Mercury::years_during(&duration);
    let expected = 280.88;
    print!("{:.2} vs {:.2}", output, expected)
}

#[derive(Debug)]
pub struct Duration {
    e: f64,
}
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let e = (s as f64) / 31_557_600_f64;
        Self { e }
    }
}
pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.e / Self::ORBITAL_PERIOD
    }
}
pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;
impl Planet for Mercury { const ORBITAL_PERIOD: f64 = 0.2408467; }
impl Planet for Venus { const ORBITAL_PERIOD: f64 = 0.61519726; }
impl Planet for Earth { const ORBITAL_PERIOD: f64 = 1.0; }
impl Planet for Mars { const ORBITAL_PERIOD: f64 = 1.8808158; }
impl Planet for Jupiter { const ORBITAL_PERIOD: f64 = 11.862615; }
impl Planet for Saturn { const ORBITAL_PERIOD: f64 = 29.447498; }
impl Planet for Uranus { const ORBITAL_PERIOD: f64 = 84.016846; }
impl Planet for Neptune { const ORBITAL_PERIOD: f64 = 164.79132; }