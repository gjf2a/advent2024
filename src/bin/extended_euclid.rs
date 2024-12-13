use std::{fmt::Display, ops::{AddAssign, SubAssign}};
use trait_set::trait_set;
use num::{Num, Signed};

trait_set! {
    pub trait EuclidNum = Num + Copy + Display + Signed + Ord + AddAssign + SubAssign;
}

fn main() {
    let args = std::env::args().skip(1).map(|a| a.parse::<i128>().unwrap()).collect::<Vec<_>>();
    if args.len() < 3 {
        println!("Usage: extended_euclid a b c (to find (x, y) in ax + by = c)")
    } else {
        for (x, y) in LinearDiophantine::new(args[0], args[1], args[2]) {
            println!("{x} {y}");
        }
    }
}

#[derive(Debug)]
struct LinearDiophantine<N: EuclidNum> {
    x: N,
    y: N,
    co_x: N,
    co_y: N,
}

impl<N: EuclidNum> LinearDiophantine<N> {
    pub fn new(a: N, b: N, c: N) -> Self {
        // From https://www.perplexity.ai/search/how-do-you-find-all-possible-x-B9.PeotlQLec9dYtLV_FlA
        let (gcd, x, y) = gcd_x_y(a, b);
        println!("gcd: {gcd} x: {x} y: {y}");
        if c % gcd == N::zero() {
            let goal_over_gcd = c / gcd;
            let mut x = x * goal_over_gcd;
            let mut y = y * goal_over_gcd;
            let mut co_x = -(b / gcd);
            let mut co_y = a / gcd;
            println!("co_x: {co_x} co_y: {co_y}");
            if y < N::zero() {
                let gap = -(y / co_y);
                println!("gap: {gap}");
                x += gap * co_x;
                y += gap * co_y;
                if y < num::zero() {
                    x += co_x;
                    y += co_y;
                }
            }
            println!("post-y: x: {x} y: {y}");
            if x < N::zero() {
                let gap = x / co_x;
                println!("gap: {gap}");
                x -= gap * co_x;
                y -= gap * co_y;
                if x < N::zero() {
                    x -= co_x;
                    y -= co_y;
                }
            }
            println!("post-x: x: {x} y: {y}");
            if x < y {
                co_x = -co_x;
                co_y = -co_y;
            }
            println!("post-negate: co_x: {co_x} co_y: {co_y}");
            Self {x, y, co_x, co_y}
        } else {
            let neg = -N::one();
            Self {x: neg, y: neg, co_x: neg, co_y: neg}
        }
    }
}

impl<N: EuclidNum> Iterator for LinearDiophantine<N> {
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
