use bitdecode::bitdecode::*;

fn main() {
    let bitcode: u8 = 0b00001111;
    match parse_bit_u(&bitcode, "aa bb aa  bb") {
        Ok(capture) => {
            println!("{:?}", capture);
            assert_eq!(capture["a"], 0b0011);
            assert_eq!(capture["b"], 0b0011);
        }
        Err(e) => {
            assert_eq!(e, true);
        }
    }
}

