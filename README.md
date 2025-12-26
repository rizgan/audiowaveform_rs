# Audio Waveform Generator (Rust)

A Rust command-line application that generates waveform data from WAV format audio files. Waveform data can be used to produce a visual rendering of the audio, similar in appearance to audio editing applications.

## Features

- **WAV Audio Support**: Process WAV format audio files
- **JSON Output**: Waveform data is output in JSON format
- **Parallel Processing**: Uses Rayon for efficient multi-file processing
- **Adaptive Detail Level**: Automatically adjusts detail based on audio duration
  - Short tracks (<5 min): High detail (3x sample rate)
  - Medium tracks (5-10 min): Medium detail (2x sample rate)
  - Long tracks (>10 min): Standard detail (1x sample rate)

## How It Works

The waveform data is produced from an input audio signal by:

1. Reading the input WAV file samples
2. Computing the peak (maximum absolute value) over groups of N input samples
3. Normalizing the peak values to a -128 to 127 range
4. Outputting the normalized waveform data as JSON

Each group of N input samples produces one peak point in the output, where N is determined by the adaptive detail level based on track duration.

## Installation

```bash
cargo build --release
```

The compiled binary will be available at `target/release/audiowaveform_rs` (or `audiowaveform_rs.exe` on Windows).

## Usage

Process a single WAV file:

```bash
audiowaveform_rs input.wav
```

Process multiple WAV files:

```bash
audiowaveform_rs file1.wav file2.wav file3.wav
```

### Output Format

Single file output:

```json
{
  "word_wf": [-45, 23, -67, 12, ...]
}
```

Multiple files output:

```json
[
  {
    "filename": "file1.wav",
    "word_wf": [-45, 23, -67, ...]
  },
  {
    "filename": "file2.wav",
    "word_wf": [-32, 45, -21, ...]
  }
]
```

## Dependencies

- **hound**: WAV file reading
- **serde_json**: JSON serialization
- **rayon**: Parallel processing

# 

## License

See LICENSE file for details.