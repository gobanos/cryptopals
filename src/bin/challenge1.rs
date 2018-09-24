extern crate cryptopals;

use cryptopals::conversion::base64::Base64Write;
use cryptopals::conversion::char::CharWrite;
use cryptopals::conversion::hex::HexRead;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(
        input.hex().to_base64().to_char().collect::<String>(),
        result
    );
}
