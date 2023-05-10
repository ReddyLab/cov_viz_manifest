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
}

impl ManifestInterval {
    fn from(inter: &Interval, default_facets: &FxHashSet<DbID>) -> Option<Self> {
        let mut bucket_list = FxHashSet::<BucketLoc>::default();

        let features_iter = inter.values.iter();
        let mut feature_count = 0;
        let mut reos = FxHashSet::<DbID>::default();
        let mut features = FxHashSet::<DbID>::default();
        if default_facets.is_empty() {
            for feature in features_iter {
                feature_count += 1;
                features.insert(feature.id);
                for facets in &feature.facets {
                    reos.insert(facets.reo_id);
                }
                bucket_list.extend(feature.associated_buckets.clone())
            }
        } else {
            // Filter out REOs that don't match a default facet
            for feature in features_iter {
                for facets in &feature.facets {
                    if facets.facet_ids.iter().any(|f| default_facets.contains(f)) {
                        reos.insert(facets.reo_id);
                        feature_count += 1;
                        bucket_list.extend(feature.associated_buckets.clone());
                        features.insert(feature.id);
                        break;
                    }
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
