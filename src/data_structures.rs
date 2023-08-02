use cov_viz_ds::{BucketLoc, ChromosomeData, DbID, Facet, Interval};
use rustc_hash::FxHashSet;
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Serialize)]
pub struct ManifestInterval {
    start: u32,
    count: usize,
    #[serde(skip)]
    pub reos: FxHashSet<DbID>,
    #[serde(skip)]
    pub features: FxHashSet<DbID>,
    associated_buckets: Vec<u32>,
    pub min_sig: f32,        // Lower significance values are more significant
    pub max_abs_effect: f32, // largest absolute effect size
}

impl ManifestInterval {
    fn from(inter: &Interval, default_facets: &FxHashSet<DbID>) -> Option<Self> {
        let mut bucket_list = FxHashSet::<BucketLoc>::default();

        let features_iter = inter.values.iter();
        let mut feature_count = 0;
        let mut reos = FxHashSet::<DbID>::default();
        let mut features = FxHashSet::<DbID>::default();
        let mut min_interval_sig = f32::MAX; // the lower the number the greater the significance
        let mut max_interval_effect: f32 = 0.0;
        if default_facets.is_empty() {
            for feature in features_iter {
                feature_count += 1;
                features.insert(feature.id);
                for facets in &feature.facets {
                    reos.insert(facets.reo_id);
                    min_interval_sig = min_interval_sig.min(facets.significance);
                    max_interval_effect = if max_interval_effect.abs() > facets.effect_size.abs() {
                        max_interval_effect
                    } else {
                        facets.effect_size
                    };
                }
                bucket_list.extend(feature.associated_buckets.clone())
            }
        } else {
            // Filter out REOs that don't match a default facet
            for feature in features_iter {
                let mut feature_observed = false;
                for observation in &feature.facets {
                    if observation
                        .facet_ids
                        .iter()
                        .any(|f| default_facets.contains(f))
                    {
                        reos.insert(observation.reo_id);
                        feature_observed = true;
                        min_interval_sig = min_interval_sig.min(observation.significance);
                        max_interval_effect =
                            if max_interval_effect.abs() > observation.effect_size.abs() {
                                max_interval_effect
                            } else {
                                observation.effect_size
                            };
                    }
                }
                if feature_observed {
                    feature_count += 1;
                    features.insert(feature.id);
                    bucket_list.extend(feature.associated_buckets.clone());
                }
            }
        };

        if feature_count == 0 {
            return None;
        }

        Some(ManifestInterval {
            start: inter.start,
            count: feature_count,
            reos,
            features,
            associated_buckets: bucket_list.iter().fold(Vec::<u32>::new(), |mut acc, b| {
                acc.push(b.chrom as u32);
                acc.push(b.idx);
                acc
            }),
            min_sig: min_interval_sig,
            max_abs_effect: max_interval_effect,
        })
    }
}

#[derive(Serialize)]
pub struct ManifestChromosomeData {
    chrom: String,
    bucket_size: u32,
    pub source_intervals: Vec<ManifestInterval>,
    pub target_intervals: Vec<ManifestInterval>,
}

impl ManifestChromosomeData {
    pub fn from(data: &ChromosomeData, default_facets: &FxHashSet<DbID>) -> Self {
        let mut si: Vec<ManifestInterval> = data
            .source_intervals
            .iter()
            .filter_map(|i| ManifestInterval::from(i, default_facets))
            .collect();
        si.sort_by(|a, b| a.start.cmp(&b.start));
        let mut ti: Vec<ManifestInterval> = data
            .target_intervals
            .iter()
            .filter_map(|i| ManifestInterval::from(i, default_facets))
            .collect();
        ti.sort_by(|a, b| a.start.cmp(&b.start));
        ManifestChromosomeData {
            chrom: data.chrom.clone(),
            bucket_size: data.bucket_size,
            source_intervals: si,
            target_intervals: ti,
        }
    }
}

#[derive(Serialize)]
pub struct GenomeInfo {
    pub file: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct Manifest {
    pub chromosomes: Vec<ManifestChromosomeData>,
    pub default_facets: Vec<DbID>,
    pub facets: Vec<Facet>,
    pub reo_count: u64,
    pub source_count: u64,
    pub target_count: u64,
    pub genome: GenomeInfo,
}

impl Manifest {
    pub fn write(&self, out: &PathBuf) {
        match serde_json::to_string(self) {
            Ok(json) => match fs::write(out, json) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
