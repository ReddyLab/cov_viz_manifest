use crate::data_structures::{GenomeInfo, Manifest, ManifestChromosomeData};
use crate::Options;
use cov_viz_ds::CoverageData;

pub fn build_manifest(data: &CoverageData, options: &Options) -> Result<Manifest, std::io::Error> {
    let genome_file = match options.genome.as_str() {
        "GRCH38" => "grch38.json",
        "GRCH37" => "grch37.json",
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Invalid genome {}", options.genome),
            ))
        }
    };

    Ok(Manifest {
        chromosomes: data
            .chromosomes
            .iter()
            .map(|c| ManifestChromosomeData::from(c))
            .collect(),
        facets: data.facets.clone(),
        genome: GenomeInfo {
            file: genome_file.to_string(),
            name: options.genome.clone(),
        },
    })
}
