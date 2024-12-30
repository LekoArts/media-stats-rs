mod util;

use clap::Parser;
use comfy_table::{ContentArrangement, Table};
use console::Emoji;
use ffprobe::ffprobe;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::env;
use std::fs::File;
use std::io;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "media-stats-rs", version, about, long_about = None)]
struct Cli {
    /// The base folder to search in
    #[arg(short, long, required = true)]
    base: String,
    /// The file pattern to search for inside the base folder
    #[arg(short, long, required = true)]
    pattern: String,
    /// Write output to a .csv file in the current directory
    #[arg(short, long, default_value_t = false)]
    csv: bool,
}

const HEADER: [&str; 8] = [
    "Filename",
    "Width",
    "Height",
    "Duration (min)",
    "Size (GB)",
    "Codec",
    "Audio",
    "Subtitles",
];

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static MOVIE: Emoji<'_, '_> = Emoji("🎬 ", "");

fn main() {
    let started = Instant::now();

    let args = Cli::parse();

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let filepath = current_dir.join(format!("media-stats-rs_{}.csv", util::get_current_date()));

    let mut table = Table::new();
    let io_file_wrt: Box<dyn io::Write> = if args.csv {
        Box::new(File::create(&filepath).expect("Failed to create csv file"))
    } else {
        Box::new(io::stdout())
    };
    let mut writer = csv::Writer::from_writer(io_file_wrt);

    table
        .set_header(HEADER)
        .set_content_arrangement(ContentArrangement::Dynamic);

    if args.csv {
        writer
            .write_record(HEADER)
            .expect("Failed to write header to csv file");
    }

    println!("{}Searching for files...", LOOKING_GLASS);

    let walker = util::get_walker(&args.base);
    let matcher = util::get_matcher(&args.base, &args.pattern);
    let mut counter = 0;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}] {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );

    for entry in walker.filter_entry(|e| !util::is_hidden(e)) {
        let entry = entry.expect("Failed to read file in given base folder.");
        let path = entry.path();

        if path.is_file() && matcher.is_match(path) {
            let file = util::FileInfo {
                absolute_path: path.to_string_lossy().to_string(),
                filename: path.file_name().unwrap().to_string_lossy().to_string(),
            };

            pb.set_message(format!("{}", &file.filename));

            let info = ffprobe(&file.absolute_path).expect(
                format!("Failed to get media info for file: {}", file.absolute_path).as_str(),
            );

            let stats = util::extract_media_stats(&info);
            let file_size_gigabytes = format!("{:.2}", stats.file_size_bytes as f64 / 1e9);

            table.add_row(vec![
                file.filename.clone(),
                stats.width.to_string(),
                stats.height.to_string(),
                stats.duration.clone(),
                file_size_gigabytes.clone(),
                stats.codec_name.clone(),
                stats.audio_languages.join(", "),
                stats.subtitles.join(", "),
            ]);

            if args.csv {
                writer
                    .serialize((
                        file.filename.clone(),
                        stats.width,
                        stats.height,
                        stats.duration,
                        file_size_gigabytes.clone(),
                        stats.codec_name,
                        stats.audio_languages.join(", "),
                        stats.subtitles.join(", "),
                    ))
                    .expect("Failed to write row to csv file");
            }

            pb.tick();

            counter += 1;
        }
    }

    if args.csv {
        writer.flush().expect("Failed to flush csv writer");
        println!("CSV file written to: {}", &filepath.display());
    }

    pb.finish_and_clear();

    println!("{table}");
    println!("");
    println!("{} Total files found: {}", MOVIE, counter);
    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()))
}
