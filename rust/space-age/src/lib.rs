#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / 31557600 as f64)
    }
}

pub trait Planet {
    fn ratio() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::ratio()
    }
}

macro_rules! impl_plant {
    ($n:ident, $p:expr) => {
        pub struct $n;
        impl Planet for $n {
            fn ratio() -> f64 {
                $p
            }
        }
    };
}

impl_plant!(Mercury, 0.2408467);
impl_plant!(Venus, 0.61519726);
impl_plant!(Earth, 1.0);
impl_plant!(Mars, 1.8808158);
impl_plant!(Jupiter, 11.862615);
impl_plant!(Saturn, 29.447498);
impl_plant!(Uranus, 84.016846);
impl_plant!(Neptune, 164.79132);
