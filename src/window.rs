use macroquad::prelude::*;

/// Everything will be drawn inside a window, clipping things that go outside
/// The window rectangle is the rectangle defining the window onscreen
/// The offset is how things drawn will be offset to fit inside the window
/// The scale is screen-space
pub(crate) fn set_camera_window(window: Rect, offset: Vec2) {
    let mut  camera = Camera2D::from_display_rect(Rect::new(offset.x, offset.y, window.w, window.h));
    camera.zoom = vec2(camera.zoom.x, -camera.zoom.y);
    camera.viewport = Some((window.x as i32, screen_height() as i32 - window.y as i32 - window.h as i32, window.w as i32, window.h as i32)); // x, y, width, height

    set_camera(&camera);
}