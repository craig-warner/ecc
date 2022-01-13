
extern crate self as hamming;

// Constants
pub const DATA_WIDTH : u32 = 64;
pub const ECC_WIDTH : u32 = 8;

/// The ham module supports a 8 bit hamming function

/// Generates a 8 bit Hamming code to the 64 bits of data in data_word.
pub fn gen_ecc(data_word:u64)->u32 {
  let mut h_matrix_slice:u64;
  let mut ecc_bit:u32;
  let mut ecc:u32;

  ecc=0;
  for n in 0..8 {
    match n {
    0=> h_matrix_slice =  0b1010011010010101010110010101010101010101011001010101011010101010,
    1=> h_matrix_slice =  0b1010011010010101010110101010101010101010101010101010101001100101,
    2=> h_matrix_slice =  0b1010011010101010101010010101010110101010101010101010101001100101,
    3=> h_matrix_slice =  0b0101100101011010101001011010101001010101101010101010101001101010,
    4=> h_matrix_slice =  0b0101101010100110101010100110101001101010010101011010100110010110,
    5=> h_matrix_slice =  0b1010100101101001101010101001101010011010010110100101100110101001,
    6=> h_matrix_slice =  0b0110100110101010011001101010011010100110100101100110011010011010,
    7=> h_matrix_slice =  0b1001101001101010100101101010100110101001101010011001010110011010,
    _ => {
	    h_matrix_slice =  0b0;
	    println!("Program Bug")
      }
    }
    ecc_bit = gen_parity(data_word & h_matrix_slice);
    ecc = (ecc<<1) | ecc_bit;
  }
  return ecc
}

/// Generates a partity for tthe 64 bits of data in data_word.
/// If there are an odd number of bits set in the 64 bits of data_word
/// then the function returns 1, otherwise the function returns 0.
fn gen_parity(data_word:u64)->u32 {

  let mut parity:u32;

  parity = 0;
  for n in 0..hamming::DATA_WIDTH {
    if (data_word & (0x1 <<n)) == 0 {
      parity ^= 0;
    } 
    else {
      parity ^= 1;
    }
  }
  return parity
}

/// Check and Corrects the data and ecc for the hamming code protected word.
/// returns:
/// * corrected data
/// * corrected ecc 
/// * indication of uncorrectable ecc error
  pub fn check_and_correct(data_word:u64, ecc_word:u32, debug:u32) -> (u64,u32,bool) {
    let cor_data: u64;
    let cor_ecc: u32;
    let bit_to_fix : u64;
    let ecc_bit_to_fix : u32;
    let uncorrectable: bool;

    // Generate New ECC
    let ecc_val:u32 = gen_ecc(data_word);
    // Calculate Syndrome : XOR regenerated ECC with original ECC
    let syndrome:u32 = ecc_val ^ ecc_word;

    // Detect if Syndrome indicates Data bits should be fixed
    match syndrome {
    0b11100101 => bit_to_fix = 0b1000000000000000000000000000000000000000000000000000000000000000,
    0b00011010 => bit_to_fix = 0b0100000000000000000000000000000000000000000000000000000000000000,
    0b11100110 => bit_to_fix = 0b0010000000000000000000000000000000000000000000000000000000000000,
    0b00011001 => bit_to_fix = 0b0001000000000000000000000000000000000000000000000000000000000000,
    0b00011111 => bit_to_fix = 0b0000100000000000000000000000000000000000000000000000000000000000,
    0b11100000 => bit_to_fix = 0b0000010000000000000000000000000000000000000000000000000000000000,
    0b11101001 => bit_to_fix = 0b0000001000000000000000000000000000000000000000000000000000000000,
    0b00010110 => bit_to_fix = 0b0000000100000000000000000000000000000000000000000000000000000000,
    0b11101010 => bit_to_fix = 0b0000000010000000000000000000000000000000000000000000000000000000,
    0b00010101 => bit_to_fix = 0b0000000001000000000000000000000000000000000000000000000000000000,
    0b00101111 => bit_to_fix = 0b0000000000100000000000000000000000000000000000000000000000000000,
    0b11010000 => bit_to_fix = 0b0000000000010000000000000000000000000000000000000000000000000000,
    0b00110111 => bit_to_fix = 0b0000000000001000000000000000000000000000000000000000000000000000,
    0b11001000 => bit_to_fix = 0b0000000000000100000000000000000000000000000000000000000000000000,
    0b00111011 => bit_to_fix = 0b0000000000000010000000000000000000000000000000000000000000000000,
    0b11000100 => bit_to_fix = 0b0000000000000001000000000000000000000000000000000000000000000000,
    0b00111101 => bit_to_fix = 0b0000000000000000100000000000000000000000000000000000000000000000,
    0b11000010 => bit_to_fix = 0b0000000000000000010000000000000000000000000000000000000000000000,
    0b00111110 => bit_to_fix = 0b0000000000000000001000000000000000000000000000000000000000000000,
    0b11000001 => bit_to_fix = 0b0000000000000000000100000000000000000000000000000000000000000000,
    0b11101100 => bit_to_fix = 0b0000000000000000000010000000000000000000000000000000000000000000,
    0b00010011 => bit_to_fix = 0b0000000000000000000001000000000000000000000000000000000000000000,
    0b01001111 => bit_to_fix = 0b0000000000000000000000100000000000000000000000000000000000000000,
    0b10110000 => bit_to_fix = 0b0000000000000000000000010000000000000000000000000000000000000000,
    0b01010111 => bit_to_fix = 0b0000000000000000000000001000000000000000000000000000000000000000,
    0b10101000 => bit_to_fix = 0b0000000000000000000000000100000000000000000000000000000000000000,
    0b01011011 => bit_to_fix = 0b0000000000000000000000000010000000000000000000000000000000000000,
    0b10100100 => bit_to_fix = 0b0000000000000000000000000001000000000000000000000000000000000000,
    0b01011101 => bit_to_fix = 0b0000000000000000000000000000100000000000000000000000000000000000,
    0b10100010 => bit_to_fix = 0b0000000000000000000000000000010000000000000000000000000000000000,
    0b01011110 => bit_to_fix = 0b0000000000000000000000000000001000000000000000000000000000000000,
    0b10100001 => bit_to_fix = 0b0000000000000000000000000000000100000000000000000000000000000000,
    0b01100111 => bit_to_fix = 0b0000000000000000000000000000000010000000000000000000000000000000,
    0b10011000 => bit_to_fix = 0b0000000000000000000000000000000001000000000000000000000000000000,
    0b01101011 => bit_to_fix = 0b0000000000000000000000000000000000100000000000000000000000000000,
    0b10010100 => bit_to_fix = 0b0000000000000000000000000000000000010000000000000000000000000000,
    0b01101101 => bit_to_fix = 0b0000000000000000000000000000000000001000000000000000000000000000,
    0b10010010 => bit_to_fix = 0b0000000000000000000000000000000000000100000000000000000000000000,
    0b01101110 => bit_to_fix = 0b0000000000000000000000000000000000000010000000000000000000000000,
    0b10010001 => bit_to_fix = 0b0000000000000000000000000000000000000001000000000000000000000000,
    0b01110011 => bit_to_fix = 0b0000000000000000000000000000000000000000100000000000000000000000,
    0b10001100 => bit_to_fix = 0b0000000000000000000000000000000000000000010000000000000000000000,
    0b11110001 => bit_to_fix = 0b0000000000000000000000000000000000000000001000000000000000000000,
    0b00001110 => bit_to_fix = 0b0000000000000000000000000000000000000000000100000000000000000000,
    0b01110101 => bit_to_fix = 0b0000000000000000000000000000000000000000000010000000000000000000,
    0b10001010 => bit_to_fix = 0b0000000000000000000000000000000000000000000001000000000000000000,
    0b01110110 => bit_to_fix = 0b0000000000000000000000000000000000000000000000100000000000000000,
    0b10001001 => bit_to_fix = 0b0000000000000000000000000000000000000000000000010000000000000000,
    0b01111001 => bit_to_fix = 0b0000000000000000000000000000000000000000000000001000000000000000,
    0b10000110 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000100000000000000,
    0b01111010 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000010000000000000,
    0b10000101 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000001000000000000,
    0b01111100 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000100000000000,
    0b10000011 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000010000000000,
    0b11110010 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000001000000000,
    0b00001101 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000100000000,
    0b10001111 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000010000000,
    0b01110000 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000001000000,
    0b11110100 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000000100000,
    0b00001011 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000000010000,
    0b10010111 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000000001000,
    0b01101000 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000000000100,
    0b10011011 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000000000010,
    0b01100100 => bit_to_fix = 0b0000000000000000000000000000000000000000000000000000000000000001,
    _ => bit_to_fix = 0b0
    }

    // Detect if Syndrome indicates ECC bits should be fixed
    match syndrome {
    0b10000000 => ecc_bit_to_fix = 0b10000000,
    0b01000000 => ecc_bit_to_fix = 0b01000000,
    0b00100000 => ecc_bit_to_fix = 0b00100000,
    0b00010000 => ecc_bit_to_fix = 0b00010000,
    0b00001000 => ecc_bit_to_fix = 0b00001000,
    0b00000100 => ecc_bit_to_fix = 0b00000100,
    0b00000010 => ecc_bit_to_fix = 0b00000010,
    0b00000001 => ecc_bit_to_fix = 0b00000001,
    _ => ecc_bit_to_fix= 0b0
    }
    

    match syndrome {
      // No Correction Required
      0 =>  {
        cor_data = data_word;
        cor_ecc = ecc_word;
        uncorrectable = false
      }
      _ => {
        // Multi-bit Error Detected
        if (bit_to_fix == 0) && (ecc_bit_to_fix == 0) {
          cor_data = data_word;
          cor_ecc = ecc_word;
          uncorrectable = true 
        }
        // ECC bit needs to be Correced
        else if (bit_to_fix == 0) && (ecc_bit_to_fix != 0) {
          cor_data = data_word;
          cor_ecc = ecc_word ^ ecc_bit_to_fix;
          uncorrectable = false 
        }
        // Data bit needs to be Correced
        else {
          cor_data = data_word ^ bit_to_fix;
          cor_ecc = ecc_word;
          uncorrectable = false 
        }
      }
    }
    if debug > 1 {
      println!("Debugging: cor_data=0x{:08x} cor_ecc=0x{:02x} uncorrectable={} syndrome=0x{:02x}", cor_data,cor_ecc,uncorrectable,syndrome);
    }
    (cor_data,cor_ecc,uncorrectable)
  }

pub mod com {
  /// 
  /// Flips a bit in the data or ecc range 
  /// 
  pub fn do_bit_flip(data:u64,ecc:u32,bit:u32)-> (u64,u32) {
    let new_data:u64;
    let new_ecc:u32;

    if bit >= (hamming::DATA_WIDTH+hamming::ECC_WIDTH) {
      new_data = data;
      new_ecc = ecc
    }
    else if bit < hamming::DATA_WIDTH {
      new_data = data ^ (0x1 << bit);
      new_ecc = ecc
    }
    else {
      new_data = data;
      new_ecc = ecc ^ (0x1 << (bit-hamming::DATA_WIDTH))
    }
    //println!("Debugging: data=0x{:08x} ecc=0x{:02x}", data,ecc);
    //println!("Debugging: new_data=0x{:08x} new_ecc=0x{:02x}", new_data,new_ecc);
    (new_data,new_ecc)
  }

  ///
  /// Picks a Random value between 0 and 71 (0-(64+8-1)
  ///
  fn pick_rand_num()-> u32 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    //rint = rng.gen_range(0..(hamming::DATA_WIDTH+hamming::ECC_WIDTH));
    let rint = rng.gen_range(0..72);
    rint
  }

  /// 
  /// Pick N unique random numbers
  /// 
  pub fn pick_n_unique_rand_nums(n:u32)-> Vec<u32> {
    let mut array = Vec::new();
    let mut cur_rand_num: u32;
    let mut total: u32;
    let mut matches: u32;

    // Check Function Argument Condition
    //println!("n={}",n);
    assert!((n>1), "n>1 wasn't true!");
    // Load Vector with first Random number selected
    cur_rand_num =pick_rand_num(); 
    array.push(cur_rand_num);
    total = 1;
    // Add Elements 2 though N
    'find_element:loop {
      'rand_try:loop {
        cur_rand_num =pick_rand_num(); 
        matches = 0;
        for i in array.iter() {
          if cur_rand_num == *i {
            matches += 1;
          }
        }
        if matches == 0 { break'rand_try; }
      }
      array.push(cur_rand_num);
      total = total + 1;
      if total == n { break'find_element; }
    }
    // Return N Unique Elements
    array
  }

  ///
  /// Picks a Random word (64 bits)
  ///
  pub fn pick_rand_word()-> u64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let rint = rng.gen::<u64>();
    rint
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_gen() {
    println!("Testing ECC Gen");
    let data_val:u64 = 0x0000_1111_2222_3333;
    let ecc_val:u32 = hamming::gen_ecc(data_val);
    println!("data 0x{:x} ecc 0x{:x}",data_val,ecc_val);
    assert_eq!(ecc_val,0xf6);
  }
  #[test]
  fn test_flip_data() {
    // Flip Data MSB
    let data_val:u64 = 0x0000_1111_2222_3333;
    let ecc_val:u32 = hamming::gen_ecc(data_val);
    let (new_data_val,new_ecc_val) = hamming::com::do_bit_flip(data_val,ecc_val,63);
    assert_eq!(new_data_val,0x8000_1111_2222_3333);
    assert_eq!(new_ecc_val,0xf6);
    let (new_data_val,new_ecc_val) = hamming::com::do_bit_flip(new_data_val,new_ecc_val,0);
    assert_eq!(new_data_val,0x8000_1111_2222_3332);
    assert_eq!(new_ecc_val,0xf6)
  }
  #[test]
  fn test_pick_pick_n_unique_rand_nums() {  
    let mut array : Vec<u32>;
    array = hamming::com::pick_n_unique_rand_nums(3);
    assert_eq!(array.len(),3);
    array = hamming::com::pick_n_unique_rand_nums(9);
    assert_eq!(array.len(),9)
  }
}
