# `crates/audio-device-cpal`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_coreaudio on iOS looks a bit different from macOS. A lot of configuration needs to use
the AVAudioSession objc API which doesn't exist on macOS.

TODO:
- Use AVAudioSession to enumerate buffer size / sample rate / number of channels and set
buffer size._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `main` | 0 | `crates/audio-device-cpal/build.rs:5` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/android.rs:13` | - |  |
| `run` | 2 | `crates/audio-device-cpal/examples/android.rs:41` | - |  |
| `write_data` | 3 | `crates/audio-device-cpal/examples/android.rs:72` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/beep.rs:29` | - |  |
| `run` | 2 | `crates/audio-device-cpal/examples/beep.rs:97` | - |  |
| `write_data` | 3 | `crates/audio-device-cpal/examples/beep.rs:128` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/enumerate.rs:6` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/feedback.rs:43` | - |  |
| `err_fn` | 1 | `crates/audio-device-cpal/examples/feedback.rs:172` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/record_wav.rs:34` | - |  |
| `sample_format` | 1 | `crates/audio-device-cpal/examples/record_wav.rs:145` | - |  |
| `wav_spec_from_config` | 1 | `crates/audio-device-cpal/examples/record_wav.rs:153` | - |  |
| `write_input_data` | 2 | `crates/audio-device-cpal/examples/record_wav.rs:164` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/synth_tones.rs:15` | - |  |
| `impl Oscillator::advance_sample` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:37` | - |  |
| `impl Oscillator::set_waveform` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:41` | - |  |
| `impl Oscillator::calculate_sine_output_from_freq` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:45` | - |  |
| `impl Oscillator::is_multiple_of_freq_above_nyquist` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:50` | - |  |
| `impl Oscillator::sine_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:54` | - |  |
| `impl Oscillator::generative_waveform` | 3 | `crates/audio-device-cpal/examples/synth_tones.rs:59` | - |  |
| `impl Oscillator::square_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:71` | - |  |
| `impl Oscillator::saw_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:75` | - |  |
| `impl Oscillator::triangle_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:79` | - |  |
| `impl Oscillator::tick` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:83` | - |  |
| `stream_setup_for` | 0 | `crates/audio-device-cpal/examples/synth_tones.rs:93` | - |  |
| `host_device_setup` | 0 | `crates/audio-device-cpal/examples/synth_tones.rs:115` | - |  |
| `make_stream` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:130` | - |  |
| `process_frame` | 3 | `crates/audio-device-cpal/examples/synth_tones.rs:176` | - |  |
| `impl Display for HostUnavailable::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:9` | - |  |
| `impl Display for BackendSpecificError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:34` | - |  |
| `impl Display for DevicesError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:53` | - |  |
| `impl From<BackendSpecificError> for DevicesError::from` | 1 | `crates/audio-device-cpal/src/error.rs:63` | - |  |
| `impl Display for DeviceNameError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:76` | - |  |
| `impl From<BackendSpecificError> for DeviceNameError::from` | 1 | `crates/audio-device-cpal/src/error.rs:86` | - |  |
| `impl Display for SupportedStreamConfigsError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:104` | - |  |
| `impl From<BackendSpecificError> for SupportedStreamConfigsError::from` | 1 | `crates/audio-device-cpal/src/error.rs:116` | - |  |
| `impl Display for DefaultStreamConfigError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:134` | - |  |
| `impl From<BackendSpecificError> for DefaultStreamConfigError::from` | 1 | `crates/audio-device-cpal/src/error.rs:150` | - |  |
| `impl Display for BuildStreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:174` | - |  |
| `impl From<BackendSpecificError> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:196` | - |  |
| `impl Display for PlayStreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:215` | - |  |
| `impl From<BackendSpecificError> for PlayStreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:228` | - |  |
| `impl Display for PauseStreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:247` | - |  |
| `impl From<BackendSpecificError> for PauseStreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:260` | - |  |
| `impl Display for StreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:276` | - |  |
| `impl From<BackendSpecificError> for StreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:289` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:12` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:25` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:50` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:58` | - |  |
| `impl From<alsa::Error> for DevicesError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:66` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/alsa/mod.rs:32` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/alsa/mod.rs:41` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:46` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:50` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:54` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:64` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:68` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:74` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:80` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:84` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/alsa/mod.rs:88` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/alsa/mod.rs:111` | - |  |
| `impl TriggerSender::wakeup` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:140` | - |  |
| `impl TriggerReceiver::clear_pipe` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:148` | - |  |
| `trigger` | 0 | `crates/audio-device-cpal/src/host/alsa/mod.rs:155` | - |  |
| `impl Drop for TriggerSender::drop` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:164` | - |  |
| `impl Drop for TriggerReceiver::drop` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:172` | - |  |
| `impl DeviceHandles::open` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:188` | - | Create `DeviceHandles` for `name` and try to open a handle for both |
| `impl DeviceHandles::try_open` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:203` | - | Get a mutable reference to the `Option` for a specific `stream_type`. |
| `impl DeviceHandles::get_mut` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:222` | - | Get a mutable reference to the `alsa::PCM` handle for a specific `stream_type`. |
| `impl DeviceHandles::take` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:232` | - | Take ownership of the `alsa::PCM` handle for a specific `stream_type`. |
| `impl Device::build_stream_inner` | 4 | `crates/audio-device-cpal/src/host/alsa/mod.rs:244` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:301` | - |  |
| `impl Device::supported_configs` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:305` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:441` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:447` | - |  |
| `impl Device::default_config` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:455` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:493` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:497` | - |  |
| `impl StreamWorkerContext::new` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:563` | - |  |
| `input_stream_worker` | 5 | `crates/audio-device-cpal/src/host/alsa/mod.rs:578` | - |  |
| `output_stream_worker` | 5 | `crates/audio-device-cpal/src/host/alsa/mod.rs:629` | - |  |
| `poll_descriptors_and_prepare_buffer` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:693` | - |  |
| `process_input` | 5 | `crates/audio-device-cpal/src/host/alsa/mod.rs:783` | - |  |
| `process_output` | 7 | `crates/audio-device-cpal/src/host/alsa/mod.rs:810` | - |  |
| `stream_timestamp` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:865` | - |  |
| `timespec_to_nanos` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:894` | - |  |
| `timespec_diff_nanos` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:900` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:905` | - |  |
| `impl Stream::new_input` | 4 | `crates/audio-device-cpal/src/host/alsa/mod.rs:913` | - |  |
| `impl Stream::new_output` | 4 | `crates/audio-device-cpal/src/host/alsa/mod.rs:945` | - |  |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:979` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:986` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:990` | - |  |
| `set_hw_params_from_format` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:996` | - |  |
| `set_sw_params_from_format` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1078` | - |  |
| `impl From<alsa::Error> for BackendSpecificError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1123` | - |  |
| `impl From<alsa::Error> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1131` | - |  |
| `impl From<alsa::Error> for SupportedStreamConfigsError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1138` | - |  |
| `impl From<alsa::Error> for PlayStreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1145` | - |  |
| `impl From<alsa::Error> for PauseStreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1152` | - |  |
| `impl From<alsa::Error> for StreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1159` | - |  |
| `impl PartialEq for Device::eq` | 2 | `crates/audio-device-cpal/src/host/asio/device.rs:39` | - |  |
| `impl Hash for Device::hash` | 2 | `crates/audio-device-cpal/src/host/asio/device.rs:47` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:53` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:60` | - | Gets the supported input configs. |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:97` | - | Gets the supported output configs. |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:132` | - | Returns the default input config |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:153` | - | Returns the default output config |
| `impl Devices::new` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:174` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:184` | - | Load drivers and return device |
| `convert_data_type` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:208` | - |  |
| `default_config_err` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:221` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/asio/mod.rs:25` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/asio/mod.rs:36` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:41` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:45` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:50` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:61` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:65` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:71` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:77` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:81` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/mod.rs:85` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/mod.rs:107` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:131` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:135` | - |  |
| `impl Stream::play` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:24` | - |  |
| `impl Stream::pause` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:29` | - |  |
| `impl Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/stream.rs:36` | - |  |
| `process_input_callback` | 7 | `crates/audio-device-cpal/src/host/asio/stream.rs:87` | - | 1. Write from the ASIO buffer to the interleaved CPAL buffer. |
| `impl Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/stream.rs:241` | - |  |
| `process_output_callback` | 8 | `crates/audio-device-cpal/src/host/asio/stream.rs:304` | - | 1. Render the given callback to the given buffer of interleaved samples. |
| `impl Device::get_or_create_input_stream` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:507` | - | Create a new CPAL Input Stream. |
| `impl Device::get_or_create_output_stream` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:554` | - | Create a new CPAL Output Stream. |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:600` | - |  |
| `asio_ns_to_double` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:605` | - |  |
| `system_time_to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:611` | - | Asio retrieves system time via `timeGetTime` which returns the time in milliseconds. |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/asio/stream.rs:621` | - | Convert the given duration in frames at the given sample rate to a `std::time::Duration`. |
| `check_config` | 4 | `crates/audio-device-cpal/src/host/asio/stream.rs:631` | - | Check whether or not the desired config is supported by the stream. |
| `cast_slice_mut` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:670` | - | Cast a byte slice into a mutable slice of desired type. |
| `from_le` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:676` | - | Helper function to convert from little endianness. |
| `from_be` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:681` | - | Helper function to convert from little endianness. |
| `asio_channel_slice` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:686` | - | Shorthand for retrieving the asio buffer slice associated with a channel. |
| `asio_channel_slice_mut` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:697` | - | Shorthand for retrieving the asio buffer slice associated with a channel. |
| `build_stream_err` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:707` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:15` | - |  |
| `impl Default for Devices::default` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:21` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:30` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:36` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:41` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:50` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:59` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:63` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:67` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:71` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:78` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:83` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:102` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:124` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:131` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:144` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:149` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:156` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:163` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:168` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:172` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:255` | - | Create an output stream. |
| `impl Stream::new` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:337` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:345` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:359` | - |  |
| `create_audio_unit` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:380` | - |  |
| `configure_for_recording` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:384` | - |  |
| `default_output_asbd` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:406` | - |  |
| `default_input_asbd` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:414` | - |  |
| `stream_config_from_asbd` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:426` | - |  |
| `audio_devices` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:16` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:63` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:83` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:91` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:121` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:57` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:66` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:71` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:75` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:79` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:89` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:93` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:99` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:105` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:109` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:113` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:135` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:165` | - |  |
| `impl Device::supported_configs` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:218` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:320` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:326` | - |  |
| `impl Device::default_config` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:332` | - |  |
| `default_config_error_from_os_status` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:336` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:415` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:419` | - |  |
| `impl fmt::Debug for Device::fmt` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:425` | - |  |
| `add_disconnect_listener` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:449` | - | Register the on-disconnect callback. |
| `audio_unit_from_device` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:473` | - |  |
| `impl Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:515` | - |  |
| `impl Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:625` | - |  |
| `set_sample_rate` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:733` | - | Attempt to set the device sample rate to the provided rate. |
| `impl Stream::new` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:890` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:898` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:912` | - |  |
| `get_io_buffer_frame_size_range` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:928` | - |  |
| `impl AudioObjectPropertyListener::new` | 3 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:24` | - | Attach the provided callback as a audio object property listener. |
| `impl AudioObjectPropertyListener::remove` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:49` | - | Explicitly remove the property listener. |
| `impl AudioObjectPropertyListener::remove_inner` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:53` | - |  |
| `impl Drop for AudioObjectPropertyListener::drop` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:68` | - |  |
| `property_listener_handler_shim` | 4 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:76` | - | Callback used to call user-provided closure as a property listener. |
| `check_os_status` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:32` | - |  |
| `asbd_from_config` | 2 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:43` | - |  |
| `host_time_to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:71` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:84` | - |  |
| `impl From<coreaudio::Error> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:93` | - |  |
| `impl From<coreaudio::Error> for SupportedStreamConfigsError::from` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:106` | - |  |
| `impl From<coreaudio::Error> for DefaultStreamConfigError::from` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:115` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:55` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:61` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:68` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:73` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:80` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:99` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:103` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:120` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:125` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:129` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:133` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:143` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:147` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:153` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:159` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:163` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:167` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:182` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:233` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:248` | - |  |
| `audio_callback_fn` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:264` | - |  |
| `set_timeout` | 6 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:344` | - |  |
| `impl Default for Devices::default` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:370` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:378` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:389` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:394` | - |  |
| `is_webaudio_available` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:403` | - |  |
| `valid_config` | 2 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:408` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:417` | - |  |
| `impl Device::new_device` | 4 | `crates/audio-device-cpal/src/host/jack/device.rs:38` | - |  |
| `impl Device::default_output_device` | 3 | `crates/audio-device-cpal/src/host/jack/device.rs:67` | - |  |
| `impl Device::default_input_device` | 3 | `crates/audio-device-cpal/src/host/jack/device.rs:81` | - |  |
| `impl Device::default_config` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:95` | - |  |
| `impl Device::supported_configs` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:111` | - |  |
| `impl Device::is_input` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:131` | - |  |
| `impl Device::is_output` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:135` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:145` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:149` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:155` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:164` | - | Returns the default input config |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:171` | - | Returns the default output config |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/jack/device.rs:175` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/jack/device.rs:214` | - |  |
| `impl PartialEq for Device::eq` | 2 | `crates/audio-device-cpal/src/host/jack/device.rs:256` | - |  |
| `impl Hash for Device::hash` | 2 | `crates/audio-device-cpal/src/host/jack/device.rs:265` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/jack/mod.rs:33` | - |  |
| `impl Host::set_connect_automatically` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:46` | - | Set whether the ports should automatically be connected to system |
| `impl Host::set_start_server_automatically` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:51` | - | Set whether a JACK server should be automatically started if it isn't already. |
| `impl Host::input_device_with_name` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:55` | - |  |
| `impl Host::output_device_with_name` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:60` | - |  |
| `impl Host::initialize_default_devices` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:65` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/jack/mod.rs:107` | - | JACK is available if |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:111` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:115` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:124` | - |  |
| `get_client_options` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:134` | - |  |
| `get_client` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:143` | - |  |
| `impl Stream::new_input` | 4 | `crates/audio-device-cpal/src/host/jack/stream.rs:26` | - |  |
| `impl Stream::new_output` | 4 | `crates/audio-device-cpal/src/host/jack/stream.rs:91` | - |  |
| `impl Stream::connect_to_system_outputs` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:158` | - | Connect to the standard system outputs in jack, system:playback_1 and system:playback_2 |
| `impl Stream::connect_to_system_inputs` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:184` | - | Connect to the standard system outputs in jack, system:capture_1 and system:capture_2 |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:210` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:215` | - |  |
| `impl LocalProcessHandler::new` | 8 | `crates/audio-device-cpal/src/host/jack/stream.rs:241` | - |  |
| `temp_buffer_to_data` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:273` | - |  |
| `impl jack::ProcessHandler for LocalProcessHandler::process` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:282` | - |  |
| `impl jack::ProcessHandler for LocalProcessHandler::buffer_size` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:378` | - |  |
| `micros_to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:399` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:407` | - |  |
| `impl JackNotificationHandler::new` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:422` | - |  |
| `impl JackNotificationHandler::send_error` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:429` | - |  |
| `impl jack::NotificationHandler for JackNotificationHandler::shutdown` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:439` | - |  |
| `impl jack::NotificationHandler for JackNotificationHandler::sample_rate` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:443` | - |  |
| `impl jack::NotificationHandler for JackNotificationHandler::xrun` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:459` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/null/mod.rs:27` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/null/mod.rs:33` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:44` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:49` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:56` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:63` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:68` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/null/mod.rs:72` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/null/mod.rs:88` | - | Create an output stream. |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/null/mod.rs:108` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:112` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:116` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:120` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:126` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:130` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:139` | - |  |
| `impl Iterator for SupportedInputConfigs::next` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:148` | - |  |
| `impl Iterator for SupportedOutputConfigs::next` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:157` | - |  |
| `with_attached` | 1 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:14` | - |  |
| `get_min_buffer_size` | 4 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:23` | - |  |
| `get_audio_track_min_buffer_size` | 3 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:46` | - |  |
| `get_audio_record_min_buffer_size` | 3 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:55` | - |  |
| `to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:11` | - |  |
| `stream_instant` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:18` | - |  |
| `impl From<oboe::Error> for StreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:30` | - |  |
| `impl From<oboe::Error> for PlayStreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:43` | - |  |
| `impl From<oboe::Error> for PauseStreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:56` | - |  |
| `impl From<oboe::Error> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:69` | - |  |
| `impl CpalInputCallback<I, C>::new` | 2 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:18` | - |  |
| `impl CpalInputCallback<I, C>::make_callback_info` | 2 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:32` | - |  |
| `impl oboe::AudioInputCallback for CpalInputCallback<T, C>::on_error_before_close` | 3 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:51` | - |  |
| `impl oboe::AudioInputCallback for CpalInputCallback<T, C>::on_error_after_close` | 3 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:59` | - |  |
| `impl oboe::AudioInputCallback for CpalInputCallback<T, C>::on_audio_ready` | 3 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:67` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/oboe/mod.rs:51` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/oboe/mod.rs:60` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:64` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:77` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:81` | - |  |
| `buffer_size_range_for_params` | 4 | `crates/audio-device-cpal/src/host/oboe/mod.rs:86` | - |  |
| `default_supported_configs` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:107` | - |  |
| `device_supported_configs` | 2 | `crates/audio-device-cpal/src/host/oboe/mod.rs:142` | - |  |
| `configure_for_device` | 3 | `crates/audio-device-cpal/src/host/oboe/mod.rs:205` | - |  |
| `build_input_stream` | 5 | `crates/audio-device-cpal/src/host/oboe/mod.rs:222` | - |  |
| `build_output_stream` | 5 | `crates/audio-device-cpal/src/host/oboe/mod.rs:246` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:275` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:282` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:292` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:302` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:313` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/oboe/mod.rs:324` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/oboe/mod.rs:398` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:474` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:487` | - |  |
| `impl CpalOutputCallback<I, C>::new` | 2 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:18` | - |  |
| `impl CpalOutputCallback<I, C>::make_callback_info` | 2 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:32` | - |  |
| `impl oboe::AudioOutputCallback for CpalOutputCallback<T, C>::on_error_before_close` | 3 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:51` | - |  |
| `impl oboe::AudioOutputCallback for CpalOutputCallback<T, C>::on_error_after_close` | 3 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:59` | - |  |
| `impl oboe::AudioOutputCallback for CpalOutputCallback<T, C>::on_audio_ready` | 3 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:67` | - |  |
| `impl Drop for ComInitialized::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/com.rs:44` | - |  |
| `com_initialized` | 0 | `crates/audio-device-cpal/src/host/wasapi/com.rs:55` | - | Ensures that COM is initialized in this thread. |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:57` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:61` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:67` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:73` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:77` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/wasapi/device.rs:81` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/wasapi/device.rs:101` | - |  |
| `impl Drop for WaveFormatExPtr::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:130` | - |  |
| `immendpoint_from_immdevice` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:137` | - |  |
| `data_flow_from_immendpoint` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:143` | - |  |
| `is_format_supported` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:150` | - |  |
| `format_from_waveformatex_ptr` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:196` | - |  |
| `cmp_guid` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:200` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:282` | - |  |
| `impl Device::from_immdevice` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:335` | - |  |
| `impl Device::ensure_future_audio_client` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:343` | - | Ensures that `future_audio_client` contains a `Some` and returns a locked mutex to it. |
| `impl Device::build_audioclient` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:363` | - | Returns an uninitialized `IAudioClient`. |
| `impl Device::supported_formats` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:379` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:466` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:477` | - |  |
| `impl Device::default_format` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:492` | - |  |
| `impl Device::data_flow` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:520` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:525` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:533` | - |  |
| `impl Device::build_input_stream_raw_inner` | 3 | `crates/audio-device-cpal/src/host/wasapi/device.rs:542` | - |  |
| `impl Device::build_output_stream_raw_inner` | 3 | `crates/audio-device-cpal/src/host/wasapi/device.rs:665` | - |  |
| `impl PartialEq for Device::eq` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:771` | - |  |
| `impl Drop for IdRAII::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:783` | - |  |
| `impl fmt::Debug for Device::fmt` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:813` | - |  |
| `impl From<Audio::IMMDevice> for Endpoint::from` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:822` | - |  |
| `impl Endpoint::data_flow` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:831` | - |  |
| `get_enumerator` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:838` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:872` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:897` | - |  |
| `impl Iterator for Devices::size_hint` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:910` | - |  |
| `default_device` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:917` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:928` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:932` | - |  |
| `get_audio_clock` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:937` | - | Get the audio clock used to produce `StreamInstant`s. |
| `config_to_waveformatextensible` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:950` | - |  |
| `buffer_size_to_duration` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:1012` | - |  |
| `buffer_duration_to_frames` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:1019` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:25` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:34` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:39` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:43` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:47` | - |  |
| `impl From<windows::core::Error> for BackendSpecificError::from` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:53` | - |  |
| `trait ErrDeviceNotAvailable::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:61` | - |  |
| `impl ErrDeviceNotAvailable for crate::BuildStreamError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:65` | - |  |
| `impl ErrDeviceNotAvailable for crate::SupportedStreamConfigsError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:71` | - |  |
| `impl ErrDeviceNotAvailable for crate::DefaultStreamConfigError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:77` | - |  |
| `impl ErrDeviceNotAvailable for crate::StreamError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:83` | - |  |
| `windows_err_to_cpal_err` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:88` | - |  |
| `windows_err_to_cpal_err_message` | 2 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:92` | - |  |
| `impl Stream::stop_and_reset` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:92` | - | Stop the WASAPI client and reset its pending stream buffer on the |
| `impl Stream::new_input` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:99` | - |  |
| `impl Stream::new_output` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:132` | - |  |
| `impl Stream::push_command` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:166` | - |  |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:177` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:188` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:193` | - |  |
| `impl Drop for StreamInner::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:202` | - |  |
| `stop_then_reset` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:212` | - |  |
| `process_commands` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:222` | - |  |
| `stop_failure_never_resets_and_reset_failure_remains_stopped` | 0 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:269` | - |  |
| `wait_for_handle_signal` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:286` | - |  |
| `get_available_frames` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:308` | - |  |
| `run_input` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:318` | - |  |
| `run_output` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:347` | - |  |
| `boost_current_thread_priority` | 0 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:376` | - |  |
| `process_commands_and_await_signal` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:392` | - |  |
| `process_input` | 4 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:425` | - |  |
| `process_output` | 4 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:494` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:546` | - | Convert the given duration in frames at the given sample rate to a `std::time::Duration`. |
| `stream_instant` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:556` | - | Use the stream's `IAudioClock` to produce the current stream instant. |
| `input_timestamp` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:577` | - | Produce the input stream timestamp. |
| `output_timestamp` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:599` | - | Produce the output stream timestamp. |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:49` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:58` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:63` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:67` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:71` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:77` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:84` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:89` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:97` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:117` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:123` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:142` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:147` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:154` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:161` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:166` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:170` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:187` | - | Create an output stream. |
| `impl Stream::audio_context` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:400` | - | Return the [`AudioContext`](https://developer.mozilla.org/docs/Web/API/AudioContext) used |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:406` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:441` | - |  |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:454` | - |  |
| `impl Default for Devices::default` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:460` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:469` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:480` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:486` | - |  |
| `is_webaudio_available` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:495` | - |  |
| `valid_config` | 2 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:504` | - |  |
| `buffer_time_step_secs` | 2 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:512` | - |  |
| `copy_to_channel` | 3 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:523` | - |  |
| `impl Mul<T> for SampleRate::mul` | 2 | `crates/audio-device-cpal/src/lib.rs:204` | - |  |
| `impl Div<T> for SampleRate::div` | 2 | `crates/audio-device-cpal/src/lib.rs:214` | - |  |
| `impl wasm_bindgen::describe::WasmDescribe for BufferSize::describe` | 0 | `crates/audio-device-cpal/src/lib.rs:241` | - |  |
| `impl wasm_bindgen::convert::IntoWasmAbi for BufferSize::into_abi` | 1 | `crates/audio-device-cpal/src/lib.rs:247` | - |  |
| `impl SupportedStreamConfig::new` | 4 | `crates/audio-device-cpal/src/lib.rs:377` | - |  |
| `impl SupportedStreamConfig::channels` | 1 | `crates/audio-device-cpal/src/lib.rs:391` | - |  |
| `impl SupportedStreamConfig::sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:395` | - |  |
| `impl SupportedStreamConfig::buffer_size` | 1 | `crates/audio-device-cpal/src/lib.rs:399` | - |  |
| `impl SupportedStreamConfig::sample_format` | 1 | `crates/audio-device-cpal/src/lib.rs:403` | - |  |
| `impl SupportedStreamConfig::config` | 1 | `crates/audio-device-cpal/src/lib.rs:407` | - |  |
| `impl StreamInstant::duration_since` | 2 | `crates/audio-device-cpal/src/lib.rs:420` | - | The amount of time elapsed from another instant to this one. |
| `impl StreamInstant::add` | 2 | `crates/audio-device-cpal/src/lib.rs:435` | - | Returns the instant in time after the given duration has passed. |
| `impl StreamInstant::sub` | 2 | `crates/audio-device-cpal/src/lib.rs:446` | - | Returns the instant in time one `duration` ago. |
| `impl StreamInstant::as_nanos` | 1 | `crates/audio-device-cpal/src/lib.rs:452` | - |  |
| `impl StreamInstant::from_nanos` | 1 | `crates/audio-device-cpal/src/lib.rs:457` | - |  |
| `impl StreamInstant::from_nanos_i128` | 1 | `crates/audio-device-cpal/src/lib.rs:464` | - |  |
| `impl StreamInstant::from_secs_f64` | 1 | `crates/audio-device-cpal/src/lib.rs:476` | - |  |
| `impl StreamInstant::new` | 2 | `crates/audio-device-cpal/src/lib.rs:482` | - |  |
| `impl InputCallbackInfo::timestamp` | 1 | `crates/audio-device-cpal/src/lib.rs:489` | - | The timestamp associated with the call to an input stream's data callback. |
| `impl OutputCallbackInfo::timestamp` | 1 | `crates/audio-device-cpal/src/lib.rs:496` | - | The timestamp associated with the call to an output stream's data callback. |
| `impl Data::from_parts` | 3 | `crates/audio-device-cpal/src/lib.rs:512` | - |  |
| `impl Data::sample_format` | 1 | `crates/audio-device-cpal/src/lib.rs:525` | - | The sample format of the internal audio data. |
| `impl Data::len` | 1 | `crates/audio-device-cpal/src/lib.rs:533` | - | The full length of the buffer in samples. |
| `impl Data::bytes` | 1 | `crates/audio-device-cpal/src/lib.rs:540` | - | The raw slice of memory representing the underlying audio data as a slice of bytes. |
| `impl Data::bytes_mut` | 1 | `crates/audio-device-cpal/src/lib.rs:550` | - | The raw slice of memory representing the underlying audio data as a slice of bytes. |
| `impl Data::as_slice` | 1 | `crates/audio-device-cpal/src/lib.rs:560` | - | Access the data as a slice of sample type `T`. |
| `impl Data::as_slice_mut` | 1 | `crates/audio-device-cpal/src/lib.rs:576` | - | Access the data as a slice of sample type `T`. |
| `impl SupportedStreamConfigRange::new` | 5 | `crates/audio-device-cpal/src/lib.rs:596` | - |  |
| `impl SupportedStreamConfigRange::channels` | 1 | `crates/audio-device-cpal/src/lib.rs:612` | - |  |
| `impl SupportedStreamConfigRange::min_sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:616` | - |  |
| `impl SupportedStreamConfigRange::max_sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:620` | - |  |
| `impl SupportedStreamConfigRange::buffer_size` | 1 | `crates/audio-device-cpal/src/lib.rs:624` | - |  |
| `impl SupportedStreamConfigRange::sample_format` | 1 | `crates/audio-device-cpal/src/lib.rs:628` | - |  |
| `impl SupportedStreamConfigRange::with_sample_rate` | 2 | `crates/audio-device-cpal/src/lib.rs:639` | - | Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size. |
| `impl SupportedStreamConfigRange::try_with_sample_rate` | 2 | `crates/audio-device-cpal/src/lib.rs:648` | - | Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size. |
| `impl SupportedStreamConfigRange::with_max_sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:663` | - | Turns this [`SupportedStreamConfigRange`] into a [`SupportedStreamConfig`] corresponding to the maximum samples rate. |
| `impl SupportedStreamConfigRange::cmp_default_heuristics` | 2 | `crates/audio-device-cpal/src/lib.rs:696` | - | A comparison function which compares two [`SupportedStreamConfigRange`]s in terms of their priority of |
| `test_cmp_default_heuristics` | 0 | `crates/audio-device-cpal/src/lib.rs:744` | - |  |
| `impl From<SupportedStreamConfig> for StreamConfig::from` | 1 | `crates/audio-device-cpal/src/lib.rs:813` | - |  |
| `test_stream_instant` | 0 | `crates/audio-device-cpal/src/lib.rs:840` | - |  |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:596` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:614` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:632` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:650` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:674` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:692` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:721` | - | The default host for the current compilation target platform. |
| `impl Default for NotSendSyncAcrossAllPlatforms::default` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:739` | - |  |
| `impl SampleFormat::sample_size` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:55` | - | Returns the size in bytes of a sample of this format. |
| `impl SampleFormat::is_int` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:70` | - |  |
| `impl SampleFormat::is_uint` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:80` | - |  |
| `impl SampleFormat::is_float` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:90` | - |  |
| `impl Display for SampleFormat::fmt` | 2 | `crates/audio-device-cpal/src/samples_formats.rs:96` | - |  |
| `trait HostTrait::is_available` | 0 | `crates/audio-device-cpal/src/traits.rs:40` | - | Whether or not the host is available on the system. |
| `trait HostTrait::devices` | 1 | `crates/audio-device-cpal/src/traits.rs:45` | - | An iterator yielding all [`Device`](DeviceTrait)s currently available to the host on the system. |
| `trait HostTrait::default_input_device` | 1 | `crates/audio-device-cpal/src/traits.rs:50` | - | The default input audio device on the system. |
| `trait HostTrait::default_output_device` | 1 | `crates/audio-device-cpal/src/traits.rs:55` | - | The default output audio device on the system. |
| `trait HostTrait::input_devices` | 1 | `crates/audio-device-cpal/src/traits.rs:61` | - | An iterator yielding all `Device`s currently available to the system that support one or more |
| `supports_input` | 1 | `crates/audio-device-cpal/src/traits.rs:62` | - |  |
| `trait HostTrait::output_devices` | 1 | `crates/audio-device-cpal/src/traits.rs:75` | - | An iterator yielding all `Device`s currently available to the system that support one or more |
| `supports_output` | 1 | `crates/audio-device-cpal/src/traits.rs:76` | - |  |
| `trait DeviceTrait::name` | 1 | `crates/audio-device-cpal/src/traits.rs:102` | - | The human-readable name of the device. |
| `trait DeviceTrait::supported_input_configs` | 1 | `crates/audio-device-cpal/src/traits.rs:107` | - | An iterator yielding formats that are supported by the backend. |
| `trait DeviceTrait::supported_output_configs` | 1 | `crates/audio-device-cpal/src/traits.rs:114` | - | An iterator yielding output stream formats that are supported by the device. |
| `trait DeviceTrait::default_input_config` | 1 | `crates/audio-device-cpal/src/traits.rs:119` | - | The default input stream format for the device. |
| `trait DeviceTrait::default_output_config` | 1 | `crates/audio-device-cpal/src/traits.rs:122` | - | The default output stream format for the device. |
| `trait DeviceTrait::build_input_stream` | 5 | `crates/audio-device-cpal/src/traits.rs:125` | - | Create an input stream. |
| `trait DeviceTrait::build_output_stream` | 5 | `crates/audio-device-cpal/src/traits.rs:153` | - | Create an output stream. |
| `trait DeviceTrait::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/traits.rs:181` | - | Create a dynamically typed input stream. |
| `trait DeviceTrait::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/traits.rs:194` | - | Create a dynamically typed output stream. |
| `trait StreamTrait::play` | 1 | `crates/audio-device-cpal/src/traits.rs:213` | - | Run the stream. |
| `trait StreamTrait::pause` | 1 | `crates/audio-device-cpal/src/traits.rs:220` | - | Some devices support pausing the audio stream. This can be useful for saving energy in |
