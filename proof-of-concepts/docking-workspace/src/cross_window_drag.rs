//! Windows-only exploratory bridge for tab drags between egui native viewports.
//!
//! egui_dock owns dragging *within* a DockArea. This module only observes the
//! physical mouse button and identifies which REAL OS window lies under it.
//! No global hooks, OLE data transfer, mouse capture or synthetic clicks.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowArea {
    Main,
    Native(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropEdge {
    Centre,
    Left,
    Right,
    Above,
    Below,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropTarget {
    pub window: WindowArea,
    pub edge: DropEdge,
}

pub fn edge_for_relative_point(x: f32, y: f32) -> Option<DropEdge> {
    if !(0.0..=1.0).contains(&x) || !(0.0..=1.0).contains(&y) {
        return None;
    }
    // A simple whole-window overlay for the foundation test. Accurate
    // per-pane hit-testing is intentionally a *separate* design question.
    let edge = if x < 0.25 {
        DropEdge::Left
    } else if x > 0.75 {
        DropEdge::Right
    } else if y < 0.25 {
        DropEdge::Above
    } else if y > 0.75 {
        DropEdge::Below
    } else {
        DropEdge::Centre
    };
    Some(edge)
}

#[derive(Clone, Copy, Debug)]
pub struct PointerSnapshot {
    pub left_down: bool,
    pub target: Option<DropTarget>,
}

#[cfg(windows)]
pub fn pointer_snapshot() -> Option<PointerSnapshot> {
    use windows_sys::Win32::Foundation::{POINT, RECT};
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        ClientToScreen, GA_ROOT, GetAncestor, GetClientRect, GetCursorPos,
        GetWindowTextW, GetWindowThreadProcessId, WindowFromPoint,
    };

    // SAFETY: Win32 samples the physical left mouse button without changing it.
    let left_down = unsafe { (GetAsyncKeyState(0x01) as u16 & 0x8000) != 0 };
    let mut pointer = POINT { x: 0, y: 0 };
    // SAFETY: pointer is a valid writable POINT.
    if unsafe { GetCursorPos(&mut pointer) } == 0 {
        return None;
    }
    // SAFETY: WindowFromPoint / GetAncestor are read-only HWND queries.
    let hit = unsafe { WindowFromPoint(pointer) };
    if hit.is_null() {
        return Some(PointerSnapshot { left_down, target: None });
    }
    let root = unsafe { GetAncestor(hit, GA_ROOT) };
    if root.is_null() {
        return Some(PointerSnapshot { left_down, target: None });
    }

    let mut owner_pid = 0_u32;
    // SAFETY: Windows owns this HWND and fills the process ID.
    unsafe { GetWindowThreadProcessId(root, &mut owner_pid) };
    if owner_pid != std::process::id() {
        return Some(PointerSnapshot { left_down, target: None });
    }

    let mut title_buffer = [0_u16; 160];
    // SAFETY: writable UTF-16 output buffer of the advertised length.
    let len = unsafe { GetWindowTextW(root, title_buffer.as_mut_ptr(), title_buffer.len() as i32) };
    let title = String::from_utf16_lossy(&title_buffer[..len.max(0) as usize]);
    let window = if title == "Comic Panel Composer — Docking PoC" {
        Some(WindowArea::Main)
    } else if let Some(prefix) = title.strip_suffix(" — native Windows window") {
        match prefix {
            "Pages" => Some(WindowArea::Native(0)),
            "Layers" => Some(WindowArea::Native(1)),
            "Inspector" => Some(WindowArea::Native(2)),
            _ => None,
        }
    } else {
        None
    };

    let target = window.and_then(|window| {
        let mut client = RECT::default();
        let mut origin = POINT { x: 0, y: 0 };
        // SAFETY: client and origin are writable POD; root is a live HWND.
        if unsafe { GetClientRect(root, &mut client) } == 0
            || unsafe { ClientToScreen(root, &mut origin) } == 0
        {
            return None;
        }
        let width = client.right - client.left;
        let height = client.bottom - client.top;
        if width < 100 || height < 100 {
            return None;
        }
        let x = (pointer.x - origin.x) as f32 / width as f32;
        let y = (pointer.y - origin.y) as f32 / height as f32;
        edge_for_relative_point(x, y).map(|edge| DropTarget { window, edge })
    });
    Some(PointerSnapshot { left_down, target })
}

#[cfg(not(windows))]
pub fn pointer_snapshot() -> Option<PointerSnapshot> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn centre_means_tab_grouping() {
        assert_eq!(edge_for_relative_point(0.5, 0.5), Some(DropEdge::Centre));
    }
    #[test]
    fn four_regions_produce_the_corresponding_split() {
        assert_eq!(edge_for_relative_point(0.1, 0.5), Some(DropEdge::Left));
        assert_eq!(edge_for_relative_point(0.9, 0.5), Some(DropEdge::Right));
        assert_eq!(edge_for_relative_point(0.5, 0.1), Some(DropEdge::Above));
        assert_eq!(edge_for_relative_point(0.5, 0.9), Some(DropEdge::Below));
    }
    #[test]
    fn outside_a_client_area_is_not_a_drop() {
        assert_eq!(edge_for_relative_point(1.2, 0.5), None);
        assert_eq!(edge_for_relative_point(-0.1, 0.5), None);
    }
}
