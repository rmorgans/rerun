"""Log an audio file."""

import rerun as rr

rr.init("rerun_example_asset_audio", spawn=True)

# Log audio asset
rr.log("/recording/audio", rr.AssetAudio(path="recording.flac"), static=True)
