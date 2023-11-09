use crate::data_structures::{GenomeInfo, Manifest, ManifestChromosomeData};
use crate::Options;
use cov_viz_ds::{CoverageData, Facet, FacetRange, FacetRange64};
use exp_viz::{filter_coverage_data, Filter};

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

    // Calculate the correct Significance and Effect Size values to use for the
    // sliders. Since we're only showing the significant REOs by default the slider
    // values should reflect that filtering.
    let mut filter = Filter::new();
    filter.categorical_facets = options.default_facets.clone();
    let filtered_data = filter_coverage_data(&filter, data, None);

    let chromosomes: Vec<ManifestChromosomeData> = filtered_data
        .chromosomes
        .iter()
        .map(|c| ManifestChromosomeData::from(c, filtered_data.bucket_size))
        .collect();

    let facets: Vec<Facet> = data
        .facets
        .iter()
        .map(|f| {
            let mut f = f.clone();
            if f.name == "Significance" {
                f.range64 = Some(FacetRange64(
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
        reo_count: filtered_data.reo_count,
        source_count: filtered_data.sources.len(),
        target_count: filtered_data.targets.len(),
        genome: GenomeInfo {
            file: genome_file.to_string(),
            name: options.genome.clone(),
        },
    })
}
