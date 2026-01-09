# Rust File Organizer

A small file organizer program I made to learn rust. It organizes the following extensions into these directories:

["jpg", "jpeg", "png", "gif", "webp", "svg", "tiff", "heif", "psd", "raw", "bmp", "ico", "ai"] → ~/Pictures

["mp4", "mov", "webm", "mkv", "flv", "ogg", "avi", "m4p", "mv4", "mpg", "mpeg"] → /Videos

["mp3", "wav", "aiff", "flac", "ogg", "opus"] → /Music

["pdf", "docx", "pptx", "csv"] → /Documents

## Usage

By default, it uses `~/Downloads` as the source directory:

`./download-organizer`

You can change the source directory by passing the path as an argument:

`./download-organizer /tmp/family-photos`

## Build

`cargo build --release`