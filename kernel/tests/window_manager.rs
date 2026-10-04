#[path = "../src/graphics/window.rs"]
mod window;

use window::{Rect, WindowManager};

#[test]
fn rectangle_hit_testing_handles_edges_and_overflow() {
    let rect = Rect { x: 10, y: 20, width: 30, height: 40 };
    assert!(rect.contains(10, 20));
    assert!(rect.contains(39, 59));
    assert!(!rect.contains(40, 59));
    assert!(!rect.contains(39, 60));

    let huge = Rect { x: usize::MAX - 2, y: 0, width: 10, height: 1 };
    assert!(huge.contains(usize::MAX - 1, 0));
    assert!(!huge.contains(0, 0));
}

#[test]
fn overlapping_windows_hit_frontmost_window() {
    let mut manager = WindowManager::new();
    let back = manager.create(Rect { x: 0, y: 0, width: 100, height: 100 }).unwrap();
    let front = manager.create(Rect { x: 20, y: 20, width: 50, height: 50 }).unwrap();

    assert_eq!(manager.window_at(25, 25), Some(front));
    assert_eq!(manager.window_at(5, 5), Some(back));
}

#[test]
fn focus_raises_window_without_changing_ids() {
    let mut manager = WindowManager::new();
    let first = manager.create(Rect { x: 0, y: 0, width: 100, height: 100 }).unwrap();
    let second = manager.create(Rect { x: 0, y: 0, width: 100, height: 100 }).unwrap();

    assert!(manager.focus(first));
    assert_eq!(manager.window_at(10, 10), Some(first));
    assert!(manager.get(first).unwrap().focused);
    assert!(!manager.get(second).unwrap().focused);
}

#[test]
fn move_and_resize_update_hit_testing() {
    let mut manager = WindowManager::new();
    let id = manager.create(Rect { x: 0, y: 0, width: 10, height: 10 }).unwrap();

    assert!(manager.move_window(id, 50, 60));
    assert_eq!(manager.window_at(1, 1), None);
    assert_eq!(manager.window_at(55, 65), Some(id));
    assert!(manager.resize_window(id, 20, 30));
    assert_eq!(manager.window_at(69, 89), Some(id));
    assert_eq!(manager.window_at(70, 89), None);
    assert!(!manager.resize_window(id, 0, 1));
}

#[test]
fn close_reuses_slot_and_keeps_other_ids_stable() {
    let mut manager = WindowManager::new();
    let first = manager.create(Rect { x: 0, y: 0, width: 10, height: 10 }).unwrap();
    let second = manager.create(Rect { x: 20, y: 20, width: 10, height: 10 }).unwrap();

    assert!(manager.close(second));
    assert_eq!(manager.len(), 1);
    assert_eq!(manager.window_at(5, 5), Some(first));
    assert_eq!(manager.get(second), None);
    let replacement = manager.create(Rect { x: 40, y: 40, width: 10, height: 10 }).unwrap();
    assert_eq!(replacement, second);
}

#[test]
fn manager_rejects_empty_windows_and_capacity_overflow() {
    let mut manager = WindowManager::new();
    assert_eq!(manager.create(Rect { x: 0, y: 0, width: 0, height: 10 }), None);
    for _ in 0..16 {
        assert!(manager.create(Rect { x: 0, y: 0, width: 1, height: 1 }).is_some());
    }
    assert_eq!(manager.len(), 16);
    assert_eq!(manager.create(Rect { x: 0, y: 0, width: 1, height: 1 }), None);
}
