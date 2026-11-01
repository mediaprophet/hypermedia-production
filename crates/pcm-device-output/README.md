# PCM device output

Native output adapter over the existing controlled `audio-device-cpal` backend.
Reuses QualiaDB `SoundBinding` and `SoundBlockReceipt` and the established native
WAV adapter's exact-format and acknowledged WASAPI reset policy. Original Poet
source remains in place. No Poet/Synth application dependency, decoder, DSP,
semantic store, authority provider or new authored clock is supplied.

The native owner supplies an atomic cancellation lease and an exact liveness
check. It authorizes every enqueue under its native source/session guard. The
device worker checks liveness outside the realtime callback. This polling is
not instantaneous revocation or a measured latency guarantee; already buffered
audio needs acknowledged reset. Other backends refuse that claim.

Caller-admitted storage: 2–32 fixed slots, 1–8192 stereo frames per slot and an
explicit logical byte budget, allocated before playback. Output is exact 48 kHz
stereo f32; unsupported hardware formats refuse. Callback lock contention or
starvation retires the epoch rather than resuming at a shifted source time.
Slot storage stays allocated through callbacks. Device callback frame counts
are not acoustic presentation timestamps. One worker owns each stream and the
process device slot remains held until it actually exits, including driver stalls.

This new source is uncompiled/unexecuted. No audible output or allocation/device
qualification is claimed. Tests and builds remain deferred during coding.
