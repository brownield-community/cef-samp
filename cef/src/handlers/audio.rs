use crate::browser::Browser;
use cef_sys::cef_audio_parameters_t;
use std::sync::Arc;

pub trait AudioHandler {
    fn get_audio_parameters(
        self: &Arc<Self>, _browser: Browser, _params: &mut cef_audio_parameters_t,
    ) -> bool {
        true
    }

    fn on_audio_stream_packet(
        self: &Arc<Self>, _browser: Browser, _stream_id: i32, _data: *mut *const f32, _frames: i32,
        _pts: i64,
    ) {
    }

    fn on_audio_stream_started(
        self: &Arc<Self>, _browser: Browser, _stream_id: i32, _channels: i32, _channel_layout: i32,
        _sample_rate: i32, _frames_per_buffer: i32,
    ) {
    }

    fn on_audio_stream_stopped(self: &Arc<Self>, _browser: Browser, _stream_id: i32) {}

    fn on_audio_stream_error(self: &Arc<Self>, _browser: Browser, _error: String) {}
}
