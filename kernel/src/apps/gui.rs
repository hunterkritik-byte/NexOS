use crate::graphics::window::{Rect, WindowManager};

pub struct GuiApplication {
    pub window_id: usize,
}

impl GuiApplication {
    pub fn launch(wm: &mut WindowManager, bounds: Rect) -> Option<Self> {
        wm.create(bounds).map(|window_id| Self { window_id })
    }
}
