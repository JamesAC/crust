#[derive(Debug)]
pub enum CrustCoreErr {
    Multi { errors: Vec<CrustCoreErr> },
    Scan { line: usize, message: String },
    Runtime,
}

pub type CrustCoreResult<T> = Result<T, CrustCoreErr>;
