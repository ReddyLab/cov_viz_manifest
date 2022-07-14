use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Options {
    pub genome: String,
    pub input_location: PathBuf,
    pub output_location: PathBuf,
}

impl Options {
    pub fn get() -> Self {
        let args: Vec<String> = env::args().collect();

        Options {
            genome: args[1].clone(),
            input_location: PathBuf::from(args[2].clone()),
            output_location: [args[3].clone().as_str(), "coverage_manifest.json"]
                .iter()
                .collect(),
        }
    }
}
