use std::io::Read;
use std::borrow::Borrow;

const CONFIG_PATH: &'static str = "/etc/httpd.conf";

const CPU_LIMIT: &'static str = "cpu_limit";
const THREAD_LIMIT: &'static str = "thread_limit";
const DOCUMENT_ROOT: &'static str = "document_root";

const DEFAULT_CPU: u8 = 1;
const DEFAULT_THREAD: u16 = 1;
pub const DEFAULT_ROOT: &'static str = "/";

#[derive(Debug)]
pub struct Config {
    pub cpu_limit: u8,
    pub thread_limit: u16,
    pub document_root: String,
}

impl Config {
    pub fn new() -> Option<Config> {
        let mut file = std::fs::File::open(CONFIG_PATH).ok()?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).ok()?;
        let mut splitted = buf.split("\n");

        let mut config = Config {
            cpu_limit: DEFAULT_CPU,
            thread_limit: DEFAULT_THREAD,
            document_root: DEFAULT_ROOT.to_string(),
        };

        for line in splitted {
            if line.starts_with(CPU_LIMIT) {
                let mut string = line.split(" ");
                string.next()?;
                config.cpu_limit = string.next()?.parse().unwrap_or(DEFAULT_CPU);
            }

            if line.starts_with(THREAD_LIMIT) {
                let mut string = line.split(" ");
                string.next()?;
                config.thread_limit = string.next()?.parse().unwrap_or(DEFAULT_THREAD);
            }

            if line.starts_with(DOCUMENT_ROOT) {
                let mut string = line.split(" ");
                string.next()?;
                config.document_root = string.next().unwrap_or(DEFAULT_ROOT).to_string();
            }
        };

        Some(config)
    }
}