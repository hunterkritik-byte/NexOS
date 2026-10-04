#[path = "../src/graphics/window.rs"]
mod window;
#[path = "../src/graphics/input.rs"]
mod input;

use input::{DesktopInput, InputEvent};
use window::{Rect, WindowManager};

fn pointer_event(x: i32, y: i32) -> InputEvent {
    InputEvent::MouseMove { x, y }
}

#[test]
fn primary_click_focuses_frontmost_window() {
    let mut windows = WindowManager::new();
    let back = windows.create(Rect { x: 2, y: 2, width: 40, height: 40 }).unwrap();
    let front = windows.create(Rect { x: 10, y: 10, width: 40, height: 40 }).unwrap();
    let mut input = DesktopInput::new();

    input.handle_event(pointer_event(20, 35), &mut windows);
    assert!(input.handle_event(
        InputEvent::MouseButton { button: 0, pressed: true },
        &mut windows,
    ));

    assert_eq!(windows.window_at(20, 35), Some(front));
    assert!(windows.get(front).unwrap().focused);
    assert!(!windows.get(back).unwrap().focused);
}

#[test]
fn dragging_title_bar_moves_window_and_release_stops_drag() {
    let mut windows = WindowManager::new();
    let id = windows.create(Rect { x: 10, y: 10, width: 50, height: 40 }).unwrap();
    let mut input = DesktopInput::new();

    input.handle_event(pointer_event(15, 15), &mut windows);
    assert!(input.handle_event(
        InputEvent::MouseButton { button: 0, pressed: true },
        &mut windows,
    ));
    assert!(input.handle_event(pointer_event(25, 30), &mut windows));
    assert_eq!(windows.get(id).unwrap().bounds, Rect {
        x: 20, y: 25, width: 50, height: 40,
    });

    assert!(input.handle_event(
        InputEvent::MouseButton { button: 0, pressed: false },
        &mut windows,
    ));
    assert!(!input.handle_event(pointer_event(35, 40), &mut windows));
    assert_eq!(windows.get(id).unwrap().bounds.x, 20);
    assert_eq!(windows.get(id).unwrap().bounds.y, 25);
}

#[test]
fn negative_pointer_coordinates_are_clamped() {
    let mut windows = WindowManager::new();
    windows.create(Rect { x: 1, y: 1, width: 20, height: 20 }).unwrap();
    let mut input = DesktopInput::new();

    input.handle_event(pointer_event(-5, -10), &mut windows);
    assert_eq!(input.cursor_position(), (0, 0));
    assert!(!input.handle_event(
        InputEvent::MouseButton { button: 0, pressed: true },
        &mut windows,
    ));
}

#[test]
fn non_primary_buttons_and_keyboard_events_do_not_start_dragging() {
    let mut windows = WindowManager::new();
    windows.create(Rect { x: 0, y: 0, width: 30, height: 30 }).unwrap();
    let mut input = DesktopInput::new();

    input.handle_event(pointer_event(5, 5), &mut windows);
    assert!(!input.handle_event(
        InputEvent::MouseButton { button: 1, pressed: true },
        &mut windows,
    ));
    assert!(!input.handle_event(
        InputEvent::Key { code: 30, pressed: true },
        &mut windows,
    ));
    assert!(!input.handle_event(pointer_event(15, 15), &mut windows));
}

#[test]
fn clicking_outside_windows_does_not_begin_a_drag() {
    let mut windows = WindowManager::new();
    let id = windows.create(Rect { x: 10, y: 10, width: 30, height: 30 }).unwrap();
    let mut input = DesktopInput::new();

    input.handle_event(pointer_event(2, 2), &mut windows);
    assert!(!input.handle_event(
        InputEvent::MouseButton { button: 0, pressed: true },
        &mut windows,
    ));
    input.handle_event(pointer_event(20, 20), &mut windows);
    assert_eq!(windows.get(id).unwrap().bounds.x, 10);
    assert_eq!(windows.get(id).unwrap().bounds.y, 10);
}
