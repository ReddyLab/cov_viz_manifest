use crate::data_structures::{GenomeInfo, Manifest, ManifestChromosomeData};
use crate::Options;
use cov_viz_ds::{CoverageData, DbID};
use rustc_hash::FxHashSet;

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

    let chromosomes: Vec<ManifestChromosomeData> = data
        .chromosomes
        .iter()
        .map(|c| ManifestChromosomeData::from(c, &options.default_facets))
        .collect();

    let mut reos = FxHashSet::<DbID>::default();
    let mut sources = FxHashSet::<DbID>::default();
    let mut targets = FxHashSet::<DbID>::default();
    for chrom in &chromosomes {
        for interval in &chrom.source_intervals {
            reos.extend(interval.reos.clone());
            sources.extend(interval.features.clone())
        }
        for interval in &chrom.target_intervals {
            reos.extend(interval.reos.clone());
            targets.extend(interval.features.clone())
        }
    }

    Ok(Manifest {
        chromosomes,
        default_facets: options.default_facets.clone().into_iter().collect(),
        facets: data.facets.clone(),
        reo_count: reos.len() as u64,
        source_count: sources.len() as u64,
        target_count: targets.len() as u64,
        genome: GenomeInfo {
            file: genome_file.to_string(),
            name: options.genome.clone(),
        },
    })
}
