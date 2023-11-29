#[derive(Debug)]
pub enum CrustCoreErr {
    RuntimeError,
}
pub type CrustCoreResult = Result<(), CrustCoreErr>;

pub fn run(script: &str) -> CrustCoreResult {
    println!("Src: {script}");
    Ok(())
}

