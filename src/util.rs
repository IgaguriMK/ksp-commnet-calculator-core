use std::fmt;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct MetricPrefix(pub f64);

impl fmt::Display for MetricPrefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = self.0;
        if x >= 1_000_000_000.0 {
            write!(f, "{:.2} G", x / 1_000_000_000.0)
        } else if x >= 1_000_000.0 {
            write!(f, "{:.2} M", x / 1_000_000.0)
        } else if x >= 1_000.0 {
            write!(f, "{:.2} k", x / 1_000.0)
        } else {
            write!(f, "{:.2}", x)
        }
    }
}
