// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, Copy, Clone)]
pub struct Duration(u64);

impl From<u64> for Duration {
  fn from(s: u64) -> Self {
    Duration(s)
  }
}

impl From<Duration> for u64 {
  fn from(d: Duration) -> Self {
    d.0
  }
}

pub trait Planet {
  fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_planet {
	($struct_name:ident, $orbital_period:expr) => {
		pub struct $struct_name;

    impl Planet for $struct_name {
      fn years_during(d: &Duration) -> f64 {
        let seconds = u64::from(*d);
        let years = (seconds as f64) / (($orbital_period as f64) * 31557600.0);

        years
      }
    }
  };
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);