mod build;
mod data_structures;
mod options;

use crate::options::Options;
use build::build_manifest;
use cov_viz_ds::CoverageData;

fn main() {
    let options = Options::get();

    let data = CoverageData::deserialize(&options.input_location);
    match data {
        Ok(data) => match build_manifest(&data, &options) {
            Ok(m) => m.write(&options.output_location),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}
