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
    pub const fn new() -> Self { Self { events: [None; 64], head:0, tail:0 } }

    pub fn push(&mut self, event: InputEvent) -> bool {
        let next=(self.tail+1)%self.events.len();
        if next==self.head { return false; }
        self.events[self.tail]=Some(event); self.tail=next; true
    }

    pub fn pop(&mut self) -> Option<InputEvent> {
        if self.head==self.tail{return None;}
        let e=self.events[self.head]; self.events[self.head]=None;
        self.head=(self.head+1)%self.events.len(); e
    }
}
