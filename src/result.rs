use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TranscriptList {
    pub video_id: String,
    pub video_name: String,
    pub available_transcripts: Vec<TranscriptType>,
}

impl Display for TranscriptList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified_available_transcripts = self
            .available_transcripts
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "Available Transcripts for video \"{}\" with title \"{}\": {}",
            self.video_id, self.video_name, stringified_available_transcripts
        )
    }
}

#[derive(Debug, Clone)]
pub struct TranscriptType {
    pub language: String,
    pub is_auto_generated: bool,
}

impl Display for TranscriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_auto_generated {
            return write!(f, "{}", self.language);
        } else {
            return write!(f, "{} (auto-generated)", self.language);
        }
    }
}
