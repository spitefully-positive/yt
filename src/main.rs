use clap::Parser;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use yt::TranscriptError;

mod result;
use result::TranscriptList;
mod youtube_transcript_api;
use youtube_transcript_api::YoutubeTranscriptAPI as TranscriptAPI;

#[derive(Parser)]
#[command(name = "yt")]
#[command(about = "YouTube Transcript API - Fetch transcripts from YouTube videos", long_about = None)]
struct Args {
    /// YouTube video URL or video ID
    video: String,

    /// Language codes (e.g., en, es, fr). Can specify multiple.
    #[arg(short, long, default_value = "de en")]
    languages: Vec<String>,

    /// Translate transcript to this language code, if this language code is not avalable
    #[arg(short, long)]
    translate: Option<String>,

    /// Output format: json, text, txt, srt, or markdown
    #[arg(short, long, default_value = "text")]
    format: String,

    /// Show transcript text with timestamps
    #[arg(long, default_value = "false")]
    timestamps: bool,

    /// List available transcripts instead of fetching
    #[arg(long, default_value = "false")]
    list: bool,

    /// Delay between requests in milliseconds
    #[arg(long, default_value = "500")]
    delay: u64,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run(args: Args) -> Result<(), TranscriptError> {
    let api = YouTubeTranscript::with_delay(args.delay);
    let video_id = YouTubeTranscript::extract_video_id(&args.video)?;

    if args.list {
        match get_available_transcripts(api, &video_id).await {
            Ok(transcript) => println!("Available Transcripts: {}", transcript),
            Err(error) => println!("{}", error),
        }
        return Ok(());
    }

    todo!("The rest of the implementation translation");
}

async fn get_available_transcripts(
    api: TranscriptAPI,
    video_id: &str,
) -> Result<TranscriptList, TranscriptError> {
    todo!("Implement");
}

async fn process_single_video(
    api: &YouTubeTranscript,
    args: &Args,
    video_id: &str,
    video_index: Option<usize>,
    total_videos: Option<usize>,
) -> Result<(), TranscriptError> {
    if video_index.is_none() {
        println!("Fetching transcript for video: {}", video_id);
    }

    let transcript = if let Some(target_lang) = &args.translate {
        let source_langs: Vec<&str> = args
            .languages
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_else(|| vec!["en"]);
        api.translate_transcript(video_id, &source_langs, target_lang)
            .await?
    } else {
        let lang_codes: Option<Vec<&str>> = args
            .languages
            .as_ref()
            .map(|v| v.iter().map(|s| s.as_str()).collect());
        api.fetch_transcript(video_id, lang_codes).await?
    };

    // Determine if we need markdown formatting from ChatGPT
    let format_markdown = args.cleanup
        && (args.format.to_lowercase() == "markdown" || args.format.to_lowercase() == "md");

    // If cleanup is requested, send to ChatGPT first
    let transcript_items = if args.cleanup {
        if video_index.is_none() {
            eprintln!("Cleaning up transcript with ChatGPT...");
        }
        let transcript_text: String = transcript
            .transcript
            .iter()
            .map(|item| item.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        let chatgpt = ChatGPT::new(args.openai_key.clone())?;
        let cleaned_text = chatgpt
            .cleanup_transcript(&transcript_text, format_markdown)
            .await?;

        // For cleanup, output the cleaned text directly as a single item
        // This preserves the cleaned flow better than trying to split it back
        vec![TranscriptItem {
            text: cleaned_text,
            start: transcript
                .transcript
                .first()
                .map(|i| i.start)
                .unwrap_or(0.0),
            duration: transcript.transcript.iter().map(|i| i.duration).sum(),
        }]
    } else {
        transcript.transcript
    };

    // Determine output destination
    // For playlists, if -o is a directory or -n is used, each video gets its own file
    let output_dest = if let Some(ref output_path) = args.output {
        let path = Path::new(output_path);

        // Check if the path is a directory:
        // 1. If it exists and is a directory
        // 2. If it ends with a path separator (directory-like)
        let is_directory = if path.exists() {
            path.is_dir()
        } else {
            // If path doesn't exist, check if it ends with a separator (directory-like)
            let sep = std::path::MAIN_SEPARATOR;
            output_path.ends_with(sep) || output_path.ends_with('/')
        };
}
