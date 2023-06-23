use crate::data_structures::{GenomeInfo, Manifest, ManifestChromosomeData};
use crate::Options;
use cov_viz_ds::{CoverageData, DbID, Facet, FacetRange};
use exp_viz::{filter_coverage_data, Filter};
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

    // Calculate the correct Significance and Effect Size values to use for the
    // sliders. Since we're only showing the significant REOs by default the slider
    // values should reflect that filtering.
    let mut filter = Filter::new();
    filter.categorical_facets = options.default_facets.clone();
    let filtered_data = filter_coverage_data(&filter, data);
    let facets: Vec<Facet> = data
        .facets
        .iter()
        .map(|f| {
            let mut f = f.clone();
            if f.name == "Significance" {
                f.range = Some(FacetRange(
                    filtered_data.numeric_intervals.sig.0,
                    filtered_data.numeric_intervals.sig.1,
                ));
            } else if f.name == "Effect Size" {
                f.range = Some(FacetRange(
                    filtered_data.numeric_intervals.effect.0,
                    filtered_data.numeric_intervals.effect.1,
                ));
            };
            f
        })
        .collect();

    Ok(Manifest {
        chromosomes,
        default_facets: options.default_facets.clone().into_iter().collect(),
        facets,
        reo_count: reos.len() as u64,
        source_count: sources.len() as u64,
        target_count: targets.len() as u64,
        genome: GenomeInfo {
            file: genome_file.to_string(),
            name: options.genome.clone(),
        },
    })
}
