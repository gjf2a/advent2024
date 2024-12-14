use std::{fmt::Display, ops::{AddAssign, SubAssign}};
use trait_set::trait_set;
use num::{Num, Signed};

trait_set! {
    pub trait EuclidNum = Num + Copy + Display + Signed + Ord + AddAssign + SubAssign;
}

#[derive(Debug)]
pub struct LinearDiophantinePositive<N: EuclidNum> {
    x: N,
    y: N,
    co_x: N,
    co_y: N,
}

impl<N: EuclidNum> LinearDiophantinePositive<N> {
    pub fn new(a: N, b: N, c: N) -> Self {
        // From https://www.perplexity.ai/search/how-do-you-find-all-possible-x-B9.PeotlQLec9dYtLV_FlA
        let (gcd, x, y) = gcd_x_y(a, b);
        if c % gcd == N::zero() {
            let goal_over_gcd = c / gcd;
            let mut result = Self {
                x: x * goal_over_gcd,
                y: y * goal_over_gcd,
                co_x: -(b / gcd),
                co_y: a / gcd,
            };
            result.ensure_positive_y();
            result.ensure_positive_x();
            if x < y {
                result.co_x = -result.co_x;
                result.co_y = -result.co_y;
            }
            result
        } else {
            let neg = -N::one();
            Self {x: neg, y: neg, co_x: neg, co_y: neg}
        }
    }

    fn ensure_positive_y(&mut self) {
        if self.y < N::zero() {
            let gap = -(self.y / self.co_y);
            self.x += gap * self.co_x;
            self.y += gap * self.co_y;
            if self.y < num::zero() {
                self.x += self.co_x;
                self.y += self.co_y;
            }
        }
    }

    fn ensure_positive_x(&mut self) {
        if self.x < N::zero() {
            let gap = self.x / self.co_x;
            self.x -= gap * self.co_x;
            self.y -= gap * self.co_y;
            if self.x < N::zero() {
                self.x -= self.co_x;
                self.y -= self.co_y;
            }
        }
    }
}

impl<N: EuclidNum> Iterator for LinearDiophantinePositive<N> {
    type Item = (N, N);
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= N::zero() && self.y >= N::zero() {
            let result = (self.x, self.y);
            self.x += self.co_x;
            self.y += self.co_y;
            Some(result)
        } else {
            None
        }
    }
}

// From https://brilliant.org/wiki/extended-euclidean-algorithm/#extended-euclidean-algorithm
fn gcd_x_y<N: EuclidNum>(a: N, b: N) -> (N, N, N) {
    let mut s = N::zero();
    let mut old_s = N::one();
    let mut t = N::one();
    let mut old_t = N::zero();
    let mut r = b;
    let mut old_r = a;

    while r != N::zero() {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
        (old_t, t) = (t, old_t - quotient * t);
    }

    (old_r, old_s, old_t)
}
