use cov_viz_ds::{DbID, Facet};
use exp_viz::{FilteredBucket, FilteredChromosome};
use serde::Serialize;
use std::{fs, path::PathBuf};

#[derive(Serialize)]
pub struct ManifestChromosomeData {
    chrom: String,
    bucket_size: u32,
    pub source_intervals: Vec<FilteredBucket>,
    pub target_intervals: Vec<FilteredBucket>,
}

impl ManifestChromosomeData {
    pub fn from(data: &FilteredChromosome, bucket_size: u32) -> Self {
        ManifestChromosomeData {
            chrom: data.chrom.clone(),
            bucket_size,
            source_intervals: data.source_intervals.clone(),
            target_intervals: data.target_intervals.clone(),
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
