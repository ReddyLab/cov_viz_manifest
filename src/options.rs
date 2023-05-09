use std::env;
use std::path::PathBuf;

use cov_viz_ds::DbID;
use rustc_hash::FxHashSet;

#[derive(Debug)]
pub struct Options {
    pub genome: String,
    pub input_location: PathBuf,
    pub output_location: PathBuf,
    pub default_facets: FxHashSet<DbID>,
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
            default_facets: if args.len() > 4 {
                args[4..]
                    .iter()
                    .filter_map(|x| DbID::from_str_radix(x, 10).ok())
                    .collect()
            } else {
                FxHashSet::default()
            },
        }
    }
}
