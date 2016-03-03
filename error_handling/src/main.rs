use std::path::Path;
use std::ffi::OsStr;

use std::num::ParseIntError;
use std::result;

use std::env;

use std::fs::File;
use std::io::Read;

use std::thread;

use std::error;
use std::error::Error;
use std::fmt;

use std::io;
use std::num;

// Demo for Option<T> - much like C++ long awaited std::optional
fn find_needle(haystack: &str, needle: char) ->Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

// that's how Option mapping to custom processing function can be implemened
// btw Option<T>::map() is implemented in a similar way
fn match_map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}

// that's how you typedef in Rust!
type MyResult<T> = result::Result<T, ParseIntError>;
#[allow(dead_code)]
fn double_number(_number_str: &str) -> MyResult<u32> { // compiler doesn't warn about unused pre-underscored variables
    unimplemented!();
}

fn double_arg(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        .ok_or("Provide at least 1 arg".to_owned()) //converts option to result
        .and_then(|arg| arg.parse::<i32>().map_err(|err| err.to_string()))
}

fn file_double_bad<P: AsRef<Path>>(file_path : P) -> i32 {
    let mut file = File::open(file_path).unwrap(); // error 1
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap(); // error 2

    let n: i32 = contents.trim().parse().unwrap(); // error 3
    2 * n
}

fn file_double_comb<P: AsRef<Path>>(file_path : P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2 * n)
}

fn file_double_try<P: AsRef<Path>>(file_path : P) -> Result<i32, String> {
    let mut file = try!(File::open(file_path).map_err(|err| err.to_string()));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|err| err.to_string()));
    let n = try!(contents.trim().parse::<i32>().map_err(|err| err.to_string()));
    Ok(2 * n)
}

// to have a reasonable human readable desc of enum values
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

// implement standard traits:
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // defer to base errors Display implementation
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        match *self {
            // defer to base errors implementation again
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => error::Error::description(err), // has its own descr, calling one from the trait
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // both of them cast concrete error types to basic trait Error object
            // that's why this works
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}

// now we allow to CliError to be converted from both its subtypes
// like c++ implicit conversions using c-tors
impl From<io::Error> for CliError {
    fn from(err: io::Error) ->CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

fn file_double_try2<P: AsRef<Path>>(file_path : P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path).map_err(CliError::Io));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(CliError::Io));
    let n = try!(contents.trim().parse::<i32>().map_err(CliError::Parse));
    Ok(2 * n)
}

fn file_double_try3<P: AsRef<Path>>(file_path : P) -> Result<i32, Box<Error>> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n = try!(contents.trim().parse::<i32>());
    Ok(2 * n)
}

fn file_double_try4<P: AsRef<Path>>(file_path : P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n: i32 = try!(contents.trim().parse());
    Ok(2 * n)
}


fn main() {
    // methods like unwrap(), and_then() etc defined for Option and Result enums are called _combinators_.
    // unwrap() encapsulates Option<T> result case analysis and panics at None
    // use it if 1) it's an example code 2) panic means there's a bug
    // or use expect(), it does the same but prints a message when panics

    // demo how to pattern-match Option<T> function result
    let filename = "rainbow-i_surrender.mp3";
    match find_needle(filename, '.') {
        None    => println!("No file extension"),
        Some(i) => println!("File extension is {}", &filename[i + 1..]),
    }

    // the same can be done by wrapping Option<T>-dealing pattern:
    println!("custom map: {}", (|| match_map(find_needle(filename, '.'), |i| &filename[i + 1..]))().unwrap());

    // and the same with stdlib-provided function:
    println!("std map: {}", find_needle(filename, '.').map(|i| &filename[i + 1..]).unwrap());

    // providing a default in case of None:
    assert_eq!(Path::new("file.mp3").extension().unwrap_or(OsStr::new("flac")), "mp3");
    assert_eq!(Path::new("hellow").extension().unwrap_or(OsStr::new("flac")), "flac");

    // combining Option<T> and Result<T, E>
    match double_arg(env::args()) {
        Ok(n) => println!("1-st arg is a valid i32: {}", n),
        Err(err) => println!("Error occured: {}", err),
    }

    // limits of combinators use cases
    // file IO expample
    // 1.version that panics on error
    let result = thread::spawn(move || file_double_bad("int.txt")).join();
    println!(">> First  file_double() version: {:?} -- isolated in its own thread", result);

    // 2. version that uses combinators to handle errors by casting them to String type arg of result
    let result = file_double_comb("int.txt");
    println!(">> Second file_double() version: {:?}", result);

    // 3. we can also do a manual case analysis and early return on Err.

    // 4. Using a try!() macro that abstracts away early returns
    let result = file_double_try("int.txt");
    println!(">> Third file_double() version: {:?}", result);

    // 5. Tweak it by introducing constom error enumeration
    let result = file_double_try2("int.txt");
    println!(">> Fourth file_double() version: {:?}", result);

    // standard error traits
    // a. Display and Error -- see impl's above

    // b. From trait
    let _string: String = From::from("moster!");
    let _bytes: Vec<u8> = From::from("nebula and");
    let _cow: ::std::borrow::Cow<str> = From::from("red giant");

    // for errors, there is an impl that allows to convert them to boxes
    let io_err: io::Error = io::Error::last_os_error();
    let parse_err: num::ParseIntError = "naaan".parse::<i32>().unwrap_err();

    // compiler thinks of err1 and 2 as completely same types
    let _err1: Box<Error> = From::from(io_err);
    let _err2: Box<Error> = From::from(parse_err);

    // And finally,
    // 6. Getting fid of map_err() calls by using common error type
    // possible since try!() calls From::from on errors
    let result = file_double_try3("int.txt");
    println!(">> Fifth file_double() version: {:?}", result);

    // No, that's not it!
    // Last impl has the problem that returned error type in opaque to the caller.
    // We reimplement it so it returns CliError, but get rid of map_err() by declaring
    // implicit conversions by implementing From trait for enum values
    let result = file_double_try4("int.txt");
    println!(">> Sixth file_double() version: {:?}", result);
}
