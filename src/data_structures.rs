use cov_viz_ds::{Bucket, ChromosomeData, DbID, Facet, Interval};
use rustc_hash::FxHashSet;
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Serialize)]
struct ManifestInterval {
    start: u32,
    count: usize,
    associated_buckets: Vec<u32>,
}

impl ManifestInterval {
    fn from(inter: &Interval, default_facets: &FxHashSet<DbID>) -> Option<Self> {
        let mut bucket_list = FxHashSet::<Bucket>::default();

        // Filter out reg effects that don't match a default facet
        let values_iter = inter.values.iter();
        let mut value_count = 0;
        if default_facets.is_empty() {
            for value in values_iter {
                value_count += 1;
                bucket_list.extend(value.associated_buckets.clone())
            }
        } else {
            for value in values_iter {
                for facets in &value.facets {
                    if facets.0.iter().any(|f| default_facets.contains(f)) {
                        value_count += 1;
                        bucket_list.extend(value.associated_buckets.clone());
                        break;
                    }
                }
            }
        };

        if value_count == 0 {
            return None;
        }

        Some(ManifestInterval {
            start: inter.start,
            count: value_count,
            associated_buckets: bucket_list.iter().fold(Vec::<u32>::new(), |mut acc, b| {
                acc.push(b.0 as u32);
                acc.push(b.1);
                acc
            }),
        })
    }
}

#[derive(Serialize)]
pub struct ManifestChromosomeData {
    chrom: String,
    bucket_size: u32,
    source_intervals: Vec<ManifestInterval>,
    target_intervals: Vec<ManifestInterval>,
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
