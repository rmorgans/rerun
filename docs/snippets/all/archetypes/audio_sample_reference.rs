//! Reference audio frames on timeline.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_audio_sample_reference").spawn()?;

    // Log audio asset first
    rec.log_static(
        "/recording/audio",
        &rerun::AssetAudio::from_file_path("recording.flac")?,
    )?;

    // Log timeline references
    let sample_rate: u64 = 48000;
    for t in 0..60 {
        rec.set_time_seconds("audio_time", t as f64);
        rec.log(
            "/recording/audio",
            &rerun::AudioSampleReference::new(t * sample_rate),
        )?;
    }

    Ok(())
}
