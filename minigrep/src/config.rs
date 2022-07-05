use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not receive query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not receive filename"),
        };

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        for arg in args {
            let arg = Arg::from_string(arg);
            if arg.group == 2 {
                if arg.val == "IGNORE_CASE".to_string() {
                    ignore_case = true;
                }
            }
        }

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

struct Arg {
    group: usize,
    val: String,
}

impl Arg {
    fn new() -> Arg {
        Arg {
            group: 0,
            val: String::new(),
        }
    }

    fn from_string(arg: String) -> Arg {
        let mut chars = arg.chars();
        let mut arg = Arg::new();

        while let Some(ch) = chars.next() {
            if ch == '-' {
                arg.group += 1;
            } else {
                arg.val.push(ch);
                break;
            }
        }
        arg.val.push_str(&chars.as_str());

        arg
    }
}
