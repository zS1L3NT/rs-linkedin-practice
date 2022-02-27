#![allow(dead_code)]

enum Scale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    fn new(degrees: f32) -> Self {
        Temperature {
            degrees,
            scale: Scale::Celsius,
        }
    }

    fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => (self.degrees - 32.0) * 5.0 / 9.0,
        }
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees * 9.0 / 5.0 + 32.0,
            Scale::Fahrenheit => self.degrees,
        }
    }
}

#[test]
fn one_degree() {
    let cold = Temperature::new(1.0);
    assert!((cold.to_fahrenheit() - 33.8) < 0.01);
    assert!((cold.to_fahrenheit() - 33.8) >= 0.0);
}

#[test]
fn boiling() {
    let hot = Temperature::new(100.0);
    assert!((hot.to_fahrenheit() - 212.0) < 0.01);
    assert!((hot.to_fahrenheit() - 212.0) >= 0.0);
}

#[test]
fn freezing() {
    let freezing = Temperature {
        degrees: Temperature::new(0.0).to_fahrenheit(),
        scale: Scale::Fahrenheit,
    };

    assert!(freezing.to_celsius() < 0.001);
    assert!(freezing.to_celsius() > -0.01);
}
