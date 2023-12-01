use std::{env::args, fs};

mod err {
    use std::io;

    #[derive(Debug)]
    pub enum CrustErr {
        IoError(io::Error),
        CoreError(crust::util::CrustCoreErr),
    }
    pub type CrustResult = Result<(), CrustErr>;

    impl From<io::Error> for CrustErr {
        fn from(err: io::Error) -> CrustErr {
            CrustErr::IoError(err)
        }
    }
    impl From<crust::util::CrustCoreErr> for CrustErr {
        fn from(err: crust::util::CrustCoreErr) -> CrustErr {
            CrustErr::CoreError(err)
        }
    }
}

fn main() {
    println!("Hello from Crust!");
    let args = args().collect::<Vec<String>>();

    match &args[..] {
        [_] => run_prompt(),
        [_, path] => run_file(path),
        _ => panic!(),
    }
    .unwrap();
}

fn run_file(path: &str) -> err::CrustResult {
    let script = fs::read_to_string(path)?;
    crust::run(&script).map_err(|err| err.into())
}

fn run_prompt() -> err::CrustResult {
    let stdin = std::io::stdin();
    loop {
        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(_) => {
                if line.starts_with("exit") {
                    break Ok(());
                } else {
                    crust::run(&line)?;
                }
            }
            Err(_) => todo!(),
        }
    }
}
