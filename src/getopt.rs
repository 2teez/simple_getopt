#![allow(unused, dead_code)]
use arraylist::arl::ArrayList;

use std::fmt;
use std::ops::Index;

pub struct Getopt {
    hash: std::collections::HashMap<String, String>,
}

impl Getopt {
    pub fn std(arr: &[String]) -> Self {
        let mut hash = std::collections::HashMap::<String, String>::new();

        let arr = ArrayList::from_slice(arr);

        if arr.len() == 1 {
            println!("Empty...");
            help("No arugment to parse");
        }
        for val in arr.into_iter().skip(1) {
            if !val.starts_with('-') {
                help(&val);
            } else if val.len() > 2 {
                let value = val
                    .chars()
                    .collect::<ArrayList<_>>()
                    .index_in(2)
                    .expect("seperating value not avaliable");
                if value != ' ' && value != ':' && value != '=' {
                    help(&val);
                } else {
                    let data = val.split(value).collect::<ArrayList<_>>();
                    hash.insert(
                        data.index_in(0).expect("key not avaliable")[1..].to_string(),
                        data.index_in(1).expect("value not avaliable").to_string(),
                    );
                }
            } else {
                hash.insert(val[1..].to_string(), "true".to_string());
            }
        }
        Getopt { hash }
    }

    pub fn get_keys(&self) -> ArrayList<String> {
        self.hash
            .keys()
            .into_iter()
            .map(|a| a.to_string())
            .collect::<ArrayList<_>>()
    }
    pub fn get_values(&self) -> ArrayList<String> {
        self.hash
            .values()
            .into_iter()
            .map(|a| a.to_string())
            .collect::<ArrayList<_>>()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.hash.contains_key(key)
    }
}

fn help(item: &str) {
    println!("<{}> is an invalid argument.\n", item);
    let msg = "\t\tUsage: cargo run -- -[flag] -[flag<=| |:>values]\n\n\
    \t\tUsage: ./simple-getopts -[flag] -[flag<=| |:values>]\n\n\
    \t\tEvery flag/switches starts with a dash '-'. Values, however are preceded with a flag, seperated \n
    \t\twith either an equal sign, a space or a colon. like so: -f:file.txt or -f=file.txt or '-f file.txt'.\n
    \t\tif you use a space to seperate your flag and values, you must quote accordingly.\n";
    print!("{}", msg);
    std::process::exit(-1);
}

impl Index<&str> for Getopt {
    type Output = String;
    fn index(&self, id: &str) -> &Self::Output {
        &self.hash[id]
    }
}

impl fmt::Debug for Getopt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.hash)
    }
}
