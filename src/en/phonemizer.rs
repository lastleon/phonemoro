use anyhow::{anyhow, Context, Result};
use phonetisaurus_g2p::PhonetisaurusModel;
use tokenizers::{normalizers::NFKC, NormalizedString, Normalizer};

use crate::en::{constants::DIGIT2WORD, word2ipa::WORD2IPA_EN};
use phonemizer_common::{TranscriptionEntry, TranscriptionLookup};

use super::constants::UNICODE2ASCII;
use super::tokenizer::{EnTokenizer, Token, TokenContext};

static PHONETISAURUS_MODEL_BIN: &[u8] = include_bytes!("data/model.fst");

/// Phonemizer struct.
#[derive(Debug)]
pub struct EnPhonemizer {
    normalizer: NFKC, // Maybe make this more dynamic with some kind of impl / dyn / where
    fallback_phonemizer: PhonetisaurusModel,
}

impl EnPhonemizer {
    /// Create a new phonemizer.
    pub fn new() -> Result<Self> {
        Ok(EnPhonemizer {
            normalizer: NFKC::default(),
            fallback_phonemizer: PhonetisaurusModel::try_from(PHONETISAURUS_MODEL_BIN)?, // TODO: find out how to only check this during compile time
        })
    }

    /// Phonemize a text. For each word, a dictionary lookup is performed, and if nothing is found, the word is
    /// phonemized with a finite state transducer trained using Phonetisaurus.
    pub fn phonemize(&self, text: &str) -> Result<String> {
        let normalized = self.normalize(text)?;
        let tokens = self.tokenize(normalized.as_str())?;
        let phonemes = self.tokens2phonemes(&tokens)?;

        Ok(phonemes)
    }

    // NORMALIZATION
    fn normalize(&self, text: &str) -> Result<String> {
        let mut text = NormalizedString::from(text);
        self.normalizer
            .normalize(&mut text)
            .map_err(|e| anyhow!(e))?; // converts Box<dyn Error + Send + Sync> to anyhow error

        // Try to normalize some unicode characters to their closes ascii counterparts,
        // for better compatibility with the lookup dicts.
        let normalized_string = text.get().chars().fold(String::new(), |mut acc, c| {
            if let Some(&replacement) = UNICODE2ASCII.get(&c) {
                acc.push_str(replacement);
            } else {
                acc.push(c);
            }
            acc
        });

        Ok(normalized_string)
    }

    // TOKENIZATION
    fn tokenize<'a>(&self, text: &'a str) -> Result<Vec<TokenContext<'a>>> {
        EnTokenizer::tokenize(text)
    }

    // TRANSCRIPTION
    fn look_up_transcription(&self, graphemes: &str) -> Option<&str> {
        if let Some(transcription) = WORD2IPA_EN.lookup_loose(graphemes) {
            return Some(match transcription {
                TranscriptionEntry::Single(ph) => ph,
                TranscriptionEntry::Multiple(variant_mapping) => variant_mapping
                    .get("DEFAULT")
                    .expect("DEFAULT variant not found."),
            });
            // TODO: Instead of using DEFAULT, use a homograph disambiguation algorithm / model
            // Possible starts:
            // - Viterbi Algorithm:
            //      - https://github.com/nkaush/pos-tagging
            //      - https://github.com/ian-nai/viterbi_pos_tagger
            // - Perceptron Tagger:
            //      - https://github.com/shubham0204/postagger.rs
            //      - Celosia
            // - BERT:
            //      - https://docs.rs/rust-bert/latest/rust_bert/pipelines/pos_tagging/
            // - Custom model
        }

        None
    }

    fn tokens2phonemes(&self, tokens: &Vec<TokenContext>) -> Result<String> {
        let results: Vec<String> = tokens
            .iter()
            .map(|tc| {
                // query dicts
                let dict_lookup_result: Option<String> = match tc.token {
                    Token::Word => self.look_up_transcription(&tc.slice).map(String::from),
                    Token::DigitSequence => {
                        let mut full_string = String::new();
                        full_string.push_str(" ");
                        tc.slice.chars().for_each(|c| {
                            let graphemes = DIGIT2WORD.get(&c).expect(
                                "Digit not found in DIGIT2WORD dict, this should not be possible.",
                            );
                            full_string.push_str(self.look_up_transcription(graphemes).unwrap());
                            full_string.push(' ');
                        });

                        Some(full_string)
                    }
                    _ => Some(String::from(tc.slice)),
                };

                if let Some(phonemes) = dict_lookup_result {
                    return phonemes;
                }

                // phonemize unknown word with FST
                let fst_phonemization = self
                    .fallback_phonemizer
                    .phonemize_word(tc.slice)
                    .with_context(|| {
                        format!("Phonemization of word {} with the FST failed.", tc.slice)
                    })
                    .unwrap();

                fst_phonemization.phonemes
            })
            .collect();
        Ok(results.join(""))
    }
}
