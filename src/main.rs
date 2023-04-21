use rand::{distributions::Uniform, Rng};
use std::collections::HashMap;

const BIT_ARRAY_SIZE: usize = (u16::MAX as usize + 1) / 8; // 8192

fn generate_random_nums() -> Vec<u16> {
    // Generate 64_000 unique random numbers in the range 0..65_535
    let mut rng = rand::thread_rng();
    let range = Uniform::new_inclusive(0, u16::MAX);
    let mut num_map = HashMap::<u16, u16>::new();

    let random_numbers: Vec<u16> = (0..64_000)
        .map(|_| {
            let mut seen = true;
            let mut num = 0u16;

            // Keep sampling until we get a number we've not already seen
            while seen {
                num = rng.sample(range);
                let occurrencs = num_map.entry(num).or_insert(0);
                *occurrencs += 1;
                seen = *occurrencs > 1;
            }
            num
        })
        .collect();

    random_numbers
}

fn convert_to_bit_array(random_numbers: &Vec<u16>) -> [u8; BIT_ARRAY_SIZE] {
    let mut bit_array: [u8; BIT_ARRAY_SIZE] = [0u8; BIT_ARRAY_SIZE]; // 8K

    for x in random_numbers {
        let byte_index: usize = *x as usize / 8;
        let random_num_bit = 1u8 << (*x as usize - (byte_index * 8));

        bit_array[BIT_ARRAY_SIZE - 1 - byte_index] |= random_num_bit;
    }

    bit_array
}

fn print_bit_array(bit_array: &[u8; BIT_ARRAY_SIZE]) {
    let mut row = 0;
    let mut col = 0u8;
    for x in bit_array {
        // print 8 bytes (64 bits) per row
        if col % 8 == 0 {
            print!("\n{:04}: ", row);
            col = 0;
            row += 1;
        }
        col += 1;

        print!("{:08b} ", x); // print the byte in binary form
    }

    println!();
}

// Locates bits in a given byte that are not set (ie: 0).
// Result is a tuple of 3 u8's, where
//   .0 == first missing bit
//   .1 == last missing bit
//   .2 == total_missing bits
fn missing_bits(x: u8) -> (Option<u8>, Option<u8>, u8) {
    let mut total_missing_bits = 0u8;
    let mut first_missing_bit: Option<u8> = None;
    let mut last_missing_bit: Option<u8> = None;

    let mut test_bit_value = 0b1000_0000u8;
    for bit_pos in (1..=8).rev() {
        if x & test_bit_value == 0 {
            first_missing_bit = Some(bit_pos);
            if last_missing_bit.is_none() {
                last_missing_bit = Some(bit_pos);
            }
            total_missing_bits += 1;
        }
        test_bit_value >>= 1;
    }
    (first_missing_bit, last_missing_bit, total_missing_bits)
}

fn missing_numbers(bit_array: &[u8]) -> (usize, usize, usize) {
    let mut first_missing_number = 0usize;
    let mut last_missing_number = 0usize;
    let mut total_missing_numbers = 0usize;

    for (index, x) in bit_array.iter().enumerate() {
        let missing_bits = missing_bits(*x);
        if let Some(num) = missing_bits.0 {
            first_missing_number = ((bit_array.len() - 1 - index) * 8) + num as usize;
        }
        if let Some(num) = missing_bits.1 {
            if last_missing_number == 0 {
                last_missing_number = ((bit_array.len() - 1 - index) * 8) + num as usize;
            }
        }

        total_missing_numbers += missing_bits.2 as usize;
    }

    (
        first_missing_number,
        last_missing_number,
        total_missing_numbers,
    )
}

fn print_missing_numbers(bit_array: &[u8]) {
    let missing_numbers = missing_numbers(bit_array);

    println!("First missing number: {}", missing_numbers.0);
    println!("Last missing number: {}", missing_numbers.1);
    println!("Total missing numbers: {}", missing_numbers.2);
}

fn main() {
    let random_numbers = generate_random_nums();
    let bit_array = convert_to_bit_array(&random_numbers);
    print_bit_array(&bit_array);
    print_missing_numbers(&bit_array);
}

#[cfg(test)]
mod tests {
    use crate::{missing_bits, missing_numbers};

    #[test]
    fn missing_bits_works() {
        assert_eq!((Some(1), Some(8), 8), missing_bits(0b0000_0000u8));
        assert_eq!((Some(3), Some(8), 6), missing_bits(0b0000_0011u8));
        assert_eq!((Some(2), Some(8), 6), missing_bits(0b0001_0001u8));
        assert_eq!((Some(6), Some(8), 2), missing_bits(0b0101_1111u8));
        assert_eq!((Some(8), Some(8), 1), missing_bits(0b0111_1111u8));
        assert_eq!((None, None, 0), missing_bits(0b1111_1111u8));
        assert_eq!((Some(1), Some(7), 5), missing_bits(0b1001_1000u8));
        assert_eq!((Some(4), Some(6), 3), missing_bits(0b1100_0111u8));
    }

    #[test]
    fn missing_numbers_works() {
        assert_eq!(
            (6, 14, 5),
            missing_numbers(&[0b1111_1111, 0b1100_0111, 0b0101_1111])
        );
        assert_eq!(
            (17, 24, 3),
            missing_numbers(&[0b0111_1100, 0b1111_1111, 0b1111_1111])
        );
        assert_eq!(
            (0, 0, 0),
            missing_numbers(&[0b1111_1111, 0b1111_1111, 0b1111_1111])
        );
        assert_eq!(
            (1, 24, 5),
            missing_numbers(&[0b0111_1111, 0b1010_1011, 0b1111_1110])
        );
        assert_eq!(
            (13, 13, 1),
            missing_numbers(&[0b1111_1111, 0b1110_1111, 0b1111_1111])
        );
    }
}
