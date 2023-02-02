use cov_viz_ds::{Bucket, ChromosomeData, Facet, Interval};
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
    fn from(inter: &Interval) -> Self {
        let mut bucket_list = FxHashSet::<Bucket>::default();
        inter
            .values
            .iter()
            .for_each(|v| bucket_list.extend(v.associated_buckets.iter()));

        ManifestInterval {
            start: inter.start,
            count: inter.values.len(),
            associated_buckets: bucket_list.iter().fold(Vec::<u32>::new(), |mut acc, b| {
                acc.push(b.0 as u32);
                acc.push(b.1);
                acc
            }),
        }
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
    pub fn from(data: &ChromosomeData) -> Self {
        let mut si: Vec<ManifestInterval> = data
            .source_intervals
            .iter()
            .map(|i| ManifestInterval::from(i))
            .collect();
        si.sort_by(|a, b| a.start.cmp(&b.start));
        let mut ti: Vec<ManifestInterval> = data
            .target_intervals
            .iter()
            .map(|i| ManifestInterval::from(i))
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
