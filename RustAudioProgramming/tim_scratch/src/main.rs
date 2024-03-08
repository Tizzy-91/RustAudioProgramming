use std::io::{BufWriter, Write};
use std::fs::File;

const SAMPLE_RATE: u32 = 44100; // Sample rate in Hz
const DURATION: f32 = 3.0; // Duration of the wave in seconds
const FREQUENCY: f32 = 440.0; // Frequency of the wave in Hz

fn main() {
    let sine_samples = generate_sine_wave(SAMPLE_RATE, DURATION, FREQUENCY);
    write_wave_file("sine_wave.wav", &sine_samples, 1, SAMPLE_RATE).expect("Failed to write sine wave WAV file");

    let triangle_samples = generate_triangle_wave(SAMPLE_RATE, DURATION, FREQUENCY);
    write_wave_file("triangle_wave.wav", &triangle_samples, 1, SAMPLE_RATE).expect("Failed to write triangle wave WAV file");
}

fn generate_sine_wave(sample_rate: u32, duration: f32, frequency: f32) -> Vec<i16> {
    let num_samples = (duration * sample_rate as f32) as usize;
    let amplitude = 0.5; // Amplitude of the wave

    let mut samples = Vec::with_capacity(num_samples);
    for t in 0..num_samples {
        let sample = (amplitude * (2.0 * std::f32::consts::PI * frequency * (t as f32) / sample_rate as f32).sin()) as i16;
        samples.push(sample);
    }
    samples
}

fn generate_triangle_wave(sample_rate: u32, duration: f32, frequency: f32) -> Vec<i16> {
    let num_samples = (duration * sample_rate as f32) as usize;
    let amplitude = 0.5; // Amplitude of the wave
    let half_period_samples = (sample_rate as f32 / (2.0 * frequency)) as usize;

    let mut samples = Vec::with_capacity(num_samples);
    for t in 0..num_samples {
        let sample = if t % (2 * half_period_samples) < half_period_samples {
            ((2.0 * amplitude * t as f32 * frequency / sample_rate as f32) - amplitude) as i16
        } else {
            ((2.0 * amplitude * (1.0 - t as f32 * frequency / sample_rate as f32)) - amplitude) as i16
        };
        samples.push(sample);
    }
    samples
}

fn write_wave_file(filename: &str, samples: &[i16], num_channels: usize, sample_rate: u32) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(filename)?);

    // Write WAV header
    writer.write_all(b"RIFF")?;
    writer.write_all(&(36 + (samples.len() * 2) as u32).to_le_bytes())?;
    writer.write_all(b"WAVEfmt ")?;
    writer.write_all(&16u32.to_le_bytes())?; // Subchunk1Size
    writer.write_all(&1u16.to_le_bytes())?; // AudioFormat (PCM)
    writer.write_all(&(num_channels as u16).to_le_bytes())?; // NumChannels
    writer.write_all(&sample_rate.to_le_bytes())?; // SampleRate
    writer.write_all(&(sample_rate * 2 * num_channels as u32).to_le_bytes())?; // ByteRate
    writer.write_all(&(2 * num_channels as u16).to_le_bytes())?; // BlockAlign
    writer.write_all(&16u16.to_le_bytes())?; // BitsPerSample
    writer.write_all(b"data")?;
    writer.write_all(&(samples.len() as u32 * 2).to_le_bytes())?;

    // Write samples
    for &sample in samples {
        writer.write_all(&sample.to_le_bytes())?;
    }

    Ok(())
}
