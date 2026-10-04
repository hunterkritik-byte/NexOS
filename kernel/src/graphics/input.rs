#[derive(Clone, Copy)]
pub enum InputEvent {
    Key { code: u16, pressed: bool },
    MouseMove { x: i32, y: i32 },
    MouseButton { button: u8, pressed: bool },
}

pub struct InputQueue {
    events: [Option<InputEvent>; 64],
    head: usize,
    tail: usize,
}

impl InputQueue {
    pub const fn new() -> Self { Self { events: [None; 64], head: 0, tail: 0 } }

    pub fn push(&mut self, event: InputEvent) -> bool {
        let next = (self.tail + 1) % self.events.len();
        if next == self.head { return false; }
        self.events[self.tail] = Some(event);
        self.tail = next;
        true
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
        if self.head == self.tail { return None; }
        let event = self.events[self.head];
        self.events[self.head] = None;
        self.head = (self.head + 1) % self.events.len();
        event
    }
}

use super::window::{Rect, WindowManager};

/// Routes queued pointer events to the window manager.
///
/// Mouse button 0 is treated as the primary button. Pressing it focuses the
/// front-most window under the pointer; pressing its title bar starts a drag.
/// A matching release ends the drag. Hardware drivers must enqueue events
/// before this controller can be used by a running desktop.
pub struct DesktopInput {
    cursor_x: usize,
    cursor_y: usize,
    dragging: Option<(usize, usize, usize)>,
}

impl DesktopInput {
    pub const fn new() -> Self {
        Self { cursor_x: 0, cursor_y: 0, dragging: None }
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    /// Applies one event and returns whether it changed desktop interaction state.
    pub fn handle_event(&mut self, event: InputEvent, windows: &mut WindowManager) -> bool {
        match event {
            InputEvent::MouseMove { x, y } => {
                self.cursor_x = nonnegative_coordinate(x);
                self.cursor_y = nonnegative_coordinate(y);

                if let Some((id, offset_x, offset_y)) = self.dragging {
                    return windows.move_window(
                        id,
                        self.cursor_x.saturating_sub(offset_x),
                        self.cursor_y.saturating_sub(offset_y),
                    );
                }
                false
            }
            InputEvent::MouseButton { button: 0, pressed: true } => {
                let Some(id) = windows.window_at(self.cursor_x, self.cursor_y) else {
                    self.dragging = None;
                    return false;
                };
                if !windows.focus(id) {
                    self.dragging = None;
                    return false;
                }

                let Some(window) = windows.get(id).copied() else {
                    self.dragging = None;
                    return false;
                };
                if in_title_bar(window.bounds, self.cursor_x, self.cursor_y) {
                    self.dragging = Some((
                        id,
                        self.cursor_x.saturating_sub(window.bounds.x),
                        self.cursor_y.saturating_sub(window.bounds.y),
                    ));
                } else {
                    self.dragging = None;
                }
                true
            }
            InputEvent::MouseButton { button: 0, pressed: false } => {
                let was_dragging = self.dragging.is_some();
                self.dragging = None;
                was_dragging
            }
            InputEvent::MouseButton { .. } | InputEvent::Key { .. } => false,
        }
    }
}

impl Default for DesktopInput {
    fn default() -> Self { Self::new() }
}

fn nonnegative_coordinate(value: i32) -> usize {
    if value < 0 { 0 } else { value as usize }
}

fn in_title_bar(bounds: Rect, x: usize, y: usize) -> bool {
    let title_height = core::cmp::min(20, bounds.height);
    x >= bounds.x
        && x < bounds.x.saturating_add(bounds.width)
        && y >= bounds.y
        && y < bounds.y.saturating_add(title_height)
}
