from __future__ import annotations

import pathlib
from typing import TYPE_CHECKING, Any

from ..error_utils import catch_and_log_exceptions

if TYPE_CHECKING:
    from .. import datatypes


class AssetAudioExt:
    """Extension for [AssetAudio][rerun.archetypes.AssetAudio]."""

    def __init__(
        self: Any,
        *,
        path: str | pathlib.Path | None = None,
        contents: datatypes.BlobLike | None = None,
        media_type: datatypes.Utf8Like | None = None,
        sample_rate: datatypes.UInt32Like | None = None,
        channel_layout: datatypes.UInt32Like | None = None,
        bits_per_sample: datatypes.UInt32Like | None = None,
        sample_format: datatypes.SampleFormatLike | None = None,
    ) -> None:
        """
        Create a new instance of the AssetAudio archetype.

        Parameters
        ----------
        path:
            A path to a file stored on the local filesystem. Mutually
            exclusive with `contents`.

        contents:
            The contents of the file. Can be a BufferedReader, BytesIO, or
            bytes. Mutually exclusive with `path`.

        media_type:
            The Media Type of the asset.

            For instance:
             * `audio/flac`
             * `audio/wav`
             * `audio/mpeg` (MP3)
             * `audio/ogg` (Opus or Vorbis)

            If omitted, it will be guessed from the `path` (if any),
            or the viewer will try to guess from the contents (magic header).
            If the media type cannot be guessed, the viewer won't be able to render the asset.

        sample_rate:
            Sample rate in Hz (e.g., 48000). Optional hint for faster initialization.

        channel_layout:
            Channel layout as a bitmask. Common values: 0x3 = Stereo, 0x3F = 5.1 Surround.

        bits_per_sample:
            Bit depth of audio samples (8, 16, 24, or 32).

        sample_format:
            Sample format for raw PCM data. Only needed for raw PCM without a container.

        """

        from ..components import MediaType

        with catch_and_log_exceptions(context=self.__class__.__name__):
            if (path is None) == (contents is None):
                raise ValueError("Must provide exactly one of 'path' or 'contents'")

            if path is None:
                blob = contents
            else:
                blob = pathlib.Path(path).read_bytes()
                if media_type is None:
                    media_type = MediaType.guess_from_path(path)

            self.__attrs_init__(
                blob=blob,
                media_type=media_type,
                sample_rate=sample_rate,
                channel_layout=channel_layout,
                bits_per_sample=bits_per_sample,
                sample_format=sample_format,
            )
            return

        self.__attrs_clear__()
