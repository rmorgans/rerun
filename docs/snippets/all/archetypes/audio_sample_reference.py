"""Reference audio frames on timeline."""

import rerun as rr

rr.init("rerun_example_audio_sample_reference", spawn=True)

# Log audio asset first
rr.log("/recording/audio", rr.AssetAudio(path="recording.flac"), static=True)

# Log timeline references
sample_rate = 48000
for t in range(60):  # 60 seconds
    rr.set_time_seconds("audio_time", float(t))
    rr.log("/recording/audio", rr.AudioSampleReference(sample_offset=t * sample_rate))
