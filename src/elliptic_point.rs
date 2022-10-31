use crate::clean_up::big;
use crate::elliptic_curve::EllipticCurve;
use crate::modular_numbers::ModNum;
use num_bigint::BigUint;
use std::fmt;

//Defines the two different types of points on an elliptic curve
#[derive(Debug, Eq, PartialEq)]
pub enum EllipticType {
    Infinity(Inf),
    Point(Point),
}

// stores the nessacary information for Infinity on an elliptic curve
#[derive(Debug, Eq, PartialEq)]
pub struct Inf {
    field: BigUint,
}

// Stores the nessacary information for a point on an elliptic curve.
#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    x: ModNum,
    y: ModNum,
}

impl Point {
    // Creates a point on the curve described by y^2 = x^3 + a*x + b
    pub fn new(x: ModNum, y: ModNum, curve_a: &ModNum, curve_b: &ModNum) -> Point {
        // These asserts ensure that the operations are on numbers from the same field and are thus valid
        assert!(
            x.field() == y.field(),
            "x and y aren't from the same field."
        );
        assert!(
            x.field() == curve_a.field(),
            "x/y and curve_a aren't from the same field"
        );
        assert!(
            x.field() == curve_b.field(),
            "x/y/curve_a and curve_b aren't from the same field"
        );

        // This assert ensures that the given x and y are valid for the given a and b
        let test_val = x.pow(&big(3)).add(&(curve_a.mul(&x))).add(curve_b);
        assert!(
            y.pow(&big(2)) == test_val,
            "Not a valid point on the given curve"
        );

        Point { x, y }
    }

    pub fn x(&self) -> &ModNum {
        &self.x
    }
    pub fn y(&self) -> &ModNum {
        &self.y
    }
}

impl Inf {
    // Every curve has an infinity/0 and so the only characteristic about Infinity points is what field they belong to.
    pub fn new(field: &BigUint) -> Inf {
        Inf {
            field: field.clone(),
        }
    }
    pub fn field(&self) -> &BigUint {
        &self.field
    }
}

impl EllipticType {
    pub fn field(&self) -> &BigUint {
        match self {
            EllipticType::Infinity(inf) => &inf.field,
            EllipticType::Point(point) => point.x.field(),
        }
    }

    // This is group operation for elliptic curves. We first ensure that the points are from the same field and then that the curve is
    // from the same field as the points. Then the kind of EllipticType for each point is matched. Infinity/0 is the operation idenity
    //  for elliptic curves. If one object is Infinity we return the other. If both points are of type Point then an examination of
    // their x and y values is made. If Self.x = Other.x and Self.y = -Other.y then they combined to create a point at Infinity. If
    //  Self.X = Other.X but Self.Y != -Other.Y then Self.Y = Other.Y This requires finding the line tangent to the curve at that
    // point and calculating its slope to determine the new point created by combined Self and Other. Finally if neither of the above
    //  is true the slope of the line connecting Self and Other is calculated in the standard manner. This slope is used to determine
    // the new point created by the combination of Self and Other.
    pub fn group_op(&self, other: &Self, curve: &EllipticCurve) -> Self {
        assert!(
            self.field() == other.field(),
            "value for field on points don't match"
        );
        assert!(
            self.field() == curve.a().field(),
            "value of the field doesn't match for points and curve"
        );
        match self {
            // self = Infinity
            EllipticType::Infinity(_) => {
                return other.clone();
            }
            EllipticType::Point(left) => {
                match other {
                    //Other = Infinity
                    EllipticType::Infinity(_) => {
                        return EllipticType::Point(Point {
                            x: left.x.clone(),
                            y: left.y.clone(),
                        });
                        //return self.clone();
                    }
                    EllipticType::Point(right) => {
                        let slope: ModNum;
                        if left.x == right.x {
                            if left.y == right.y.add_inv() {
                                return EllipticType::Infinity(Inf::new(left.x.field()));
                            } else {
                                // (3x^2+a)/(2y)
                                slope = (ModNum::new(&big(3), left.x.field())
                                    .mul(&left.x.pow(&big(2)))
                                    .add(curve.a()))
                                .mul(
                                    &(ModNum::new(&big(2), left.x.field()).mul(&left.y)).mul_inv(),
                                );
                            }
                        } else {
                            // (right.y + (-left.y)) * 1/(right.x+(-left.x)) = (right.y-left.y)/(right.x-left.x)
                            slope = ((&right.y).add(&(&left.y).add_inv()))
                                .mul(&((&right.x).add(&(&left.x).add_inv())).mul_inv());
                        }
                        // slope^2 + (-left.x) + (-right.x) = slope^2 - xleft.x - right.x
                        let new_x = slope
                            .pow(&big(2))
                            .add(&(&left.x).add_inv())
                            .add(&(&right.x).add_inv());
                        let new_y = left
                            .y
                            .add_inv()
                            .add(&(slope.mul(&((&new_x).add(&left.x.add_inv())))).add_inv());
                        return EllipticType::Point(Point { x: new_x, y: new_y });
                    } //return left.group_op_point(&right, curve),
                }
            }
        }
    }

    // Returns the operational inverse of self. If Self.y = 0 or Self is Infinity then Self is it's own inverse
    pub fn group_inv(&self) -> Self {
        match self {
            EllipticType::Infinity(inf) => {
                return EllipticType::Infinity(Inf {
                    field: inf.field.clone(),
                })
            }
            EllipticType::Point(point) => {
                if point.y.value() == &big(0) {
                    return EllipticType::Point(Point {
                        x: point.x.clone(),
                        y: point.y.clone(),
                    });
                } else {
                    return EllipticType::Point(Point {
                        x: point.x.clone(),
                        y: point.y.add_inv(),
                    });
                }
            }
        }
    }

    // Performs exponentiation on elliptic type objects. Uses a recursive algorithm to do so.
    pub fn pow(&self, exp: &BigUint, curve: &EllipticCurve) -> Self {
        match self {
            EllipticType::Infinity(inf) => {
                return EllipticType::Infinity(Inf {
                    field: inf.field.clone(),
                })
            }
            EllipticType::Point(point) => {
                if exp == &big(0) {
                    return EllipticType::Infinity(Inf {
                        field: point.x.field().clone(),
                    });
                } else if exp == &big(1) {
                    return EllipticType::Point(Point {
                        x: point.x.clone(),
                        y: point.y.clone(),
                    });
                }
                if exp % &big(2) == big(0) {
                    let temp = self.pow(&(exp / &big(2)), curve);
                    return temp.group_op(&temp, curve);
                } else {
                    let temp = self.pow(&((exp - &big(1)) / &big(2)), curve);
                    return temp.group_op(&temp, curve).group_op(self, curve);
                }
            }
        }
    }
}

impl Clone for EllipticType {
    fn clone(&self) -> Self {
        match self {
            EllipticType::Infinity(inf) => {
                return EllipticType::Infinity(Inf {
                    field: inf.field().clone(),
                })
            }
            EllipticType::Point(point) => {
                return EllipticType::Point(Point {
                    x: point.x.clone(),
                    y: point.y.clone(),
                })
            }
        }
    }
}
impl Clone for Inf {
    fn clone(&self) -> Self {
        Inf {
            field: self.field.clone(),
        }
    }
}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl fmt::Display for EllipticType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EllipticType::Point(point) => {
                write!(
                    f,
                    "({},{}) in field F{}",
                    point.x.value(),
                    point.y.value(),
                    point.x.field(),
                )
            }
            EllipticType::Infinity(inf) => {
                write!(f, "Infinity in field F{}", inf.field,)
            }
        }
    }
}
