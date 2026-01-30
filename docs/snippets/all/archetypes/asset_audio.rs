//! Log an audio file.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_asset_audio").spawn()?;

    // Log audio asset
    rec.log_static(
        "/recording/audio",
        &rerun::AssetAudio::from_file_path("recording.flac")?,
    )?;

    Ok(())
}
