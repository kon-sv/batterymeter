use std::{fmt};
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[derive(Default, Debug)]
pub struct Metric {
    pub timestamp: u128,
    pub energy_now: i32,
    pub capacity: i8,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(T:{}, C:{}, EN:{})",
            self.timestamp, self.capacity, self.energy_now
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
