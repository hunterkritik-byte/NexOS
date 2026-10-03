#[derive(Clone, Copy)]
pub struct Rect { pub x: usize, pub y: usize, pub width: usize, pub height: usize }

pub struct Window {
    pub bounds: Rect,
    pub focused: bool,
}

pub struct WindowManager {
    windows: [Option<Window>; 16],
    count: usize,
}

impl WindowManager {
    pub const fn new() -> Self { Self { windows: [const { None }; 16], count: 0 } }

    pub fn create(&mut self, bounds: Rect) -> Option<usize> {
        if self.count >= self.windows.len() { return None; }
        let id=self.count;
        self.windows[id]=Some(Window { bounds, focused: true });
        self.count+=1;
        Some(id)
    }

    pub fn focus(&mut self, id: usize) {
        for (i,w) in self.windows.iter_mut().enumerate() {
            if let Some(w)=w { w.focused=i==id; }
        }
    }
}
