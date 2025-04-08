use anyhow;
use phf_codegen;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

// This enum is only used for building the final phf map
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Transcription {
    Single(String),
    Multiple(HashMap<String, String>),
}

pub fn create_phf_map(data_path: &PathBuf) -> anyhow::Result<phf_codegen::Map<String>> {
    // deserialize data
    let data_reader = BufReader::new(File::open(data_path)?);
    let data: HashMap<String, Transcription> = serde_json::from_reader(data_reader)?;

    // build phf map
    let mut phf_builder = phf_codegen::Map::<String>::new();

    for (graphemes, transcription) in data.iter() {
        match transcription {
            Transcription::Single(p) => {
                phf_builder.entry(
                    graphemes.clone(),
                    format!("::phonemizer_common::TranscriptionEntry::Single(\"{}\")", p).as_str(),
                );
            }

            Transcription::Multiple(map) => {
                let mut inner_phf_builder = phf_codegen::Map::<&str>::new();
                for (variant, phonemes) in map.iter() {
                    inner_phf_builder.entry(variant, format!("\"{}\"", phonemes).as_str());
                }
                phf_builder.entry(
                    graphemes.clone(),
                    format!(
                        "::phonemizer_common::TranscriptionEntry::Multiple({})",
                        inner_phf_builder.build()
                    )
                    .as_str(),
                );
            }
        };
    }

    Ok(phf_builder)
}
