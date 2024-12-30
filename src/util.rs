use std::collections::HashSet;

use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Utc,
};
use ffprobe::FfProbe;
use globset::{GlobBuilder, GlobMatcher};
use walkdir::{DirEntry, WalkDir};

/// Create a glob matcher from a pattern
fn create_glob(pattern: &str) -> globset::GlobMatcher {
    let glob = GlobBuilder::new(pattern)
        .literal_separator(true)
        .build()
        .expect("Failed to build glob");
    glob.compile_matcher()
}

/// Hide files starting with a dot
pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Remove trailing slash from a &str
fn remove_trailing_slash(s: &str) -> &str {
    if s.ends_with('/') {
        &s[..s.len() - 1]
    } else {
        s
    }
}

/// Remove leading slash from a &str
fn remove_leading_slash(s: &str) -> &str {
    if s.starts_with('/') {
        &s[1..]
    } else {
        s
    }
}

pub struct FileInfo {
    pub absolute_path: String,
    pub filename: String,
}

/// Create a walker and count total files
pub fn get_walker(unsafe_base: &str) -> walkdir::IntoIter {
    let base = remove_trailing_slash(unsafe_base);

    WalkDir::new(base).sort_by_file_name().into_iter()
}

/// Create a glob matcher from a base and pattern
pub fn get_matcher(unsafe_base: &str, unsafe_pattern: &str) -> GlobMatcher {
    let base = remove_trailing_slash(unsafe_base);
    let pattern = remove_leading_slash(unsafe_pattern);

    create_glob(format!("{}/{}", base, pattern).as_str())
}

pub struct MediaStats {
    pub height: i64,
    pub width: i64,
    pub duration: String,
    pub file_size_bytes: u64,
    pub codec_name: String,
    pub audio_languages: Vec<String>,
    pub subtitles: Vec<String>,
}

/// Extract relevant media info
pub fn extract_media_stats(info: &FfProbe) -> MediaStats {
    let height = info
        .streams
        .iter()
        .find(|s| s.height.is_some())
        .unwrap()
        .height
        .unwrap();
    let width = info
        .streams
        .iter()
        .find(|s| s.width.is_some())
        .unwrap()
        .width
        .unwrap();
    let raw_duration = info.format.duration.clone();
    let codec_name = info
        .streams
        .iter()
        .find(|s| s.codec_type == Some("video".to_string()))
        .unwrap()
        .codec_name
        .clone();
    let audio_languages: Vec<String> = info
        .streams
        .iter()
        .filter(|s| s.codec_type == Some("audio".to_string()))
        .map(|s| {
            s.tags
                .as_ref()
                .and_then(|t| t.language.clone())
                .unwrap_or("N/A".to_string())
        })
        .collect();
    let raw_subtitles: Vec<String> = info
        .streams
        .iter()
        .filter(|s| s.codec_type == Some("subtitle".to_string()))
        .map(|s| {
            s.tags
                .as_ref()
                .and_then(|t| t.language.clone())
                .unwrap_or("N/A".to_string())
        })
        .collect();

    let duration = raw_duration.unwrap_or("0".to_string());
    // Remove duplicates
    let subtitles = raw_subtitles
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let file_size_bytes = info.format.size.parse::<u64>().unwrap_or(0);

    MediaStats {
        height,
        width,
        duration: format!("{:.0}", duration.parse::<f64>().unwrap() / 60.0),
        file_size_bytes,
        codec_name: codec_name.unwrap_or("N/A".to_string()),
        audio_languages,
        subtitles,
    }
}

pub fn get_current_date<'a>() -> DelayedFormat<StrftimeItems<'a>> {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d_%H-%M-%S")
}
