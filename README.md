This project is my attempt to learn Rust. It is in no way intended for real world use and is only a toy. I have implemented the El Gamal encryption scheme, described here - https://caislab.kaist.ac.kr/lecture/2010/spring/cs548/basic/B02.pdf, over Elliptic Curves. As a recent Mathematics graduate I'm motivated by programming that is closely related to mathematical topics. I have also implemented this same project in Python where it was criminally slow.

To use this library you need to create an El Gamal struct.

  ```
  // You can select 256, 384, 521 bit encryption
   // 256 - Curves::TwoFiveSix
   // 384 - Curves::ThreeEightFour
   // 521 - Curves::FiveTwoOne
   let el_gamal = ElGamal::new(Curves::TwoFiveSix);
```

to encrypt use

   ```let cipher_text = encrypt(&el_gamal.public_key(), &el_gamal.curve(), &plain_text);```


to decrypt

  ``` let message = decrypt(&el_gamal.private_key(), el_gamal.curve(), &cipher_text);```


The libray is built from the BigNum crate.` ModNom`s are modular numbers with a `value` and `field` each of which is a `BigUint`.You can add or multiply `ModNum`s with `self.add(&other)` and `self.mul(&other)` respectively. You can also invert them with `add_inv` or `mul_inv`, find the `sqrt`, or use `pow` do exponentiation.


`EllipticType`s are enums which can be `Point`s or `Infinity`. `Point`s contain an `x`,`y` which are `ModNum`s. You can use `group_op` to combine them, `group_inv` to find their inverse, or `pow` to do exponentiation.

`EllipticCurves` contain the `a`, `b`, and `init_point` needed to fully describe and use an elliptic curve.