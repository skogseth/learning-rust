#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Os {
    Darwin,
    Linux,
    Windows,
}

impl Os {
    pub fn current() -> Self {
        Os::from(std::env::consts::OS)
    }
}

impl From<&str> for Os {
    fn from(s: &str) -> Self {
        match s {
            "macos" => Os::Darwin,
            "linux" => Os::Linux,
            "windows" => Os::Windows,
            _ => panic!("unknow os!"),
        }
    }
}

