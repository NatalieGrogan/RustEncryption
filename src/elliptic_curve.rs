use crate::elliptic_point::EllipticType;
use crate::modular_numbers::ModNum;
use num_bigint::BigUint;
use std::fmt;

// Simple struct that holds the a, b, and starting point for an elliptic curve
//  Elliptic curves have the form y^2 = x^3 + a*x + b
//   Curves usually, always(?), come with an initial point. This isn't strictly required but if it wasn't
//  supplied you would need to calculate, fairly simple, and publish an additional piece of information along
//   with your public_key
#[derive(Debug, Eq, PartialEq)]
pub struct EllipticCurve {
    a: ModNum,
    b: ModNum,
    init_point: EllipticType,
}

impl EllipticCurve {
    // Creates an elliptic curve struct. ensures that the supplied data is from the same field thus ensuring
    //  the calculations generated with the data is valid/meaningful.
    pub fn new(a: ModNum, b: ModNum, init_point: EllipticType) -> EllipticCurve {
        assert!(a.field() == b.field(), "a & b aren't from the same field");
        assert!(
            a.field() == init_point.field(),
            "a and init_x aren't from the same field"
        );

        match init_point {
            EllipticType::Infinity(inf) => {
                return EllipticCurve {
                    a,
                    b,
                    init_point: EllipticType::Infinity(inf),
                }
            }
            EllipticType::Point(point) => {
                return EllipticCurve {
                    a: a,
                    b: b,
                    init_point: EllipticType::Point(point),
                };
            }
        }
    }

    pub fn a(&self) -> &ModNum {
        &self.a
    }
    pub fn b(&self) -> &ModNum {
        &self.b
    }
    pub fn field(&self) -> &BigUint {
        &self.a.field()
    }
    pub fn init_point(&self) -> &EllipticType {
        &self.init_point
    }
}

impl Clone for EllipticCurve {
    fn clone(&self) -> EllipticCurve {
        EllipticCurve {
            a: self.a.clone(),
            b: self.b.clone(),
            init_point: self.init_point.clone(),
        }
    }
}

impl fmt::Display for EllipticCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "x^3 + {}x + {} in field F{}",
            self.a.value(),
            self.b.value(),
            self.a.field()
        )
    }
}
