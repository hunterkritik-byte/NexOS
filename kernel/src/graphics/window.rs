#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    /// Returns whether a point lies inside this rectangle.
    /// Saturating arithmetic prevents coordinate overflow from wrapping.
    pub fn contains(&self, x: usize, y: usize) -> bool {
        self.width > 0
            && self.height > 0
            && x >= self.x
            && y >= self.y
            && x < self.x.saturating_add(self.width)
            && y < self.y.saturating_add(self.height)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub bounds: Rect,
    pub focused: bool,
}

const MAX_WINDOWS: usize = 16;

/// Fixed-capacity window registry with stable IDs and a separate z-order.
/// The final z-order entry is the front-most window.
pub struct WindowManager {
    windows: [Option<Window>; MAX_WINDOWS],
    z_order: [usize; MAX_WINDOWS],
    count: usize,
}

impl WindowManager {
    pub const fn new() -> Self {
        Self {
            windows: [const { None }; MAX_WINDOWS],
            z_order: [0; MAX_WINDOWS],
            count: 0,
        }
    }

    /// Creates a window and returns its stable ID. Empty windows are rejected.
    pub fn create(&mut self, bounds: Rect) -> Option<usize> {
        if bounds.width == 0 || bounds.height == 0 || self.count == MAX_WINDOWS {
            return None;
        }

        let id = self.windows.iter().position(Option::is_none)?;
        for window in self.windows.iter_mut().flatten() {
            window.focused = false;
        }

        self.windows[id] = Some(Window { bounds, focused: true });
        self.z_order[self.count] = id;
        self.count += 1;
        Some(id)
    }

    pub fn get(&self, id: usize) -> Option<&Window> {
        self.windows.get(id)?.as_ref()
    }

    /// Focuses a window and raises it to the front-most position.
    pub fn focus(&mut self, id: usize) -> bool {
        if self.get(id).is_none() {
            return false;
        }

        for window in self.windows.iter_mut().flatten() {
            window.focused = false;
        }
        if let Some(window) = self.windows[id].as_mut() {
            window.focused = true;
        }

        let Some(position) = self.z_order[..self.count]
            .iter()
            .position(|&window_id| window_id == id)
        else {
            return false;
        };
        for index in position..self.count - 1 {
            self.z_order[index] = self.z_order[index + 1];
        }
        self.z_order[self.count - 1] = id;
        true
    }

    /// Returns the front-most window under a screen point.
    pub fn window_at(&self, x: usize, y: usize) -> Option<usize> {
        self.z_order[..self.count]
            .iter()
            .rev()
            .copied()
            .find(|&id| self.windows[id].as_ref().is_some_and(|w| w.bounds.contains(x, y)))
    }

    pub fn move_window(&mut self, id: usize, x: usize, y: usize) -> bool {
        let Some(window) = self.windows.get_mut(id).and_then(Option::as_mut) else {
            return false;
        };
        window.bounds.x = x;
        window.bounds.y = y;
        true
    }

    /// Resizes a window. Zero-sized windows are rejected.
    pub fn resize_window(&mut self, id: usize, width: usize, height: usize) -> bool {
        if width == 0 || height == 0 {
            return false;
        }
        let Some(window) = self.windows.get_mut(id).and_then(Option::as_mut) else {
            return false;
        };
        window.bounds.width = width;
        window.bounds.height = height;
        true
    }

    /// Closes a window without changing the IDs of remaining windows.
    pub fn close(&mut self, id: usize) -> bool {
        if self.windows.get(id).and_then(Option::as_ref).is_none() {
            return false;
        }
        let was_focused = self.windows[id].as_ref().is_some_and(|w| w.focused);
        self.windows[id] = None;
        let Some(position) = self.z_order[..self.count]
            .iter()
            .position(|&window_id| window_id == id)
        else {
            return false;
        };
        for index in position..self.count - 1 {
            self.z_order[index] = self.z_order[index + 1];
        }
        self.count -= 1;

        if was_focused {
            for window in self.windows.iter_mut().flatten() {
                window.focused = false;
            }
            if self.count > 0 {
                let next = self.z_order[self.count - 1];
                if let Some(window) = self.windows[next].as_mut() {
                    window.focused = true;
                }
            }
        }
        true
    }

    pub fn len(&self) -> usize { self.count }

    pub fn is_empty(&self) -> bool { self.count == 0 }
}

impl Default for WindowManager {
    fn default() -> Self { Self::new() }
}
