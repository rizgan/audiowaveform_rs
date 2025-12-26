use std::env;
use std::path::Path;
use audiowaveform_rs::{process_multiple_files, validate_audio_path};
use serde_json::json;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Please specify one or more paths to WAV audio files as arguments.");
        eprintln!("Usage: {} <file1.wav> [file2.wav] [file3.wav] ...", args[0]);
        return;
    }

    let file_paths: Vec<String> = args.iter().skip(1).map(|s| s.clone()).collect();
    
    for path in &file_paths {
        let audio_path = Path::new(path);
        if let Err(e) = validate_audio_path(audio_path) {
            eprintln!("Error with file {}: {}", path, e);
            return;
        }
    }

    if file_paths.len() == 1 {
        let results = process_multiple_files(file_paths);
        match &results[0] {
            Ok(waveform) => {
                let json_output = json!({
                    "word_wf": waveform.data
                });
                println!("{}", json_output);
            },
            Err(error) => eprintln!("Error generating waveform: {}", error),
        }
    } else {
        let results = process_multiple_files(file_paths);
        let mut all_results = Vec::new();
        
        for result in results {
            match result {
                Ok(waveform) => {
                    all_results.push(json!({
                        "filename": waveform.filename,
                        "word_wf": waveform.data
                    }));
                },
                Err(error) => {
                    eprintln!("Error: {}", error);
                }
            }
        }
        
        let json_output = json!({
            "results": all_results
        });
        println!("{}", json_output);
    }
}
