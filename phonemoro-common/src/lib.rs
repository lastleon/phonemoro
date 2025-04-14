use phf::Map;

#[derive(Debug)]
pub enum TranscriptionEntry {
    Single(&'static str),
    Multiple(Map<&'static str, &'static str>),
}

pub trait TranscriptionLookup {
    fn lookup_exact(&self, graphemes: &str) -> Option<&TranscriptionEntry>;

    /// First, try looking up the exact word, if not found try looking up the lowercased word
    fn lookup_loose(&self, graphemes: &str) -> Option<&TranscriptionEntry> {
        self.lookup_exact(graphemes)
            .or(self.lookup_exact(graphemes.to_lowercase().as_str()))
    }
}

#[derive(Debug)]
pub struct TranscriptionDict {
    pub dict_name: &'static str,
    pub map: Map<&'static str, TranscriptionEntry>,
}

impl TranscriptionLookup for TranscriptionDict {
    fn lookup_exact(&self, graphemes: &str) -> Option<&TranscriptionEntry> {
        self.map.get(graphemes)
    }
}
