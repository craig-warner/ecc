# ECC Tester
```
ECC Tester 0.1.1
Written by: Craig Warner
A program to test the error correction and detection strength
of ECC codes.  The current version on the program only allows
the testing in a 8 bit Hammings code.  The program lets the 
user specify the number of data patterns to test, the number 
of trials for multi-bit flips, and the maximum number of 
errors to inject.

USAGE:
    ecc [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    verbose

OPTIONS:
    -f, --max_flips <MAX_FLIPS>...    Maximum Number of flips tested (only 2-60 are supported) [default: 8]
    -p, --patterns <PATTERNS>...      Number of data patterns to test [default: 100]
    -t, --trials <TRIALS>...          Number of multi-bit flip trials per data pattern [default: 100]
 '''
