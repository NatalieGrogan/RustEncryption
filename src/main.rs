use encryption::el_gamal::{decrypt, encrypt, Curves, ElGamal};

fn main() {
    // You can select 256, 384, 521 bit encryption
    // 256 - Curves::TwoFiveSix
    // 384 - Curves::ThreeEightFour
    // 521 - Curves::FiveTwoOne
    // Custom - Create you're own curve object and pass it in. The field of your custom curve must be greater
    //           than 256^2 + 1
    let twofivesix = ElGamal::new(Curves::TwoFiveSix);

    // Just some bit of the Matrix screenplay I copied in to demonstrate the functionality
    let plain_text = "The main offices are along each wall, the windows
	overlooking downtown Chicago.

	RHINEHEART, the ultimate company man, lectures Neo
	without looking at him, typing at his computer
	continuously.

	Neo stares at two window cleaners on a scaffolding
	outside, dragging their rubber squeegees down across the
	surface of the glass.

					RHINEHEART
			You have a problem, Mr. Anderson.
			You think that you're special.
			You believe that somehow the rules
			do not apply to you.

	He stops, glancing over his glasses at Neo, who turns in
	time.

					RHINEHEART
			Obviously, you are mistaken.

	His long, bony fingers resume clicking the keyboard.

					RHINEHEART
			This company is one of the top
			software companies in the world
			because every single employee
			understands that they are a part
			of a whole.  Thus, if an employee
			has a problem, the company has a
			problem.

	He turns again.

					RHINEHEART
			The time has come to make a 
			choice, Mr. Anderson.  Either you
			choose to be at your desk on time
			from this day forth, or you choose
			to find yourself another job.  Do
			I make myself clear?"
        .to_string();

    // To encode you must call the encrypt function and pass the public key, the curve, and the plain text
    let cipher_text = encrypt(&twofivesix.public_key(), &twofivesix.curve(), &plain_text);

    // To decode you call decrypt with the private key, curve, and cypher text. The program will most likely
    //  panic if the wrong private key is used b/c that could lead to a utf8 encoding error. Possibly you will
    //   get text back with the wrong private key but it will not be the original.
    let message = decrypt(&twofivesix.private_key(), twofivesix.curve(), &cipher_text);
    println!("{}", message);
}
