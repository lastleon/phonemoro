use phonemizer_common::{TranscriptionDict, TranscriptionEntry, TranscriptionLookup};
use phonemizer_macros::phm_from_json;

pub struct TranscriptionStoreEN {
    dicts: [TranscriptionDict; 2usize],
}

impl TranscriptionLookup for TranscriptionStoreEN {
    fn lookup_exact(&self, graphemes: &str) -> Option<&TranscriptionEntry> {
        self.dicts
            .iter()
            .find_map(|dict| dict.lookup_exact(graphemes))
    }
    fn lookup_loose(&self, graphemes: &str) -> Option<&TranscriptionEntry> {
        self.dicts
            .iter()
            .find_map(|dict| dict.lookup_loose(graphemes))
    }
}

pub static WORD2IPA_EN: TranscriptionStoreEN = TranscriptionStoreEN {
    dicts: [
        TranscriptionDict {
            dict_name: "us_gold",
            map: phm_from_json!("./src/en/data/us_gold.json"),
        },
        TranscriptionDict {
            dict_name: "us_silver",
            map: phm_from_json!("./src/en/data/us_silver.json"),
        },
    ],
};
