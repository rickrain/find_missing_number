use rand::{distributions::Uniform, Rng};
use std::collections::HashMap;

const BIT_ARRAY_SIZE: usize = (u16::MAX as usize + 1) / 8; // 8K

fn generate_random_nums() -> Vec<u16> {
    // Generate 64_000 unique random numbers in the range 0..64K
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, u16::MAX); // 0..64K
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
fn missing_bits(x: u8) -> (u8, u8, u8) {
    let mut total_missing_bits = 0u8;
    let mut first_missing_bit = 0u8;
    let mut last_missing_bit = 0u8;

    let mut test_bit_value = 0b1000_0000u8;
    for bit_pos in (1..=8).rev() {
        if x & test_bit_value == 0 {
            total_missing_bits += 1;
            first_missing_bit = bit_pos;
            if last_missing_bit == 0 {
                last_missing_bit = bit_pos;
            }
        }
        test_bit_value >>= 1;
    }
    (first_missing_bit, last_missing_bit, total_missing_bits)
}

fn print_missing_numbers(bit_array: &[u8; BIT_ARRAY_SIZE]) {
    let mut missing_number_count = 0;
    let mut first_missing_number = 0;

    for (index, x) in bit_array.iter().enumerate() {
        let mut bit_value = 1u8;
        let mut bit_pos = 0;
        while bit_value != 0 {
            if x & bit_value == 0 {
                missing_number_count += 1;
                first_missing_number = ((BIT_ARRAY_SIZE - 1 - index) * 8) + bit_pos;
            }
            bit_value <<= 1;
            bit_pos += 1;
        }
    }

    println!("Missing numbers: {}", missing_number_count);
    println!("First missing number: {}", first_missing_number);
}

fn main() {
    let random_numbers = generate_random_nums();
    let bit_array = convert_to_bit_array(&random_numbers);
    print_bit_array(&bit_array);
    print_missing_numbers(&bit_array);
}

#[cfg(test)]
mod tests {
    use crate::missing_bits;

    #[test]
    fn first_missing_bit_works() {
        assert_eq!((1, 8, 8), missing_bits(0b0000_0000u8));
        assert_eq!((3, 8, 6), missing_bits(0b0000_0011u8));
        assert_eq!((2, 8, 6), missing_bits(0b0001_0001u8));
        assert_eq!((6, 8, 2), missing_bits(0b0101_1111u8));
        assert_eq!((8, 8, 1), missing_bits(0b0111_1111u8));
        assert_eq!((0, 0, 0), missing_bits(0b1111_1111u8));
        assert_eq!((1, 7, 5), missing_bits(0b1001_1000u8));

    }
}

