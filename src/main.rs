// 
// ECC Tester - A program to test ECC codes, written in Rust
//  
extern crate hamming;
use std::process;
use clap::{crate_version,App,Arg};

// Globals
pub static GBL_DEBUG : u32 = 0; // 0=> No Debug; 1=> Major Event Debug ; 2=> All Debug

/// ECC Tester Program
/// * Tests all single bit flips
/// * Tests all two bit, contiguous bit flips
fn main() {    
  let mut num_data_patterns : u32 = 100;
  let mut num_trials: u32 = 100;
  let mut max_flips: u32 = 8;
  let mut is_verbose: bool = false;
  let matches = App::new("ECC Tester")
    .version(crate_version!())
    .author("Written by: Craig Warner")
    .about(
      "A program to test the error correction and detection strength
of ECC codes.  The current version on the program only allows
the testing in a 8 bit Hammings code.  The program lets the 
user specify the number of data patterns to test, the number 
of trials for multi-bit flips, and the maximum number of 
errors to inject.\n"
    )
    .arg(Arg::with_name("PATTERNS")
      .long("patterns")
      .short("p")
      .multiple(true)
      .help("Number of data patterns to test")
      .takes_value(true)
      .default_value("100")
    )
    .arg(Arg::with_name("TRIALS")
      .long("trials")
      .short("t")
      .multiple(true)
      .help("Number of multi-bit flip trials per data pattern")
      .takes_value(true)
      .default_value("100")
    )
    .arg(Arg::with_name("MAX_FLIPS")
      .long("max_flips")
      .short("f")
      .multiple(true)
      .help("Maximum Number of flips tested (only 2-60 are supported)")
      .takes_value(true)
      .default_value("8")
    )
    .arg(Arg::with_name("verbose")
      .long("verbose")
      .short("v")
      .multiple(true)
      .help("verbose")
      .takes_value(false)
    )
    .get_matches();

  if matches.is_present("verbose") {
    is_verbose = true;
  }
  // Argument Parsing: data_patterns
  if let Some(input) = matches.value_of("PATTERNS") {
    match input.parse::<u32>() {
      Ok(n) => {
        if is_verbose {
          println!("Number of Data Patterns = {}", n);
        }
        num_data_patterns = n;
      },
      Err(_n) => {
        eprintln!("Error:Data Pattern Argument is not supported {}",input);
        process::exit(1) 
      }
    }
  }
  // Argument Parsing: Trials 
  if let Some(input) = matches.value_of("TRIALS") {
    match input.parse::<u32>() {
      Ok(n) => {
        if is_verbose {
          println!("Number of Trials per Data Pattern = {}", n);
        }
        num_trials = n;
      },
      Err(_n) => {
        eprintln!("Error:Data Pattern Argument is not supported {}",input);
        process::exit(1) 
      }
    }
  }
  // Argument Parsing: max_num_flips 
  if let Some(input) = matches.value_of("MAX_FLIPS") {
    match input.parse::<u32>() {
      Ok(n) => {
        if (n > 60) || (n<2) {
          eprintln!("Error:Max Flip Argument is not supported {}",input);
          process::exit(1) 
        }
        else {
          if is_verbose {
            println!("Maximum Number of Flips = {}", n);
          }
          max_flips = n;
        }
      },
      Err(_n) => {
        eprintln!("Error:Max Flip Argument is not supported {}",input);
        process::exit(1) 
      }
    }
  }
  analyze_single_bit_flips(num_data_patterns,is_verbose); 
  // Test Multi-bit flips
  // - 2-8 bit flips
  // - Number of trials
  // - Number of data patttens
  for n in 2..(max_flips+1) {  
    analyze_multi_bit_flips(n,num_trials,num_data_patterns,is_verbose);
  }
}

fn report_data_pattern (data_val:u64,is_verbose:bool) {
  if is_verbose {
    println!("Data Pattern 0x{:08x}",data_val)
  }
}

/// Test all Data and ECC single bit flips
/// - Do testing for a specified number of randomly generated data words
fn analyze_single_bit_flips(num_data_patterns:u32,is_verbose:bool) {
  let mut data_val:u64;
  for _i in 0..num_data_patterns {
    data_val = hamming::com::pick_rand_word();
    report_data_pattern(data_val,is_verbose);
    single_bit_flips(data_val)
  }
  if GBL_DEBUG > 0 {
    println!("Debug:Status:Single bit error testing complete")
  }
}

/// Test all Data and ECC single bit flips
/// - All of these cases will be corrected my Hamming Code
fn single_bit_flips(data_val:u64) {
  let ecc_val:u32 = hamming::gen_ecc(data_val);

  // Walk Through each data bit and each ecc bit
  for n in 0..(hamming::DATA_WIDTH+hamming::ECC_WIDTH) {
    // Corrupt
    let (corrupted_data_val,corrupted_ecc_val) = hamming::com::do_bit_flip(data_val,ecc_val,n);
    // Fix up
    let (cor_data_val,_cor_ecc,uncor) = hamming::check_and_correct(corrupted_data_val,corrupted_ecc_val,GBL_DEBUG);
    if uncor {
      eprintln!("Error:HAMMING CODE is broken: n={}: data=0x{:08x} ecc=0x{:02x} corrupted_data=0x{:08x}", n, data_val,ecc_val,corrupted_data_val);
      process::exit(1) 
    }
    else if data_val != cor_data_val {
      eprintln!("Error:HAMMING CODE does not correct data");
      process::exit(1) 
    }
  }
  if GBL_DEBUG > 1 {
    println!("Debug:Status: data_val=0x{:08x}tested all single bit errors",data_val)
  }
}

// Generate a percentage f64 from two u32s 
fn percent_from_u32s (n:u32, all:u32) -> f64 {
  let percent:f64 = ((n as f64) / (all as f64)) * 100.0;
  percent
}

/// Test Randomly selected Multi-bit flips
/// * n = Number of bit flits 
/// * num_trials = Number of random trials performed
fn analyze_multi_bit_flips(n:u32,num_trials:u32,num_data_patterns:u32,is_verbose:bool) {
  let mut array : [u32;3] = [0,0,0];
  let mut data_val:u64;
  for _i in 0..num_data_patterns {
    data_val = hamming::com::pick_rand_word();
    report_data_pattern(data_val,is_verbose);
    for _j in 0..num_trials {
      let class = multi_bit_flips(n,data_val);
      match class { // ENHANCE: Make class enum
        1 => array[0] += 1,
        2 => array[1] += 1,
        3 => array[2] += 1,
        _ => {
          eprintln!("Error:Bad class");
          process::exit(1) 
        }
      }
    }
  }
  let total_trials:u32 = num_trials * num_data_patterns;
  println!("Report:Error Count: n={}", n);
  println!("Report:   Corrected    ={:5} percent={:5.2}%", array[0],percent_from_u32s(array[0],total_trials));
  println!("Report:   Uncorrectable={:5} percent={:5.2}%", array[1],percent_from_u32s(array[1],total_trials));
  println!("Report:   Aliased      ={:5} percent={:5.2}%", array[2],percent_from_u32s(array[2],total_trials));
}

/// Test Randomly selected Multi-bit flips
fn multi_bit_flips(n:u32,data_val:u64) -> u32 {
  let _array : Vec<u32>;
  let _i:u32;
  let mut j:u32;
  let ecc_val:u32 = hamming::gen_ecc(data_val);
  let mut corrupted_data_val:u64 = data_val;
  let mut corrupted_ecc_val:u32 = ecc_val;
  let mut _updated_data_val:u64;
  let mut _updated_ecc_val:u32;
  let mut _cor_data_val:u64;
  let mut _cor_ecc_val:u32;
  let class:u32;

  // Find N Bits to Flip
  let _array = hamming::com::pick_n_unique_rand_nums(n);
  // Flip All N bits
  if GBL_DEBUG > 1 {
    println!("Debug:n={}",n);
    j=0;
    for _i in _array.iter() {
      println!("Debug:element #={},element={}",j,*_i);
      j+=1
    }
  }
  for _i in _array.iter() {
    let (_updated_data_val,_updated_ecc_val) = hamming::com::do_bit_flip(corrupted_data_val,corrupted_ecc_val,*_i);
    corrupted_data_val = _updated_data_val;
    corrupted_ecc_val = _updated_ecc_val;
  }
  // Attempt Fix Up
  let (_cor_data_val,_cor_ecc_val,uncor) = hamming::check_and_correct(corrupted_data_val,corrupted_ecc_val,GBL_DEBUG);
  // Classify into 
  //  1) Corrected
  //  2) Detected as Uncorrectable
  //  3) Aliased to Correctable
  if uncor {
    class = 2;
  }
  else if (_cor_data_val == data_val) && (_cor_ecc_val == ecc_val) {
    class = 1;
  }
  else {
    class = 3;
  }
  // Report Results - Enum : 
  if GBL_DEBUG > 0 {
    println!("Debug:n={},class={}",n,class);
  }
  if GBL_DEBUG > 1 {
    println!("Debug:data=          0x{:08x},ecc=          0x{:02x}",data_val,ecc_val);
    println!("Debug:corrupted_data=0x{:08x},corrupted_ecc=0x{:02x}",corrupted_data_val,corrupted_ecc_val);
    println!("Debug:cor_data=      0x{:08x},cor_ecc=      0x{:02x}",_cor_data_val,_cor_ecc_val)
  }
  class
}