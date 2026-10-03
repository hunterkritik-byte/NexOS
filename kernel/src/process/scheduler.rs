use core::sync::atomic::{AtomicU64, Ordering};

pub type ProcessId = u64;

static NEXT_PID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Exited,
}

#[derive(Clone, Copy)]
pub struct Process {
    pub pid: ProcessId,
    pub state: ProcessState,
    pub entry: usize,
    pub user_stack: usize,
}

impl Process {
    pub const fn new(entry: usize, user_stack: usize) -> Self {
        Self {
            pid: 0,
            state: ProcessState::Ready,
            entry,
            user_stack,
        }
    }

    pub fn assign_pid(&mut self) {
        self.pid = NEXT_PID.fetch_add(1, Ordering::Relaxed);
    }
}

pub struct Scheduler {
    processes: [Option<Process>; 16],
    len: usize,
    current: usize,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            processes: [None; 16],
            len: 0,
            current: 0,
        }
    }

    pub fn add(&mut self, mut process: Process) -> Option<ProcessId> {
        if self.len == self.processes.len() {
            return None;
        }
        process.assign_pid();
        let pid = process.pid;
        self.processes[self.len] = Some(process);
        self.len += 1;
        Some(pid)
    }

    pub fn next(&mut self) -> Option<Process> {
        if self.len == 0 {
            return None;
        }

        for _ in 0..self.len {
            self.current = (self.current + 1) % self.len;
            if let Some(mut process) = self.processes[self.current] {
                if process.state == ProcessState::Ready {
                    process.state = ProcessState::Running;
                    self.processes[self.current] = Some(process);
                    return Some(process);
                }
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
