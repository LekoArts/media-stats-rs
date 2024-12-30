# media-stats-rs

Generate a table with media statistics of video files in a folder. You'll receive information like the resolution or codec to quickly sort and filter your media files. Optionally you can create a `.csv` file with the information.

Uses [`ffprobe`](https://docs.rs/ffprobe/latest/ffprobe/) to inspect the files.

## Installation

You'll need to have the [Rust development environment](https://www.rustup.rs/) installed and up to date.

Once you have rust and dependencies installed, use cargo to install media-stats-rs:

```shell
cargo install --locked media-stats-rs
```

## Usage

You need to pass a `base` and `pattern`.

```shell
Usage: media-stats-rs [OPTIONS] --base <BASE> --pattern <PATTERN>

Options:
  -b, --base <BASE>        The base folder to search in
  -p, --pattern <PATTERN>  The file pattern to search for inside the base folder
  -c, --csv                Write output to a .csv file in the current directory
  -h, --help               Print help
  -V, --version            Print version
```

Example:

```shell
media-stats-rs --base "/Users/movies" --pattern "**/*.{mkv,mp4}"
```

Output:

```shell
🔍 Searching for files...
+-----------------------------------------+-------+--------+----------------+-------+-----------------+------------------------------------------------+
| Filename                                | Width | Height | Duration (min) | Codec | Audio Languages | Subtitles                                      |
+======================================================================================================================================================+
| Millennium Actress (2001).mkv           | 1920  | 1040   | 86             | hevc  | ger, jpn        | ger                                            |
+-----------------------------------------+-------+--------+----------------+-------+-----------------+------------------------------------------------+

🎬  Total files found: 1
✨  Done in 0 seconds
```