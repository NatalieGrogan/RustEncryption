use crate::clean_up::big;
use crate::elliptic_curve::EllipticCurve;
use crate::elliptic_point::{EllipticType, Point};
use crate::modular_numbers::ModNum;
use num_bigint::{BigUint, RandBigInt};

// This value is used to determine was sized chunks to use for message encoding.
const W: u32 = 2_u32.pow(8);

// These are the possible encodings to use.
// These encoding were copied from http://www.secg.org/sec2-v2.pdf While is paper is 12 years old there
//  is not yet a version 3 that I could find.
// They are referred to as secp256r1, secp384r1, and secp521r1 respectibely. Users can also specify their own
// curve. However the field the user selects must be greater than 65_537, 256^2 + 1. This is due to specific
//  implementation choices.
pub enum Curves {
    TwoFiveSix,
    ThreeEightFour,
    FiveTwoOne,
    Custom(EllipticCurve),
}

#[derive(Debug)]
pub struct ElGamal {
    curve: EllipticCurve,
    public_key: EllipticType,
    private_key: BigUint,
}

impl ElGamal {
    // Invokes the correct constructor for the ElGamal struct.
    pub fn new(curve: Curves) -> ElGamal {
        match curve {
            Curves::TwoFiveSix => return Self::new_256(),
            Curves::ThreeEightFour => return Self::new_384(),
            Curves::FiveTwoOne => return Self::new_521(),
            Curves::Custom(curve) => return Self::new_custom(curve),
        }
    }

    fn new_256() -> ElGamal {
        let field = BigUint::parse_bytes(
            b"FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF",
            16,
        )
        .unwrap();
        let a = BigUint::parse_bytes(
            b"FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFC",
            16,
        )
        .unwrap();
        let a = ModNum::new(&a, &field);
        let b = BigUint::parse_bytes(
            b"5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B",
            16,
        )
        .unwrap();
        let b = ModNum::new(&b, &field);
        let init_x = BigUint::parse_bytes(
            b"036B17D1F2E12C4247F8BCE6E563A440F277037D812DEB33A0F4A13945D898C296",
            16,
        )
        .unwrap();
        let init_x = ModNum::new(&init_x, &field);
        let init_y = calc_y(&init_x, &a, &b);
        let init_point = EllipticType::Point(Point::new(init_x, init_y, &a, &b));
        let curve = EllipticCurve::new(a, b, init_point);
        let mut rng = rand::thread_rng();
        let private_key = rng.gen_biguint_range(&big(0), &field);
        let public_key = curve.init_point().pow(&private_key, &curve);

        ElGamal {
            curve,
            public_key,
            private_key,
        }
    }

    fn new_384() -> ElGamal {
        let field = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFF0000000000000000FFFFFFFF",
            16,
        )
        .unwrap();
        let a = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFF0000000000000000FFFFFFFC",
            16,
        )
        .unwrap();
        let a = ModNum::new(&a, &field);
        let b = BigUint::parse_bytes(
            b"B3312FA7E23EE7E4988E056BE3F82D19181D9C6EFE8141120314088F5013875AC656398D8A2ED19D2A85C8EDD3EC2AEF",
            16,
        )
        .unwrap();
        let b = ModNum::new(&b, &field);
        let init_x = BigUint::parse_bytes(
            b"03AA87CA22BE8B05378EB1C71EF320AD746E1D3B628BA79B9859F741E082542A385502F25DBF55296C3A545E3872760AB7",
            16,
        )
        .unwrap();
        let init_x = ModNum::new(&init_x, &field);
        let init_y = calc_y(&init_x, &a, &b);
        let init_point = EllipticType::Point(Point::new(init_x, init_y, &a, &b));
        let curve = EllipticCurve::new(a, b, init_point);
        let mut rng = rand::thread_rng();
        let private_key = rng.gen_biguint_range(&big(0), &field);
        let public_key = curve.init_point().pow(&private_key, &curve);

        ElGamal {
            curve,
            public_key,
            private_key,
        }
    }

    fn new_521() -> ElGamal {
        let field = BigUint::parse_bytes(
            b"01FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
            16,
        )
        .unwrap();
        let a = BigUint::parse_bytes(
            b"01FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFC",
            16,
        )
        .unwrap();
        let a = ModNum::new(&a, &field);
        let b = BigUint::parse_bytes(
            b"0051953EB9618E1C9A1F929A21A0B68540EEA2DA725B99B315F3B8B489918EF109E156193951EC7E937B1652C0BD3BB1BF073573DF883D2C34F1EF451FD46B503F00",
            16,
        )
        .unwrap();
        let b = ModNum::new(&b, &field);
        let init_x = BigUint::parse_bytes(
            b"0200C6858E06B70404E9CD9E3ECB662395B4429C648139053FB521F828AF606B4D3DBAA14B5E77EFE75928FE1DC127A2FFA8DE3348B3C1856A429BF97E7E31C2E5BD66",
            16,
        )
        .unwrap();
        let init_x = ModNum::new(&init_x, &field);
        let init_y = calc_y(&init_x, &a, &b);
        let init_point = EllipticType::Point(Point::new(init_x, init_y, &a, &b));
        let curve = EllipticCurve::new(a, b, init_point);
        let mut rng = rand::thread_rng();
        let private_key = rng.gen_biguint_range(&big(0), &field);
        let public_key = curve.init_point().pow(&private_key, &curve);

        ElGamal {
            curve,
            public_key,
            private_key,
        }
    }

    fn new_custom(curve: EllipticCurve) -> ElGamal {
        let field = curve.a().field();
        let mut rng = rand::thread_rng();
        let private_key = rng.gen_biguint_range(&big(0), &field);
        let public_key = curve.init_point().pow(&private_key, &curve);

        ElGamal {
            curve,
            private_key,
            public_key,
        }
    }

    pub fn curve(&self) -> &EllipticCurve {
        &self.curve
    }
    pub fn public_key(&self) -> &EllipticType {
        &self.public_key
    }
    pub fn private_key(&self) -> &BigUint {
        &self.private_key
    }
}

pub fn decrypt(
    private_key: &BigUint,
    curve: &EllipticCurve,
    cipher_text: &Vec<(EllipticType, EllipticType)>,
) -> String {
    fn decode(m: &EllipticType) -> String {
        match m {
            EllipticType::Infinity(_) => {
                panic!("I'm not sure how to handle this right now")
            }
            EllipticType::Point(point) => {
                let x = point.x().value() / &big(W);
                let temp_m = x.to_bytes_le();
                let message = String::from_utf8(temp_m);
                match message {
                    Err(e) => {
                        panic!("Didn't return valid utf8 {:?}", e)
                    }
                    Ok(mes) => {
                        return mes;
                    }
                }
            }
        }
    }
    let mut plain_text: String = String::new();
    for message_pair in cipher_text {
        let c_0 = &message_pair.0;
        let c_1 = &message_pair.1;

        let m = ((c_0.group_inv()).pow(private_key, curve)).group_op(c_1, curve);
        let message = decode(&m);
        plain_text.push_str(&message);
    }
    plain_text
}

pub fn encrypt(
    public_key: &EllipticType,
    curve: &EllipticCurve,
    plain_text: &String,
) -> Vec<(EllipticType, EllipticType)> {
    let field = curve.field();
    fn encode(text: &BigUint, curve: &EllipticCurve) -> EllipticType {
        let mut counter: u32 = 0;
        loop {
            let intermediate = big(W) * text + &big(counter);
            let x = ModNum::new(&intermediate, curve.field());
            let y = calc_y(&x, curve.a(), curve.b());
            if y.value() != &big(0) {
                return EllipticType::Point(Point::new(x, y, &curve.a(), &curve.b()));
            } else {
                counter += 1;
            }
            if counter > W {
                panic!(
                    "Something exceeding rare, 10^-78 level rare, occured. Increase the size of W"
                );
            }
        }
    }

    // Splits the full string message into chunks of appropriate size as determined by n. Then converts those
    //  appropriately sized chunks into integers and combines them into a Vec.
    fn split(message: &String, n: usize) -> Vec<BigUint> {
        let mes_bytes = message.as_bytes();
        let mut messages: Vec<BigUint> = Vec::new();
        for i in (0..mes_bytes.len()).step_by(n) {
            let mes_as_num = if (i + n) > (mes_bytes.len() - 1) {
                BigUint::from_bytes_le(&mes_bytes[i..])
            } else {
                BigUint::from_bytes_le(&mes_bytes[i..i + n])
            };
            messages.push(mes_as_num);
        }
        messages
    }
    // n - Determines the maximimum chunk size for message encoding. Allows messages 256 potentional values
    //  This is needed because not every value is a valid point on the curve. ~50% of x values generate a
    //   valid z such that there exists a y^2 = z. With 256 additional values of X to choose from the
    // potentional that this algorithim doesn't find a valid point is ~10^-78. For referenc it is estimated
    //  this is approximately the number of atoms in the universe. Aka if you repeated this encoding once
    // for each atom in the universe it would fail ~ONE(!!!) time.
    let n: usize = ((curve.field() / big(256)).bits() / 8).try_into().unwrap();
    let message_list = split(&plain_text, n);
    let mut point_mes_list: Vec<EllipticType> = Vec::new();
    for mes_as_num in message_list {
        point_mes_list.push(encode(&mes_as_num, &curve));
    }
    let mut encrypted_message_vec: Vec<(EllipticType, EllipticType)> = Vec::new();
    let mut rng = rand::thread_rng();

    for point in &point_mes_list {
        // The specific value for s is irrelevant to decryption. So in order to increase the difficulty of the
        // task of breaking encryption a random value for s i used.
        let s = rng.gen_biguint_range(&(field / (2 * W)), &(field / W));

        // C_0 = (initial_curve_position)^s
        let c_0 = curve.init_point().pow(&s, curve);
        // h_to_the_s = (public_key)^s = (initial_curve_position)^(private_key)^s
        let h_to_the_s = public_key.pow(&s, curve);
        // C_1 = h_to_the_s * (message_chunk_as_a_number) = (initial_curve_position)^(private_key)^s * message
        let c_1 = h_to_the_s.group_op(&point, curve);
        encrypted_message_vec.push((c_0, c_1));
    }

    encrypted_message_vec
}

// Generates one of the y values for a given x
fn calc_y(x: &ModNum, a: &ModNum, b: &ModNum) -> ModNum {
    x.pow(&big(3)).add(&(a.mul(&x))).add(b).sqrt()
}
