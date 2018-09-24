extern crate cryptopals;

use cryptopals::bit::XorIter;
use cryptopals::conversion::hex::HexRead;
use cryptopals::conversion::hex::HexWrite;

fn main() {
    let input_a = "1c0111001f010100061a024b53535009181c";
    let input_b = "686974207468652062756c6c277320657965";
    let result = "746865206b696420646f6e277420706c6179";

    assert_eq!(
        input_a
            .hex()
            .xor(input_b.hex())
            .to_hex()
            .collect::<String>(),
        result
    );
}
