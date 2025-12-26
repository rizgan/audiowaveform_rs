use hound::{WavReader, Error};
use std::path::Path;
use rayon::prelude::*;

pub struct AudioWaveform {
    pub data: Vec<i8>,
    pub filename: String,
}

pub fn process_audio_file(file_path: &str) -> Result<Vec<i8>, Box<dyn std::error::Error>> {
    let mut audio_reader = WavReader::open(file_path)?;
    let audio_spec = audio_reader.spec();
    let audio_samples: Vec<i32> = audio_reader.samples().collect::<Result<Vec<i32>, Error>>()?;

    let track_duration = audio_samples.len() as f32 / audio_spec.sample_rate as f32;
    
    let detail_level = if track_duration < 300.0 {
        3
    } else if track_duration < 600.0 {
        2
    } else {
        1
    };

    let chunk_size = audio_spec.sample_rate / detail_level;
    let amplitude_max = 2.0_f32.powf(audio_spec.bits_per_sample as f32 - 1.0);
    
    let mut wave_points = Vec::new();
    
    for sample_group in audio_samples.chunks(chunk_size as usize) {
        let peak_value = sample_group.iter().map(|&s| s.abs()).max().unwrap_or(0);
        let scaled_value = (peak_value as f32 / amplitude_max * 127.0) as i8;
        wave_points.push(-scaled_value);
        wave_points.push(scaled_value);
    }

    Ok(wave_points)
}

pub fn process_multiple_files(file_paths: Vec<String>) -> Vec<Result<AudioWaveform, String>> {
    file_paths
        .par_iter()
        .map(|path| {
            match process_audio_file(path) {
                Ok(data) => Ok(AudioWaveform {
                    data,
                    filename: Path::new(path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                }),
                Err(e) => Err(format!("Error processing {}: {}", path, e)),
            }
        })
        .collect()
}

pub fn validate_audio_path(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err("The specified file does not exist.".to_string());
    }
    
    if path.extension().unwrap_or_default() != "wav" {
        return Err("The specified file is not a WAV file.".to_string());
    }
    
    Ok(())
}
