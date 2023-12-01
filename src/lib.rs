use util::CrustCoreResult;

use crate::scanner::Scanner;

mod scanner;
pub mod util;

pub fn run(script: &str) -> CrustCoreResult<()> {
    println!("Src: {script}");
    let scanner = Scanner::new(script);

    let tokens = scanner.scan_tokens();
    println!("Tokens: {tokens:#?}");
    Ok(())
}
