use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let mut ignore_case = env::var("IGNORE_CASE").is_ok();

        if args.len() > 3 {
            let arg = args[3].clone();
            let arg = Arg::from_string(arg);
            if arg.group == 2 && arg.val == "IGNORE_CASE".to_string() {
                ignore_case = true;
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
