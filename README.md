# simple_getopt

## Name

`simple_getopt` -- Yet Another Getopt Parse Command Options Utility But Easy And Simple - (YAGP).

## Installation

In the Cargo.toml file

    [dependancies]
    simple_getopt = "0.2.1"

In the main.rs file

### To Use

```
use simple_getopt::Getopt;
```

## Description

`simple_getopt` - Parse options on command line by breaking up each of the options for easy usage. This crate easily parse the CLI options and get out of the way of the user.

`simple_getopt` returns a rust hash-map wrapped in a read-only implementation. All options given from the CLI are in two categories:

1. flag/switches without an attached values. These are seen as a `boolean` values.
2. options that has values attached. These can be used as HashMap regular key/value.

It should be emphasised that `simple_getopt` job is to parse (i.e break up nicely) options given from the CLI and allow the user focus on how to use these without cumberasome formatting. The returned options are `READ-ONLY`. So there are no suprises since the user or the program cannot modify the options given from the CLI.

## Example
```
        use getopt::Getopt;
        use std::env;
        ...
```
```
        // using the following from the CLI
        // cargo run -- -a=[1, 2, 3, 4, 5] -b=[6, 7, 8, 9, 10] -c=[11, 12, 13]
        fn main() {
            // All you need. CLI options parsed there.
            let parser = Getopt::std(&std::env::args().collect::<Vec<_>>());

            for val in ["a", "b", "c"].iter() {
                if parser.exists(val) {
                    println!(
                        "{:?}",
                        &parser[val]
                            .chars()
                            .map(|a| u64::from_str(&a.to_string()))
                            .map(|a| if let Ok(a) = a { a } else { 0 })
                            .sum::<u64>()
                    );  // 15, 40, 36
                }
            }
        }
```
```
    use getopt::Getopt;
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    ...

    let parse = Getopt::std(&env::args().collect::<Vec<_>>()); // parsing done.

       // if you have an option -i with a value which is a valid file
       // it reads the file to the standard output device.
       if parse.exists("i") {
           let file = File::open(&parse["i"]).unwrap();
           let buf = BufReader::new(file);
           for line in buf.lines() {
               println!("{}", line.unwrap());
           }
       }
```
```
    use getopt::Getopt;
    use std::env;
    ...
    let parse = Getopt::std(&env::args().collect::<Vec<_>>()); // parsing done.

    // print all keys and values
    println!("{}, {}", parse.get_keys(), parse.get_values());

    // cargo run -- -x=12 -y=24
    println!(
        "{:?}",
        parse["x"].parse::<u64>().unwrap() + parse["y"].parse::<u64>().unwrap()
    ); // 36

    // User need `use std::str::FromStr;` for below code to work
    println!(
        "{}",
        u64::from_str(&parse["x"]).unwrap() + u64::from_str(&parse["y"]).unwrap()
    ); // 36

```

### Simple-Getopt Methods

`simple_getopt` returns a read-only hashmap implementation. So, to use the following functions are provided:

1. std

    > **_pub fn std(arr: &[String]) -> Getopt_**
    >
    > - It is an associated function. It takes, reference to slice of String and return structure Getopt. This is the function used to get all options given from the CLI. The User *DOESN'T* need to skip any of the option including the program name.

           let parser = Getopt::std(&env::args().collect::<Vec<_>>());

2.  get_keys

    > **_pub fn get_keys(&self) -> ArrayList\<String>_**
    >
    > - This function returns collections of *ALL* the flages and options in a key/values options as keys. `ArrayList` is a wrapper on-top of rust vector. It works out of the boxes.

        println!("Keys: {}", parser.get_keys());
        //or
        parser.get_keys().print(); //.print() is from ArrayList

3.  get_values

    > **_pub fn get_values(&self) -> ArrayList\<String>_**
    >
    > - This function returns collections of *ALL* the flages and options in a key/values options as values. `ArrayList` is a wrapper on-top of rust vector. It works out of the boxes.

        println!("Values: {}", parser.get_values());
        //or
        parser.get_values().print(); //.print() is from ArrayList

4.  exists

    > **_pub fn exists(&self, key: &str) -> bool_**
    >
    > - It returns a boolean value of either false or true, if the key supplied is contained as key in the getopt's returned instance for all the CLI options given.


### Limitation by Design
Since simplicity is the core of this crate design, the following design options are followed:

1. The flags or switches are single lettered and uses a single dash like so: `-a` `-b` `-c`. The user cannot use double dashes or more than one letter as flags. i.e `--file` not allowed.

2. The flags *CANNOT* be combined together i.e `-abc` not allowed. Each flag *_MUST_* standalone. This might change in the future releases.

3. options with key/value, also uses both single dash and letter. And the key *_MUST_* be seperated from the value using either an equal sign `=`, or a space ` ` or a colon `:`. Like so: `-f=my_text_file.txt` or `-o:output_file.txt` or `'i input_file.db'`.
_NOTE_: if a space is used to seperate the flag and values, you must quote them accordingly.
The User program will panic with wrong seperator.

#### Alternatives
There are other great getopts in rust crates. Many of them are very verbose, opinionated and powerful but not as easy as `simple_getopt`. If you care to check them up in case you want to customize your CLI options more; `clap`(https://crates.io/crates/clap), `structopt`(https://crates.io/crates/structopt), `argh`(https://crates.io/crates/argh), etc.
