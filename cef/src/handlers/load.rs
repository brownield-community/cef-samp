use crate::browser::{Browser, Frame};
use std::sync::Arc;

pub trait LoadHandler {
    fn on_load_end(self: &Arc<Self>, _browser: Browser, _frame: Frame, _status_code: i32) {}

    fn on_loading_state_change(
        self: &Arc<Self>, _browser: Browser, _is_loading: bool, _can_go_back: bool,
        _can_go_forward: bool,
    ) {
    }
}
