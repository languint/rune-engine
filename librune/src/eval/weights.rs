use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Weights(pub i16, pub i16);

impl Weights {
    #[must_use]
    #[inline]
    pub fn middlegame(&self) -> i16 {
        self.0
    }

    #[must_use]
    #[inline]
    pub fn endgame(&self) -> i16 {
        self.1
    }
}

impl ops::Add for Weights {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
