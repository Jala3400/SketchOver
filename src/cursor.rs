use winit::event_loop::ActiveEventLoop;

pub fn color_circle_cursor(
    event_loop: &ActiveEventLoop,
    radius: i32,
    current_color: u32,
) -> winit::window::CustomCursor {
    let diameter = radius * 2 + 1 as i32;
    // Using a flat vector to store the pattern
    let mut cursor_pattern = vec![0u8; ((diameter * diameter) * 4) as usize];

    let mut x = radius;
    let mut y = 0;
    let mut decision = 1 - radius;

    while x >= y {
        // Draw horizontal lines to fill the circle
        for i in (-x as i32)..=(x as i32) {
            let center = radius as i32;
            set_pixel(
                &mut cursor_pattern,
                center + i,
                center + y,
                diameter,
                current_color,
            );
            set_pixel(
                &mut cursor_pattern,
                center + i,
                center - y,
                diameter,
                current_color,
            );
        }
        for i in (-y as i32)..=(y as i32) {
            let center = radius as i32;
            set_pixel(
                &mut cursor_pattern,
                center + i,
                center + x,
                diameter,
                current_color,
            );
            set_pixel(
                &mut cursor_pattern,
                center + i,
                center - x,
                diameter,
                current_color,
            );
        }

        y += 1;

        if decision <= 0 {
            decision += 2 * y + 1;
        } else {
            x -= 1;
            decision += 2 * (y - x) + 1;
        }
    }

    let hotspot_x = radius as u16;
    let hotspot_y = radius as u16;
    let custom_cursor_source = winit::window::CustomCursor::from_rgba(
        cursor_pattern,
        diameter as u16,
        diameter as u16,
        hotspot_x,
        hotspot_y,
    )
    .unwrap();

    event_loop.create_custom_cursor(custom_cursor_source)
}

pub fn erasing_cursor(event_loop: &ActiveEventLoop, radius: i32) -> winit::window::CustomCursor {
    let diameter = radius * 2 + 1 as i32;
    let mut cursor_pattern = vec![0u8; ((diameter * diameter) * 4) as usize];

    paint_outline_circle(&mut cursor_pattern, radius, 0xFFFFFFFF, radius);
    paint_outline_circle(&mut cursor_pattern, radius - 1, 0xFF000000, radius);

    let hotspot_x = radius as u16;
    let hotspot_y = radius as u16;
    let custom_cursor_source = winit::window::CustomCursor::from_rgba(
        cursor_pattern,
        diameter as u16,
        diameter as u16,
        hotspot_x,
        hotspot_y,
    )
    .unwrap();

    event_loop.create_custom_cursor(custom_cursor_source)
}

fn paint_outline_circle(cursor_pattern: &mut Vec<u8>, radius: i32, color: u32, center: i32) {
    let mut x = radius;
    let mut y = 0;
    let mut decision = 1 - radius;
    let width = center * 2 + 1;

    while x >= y {
        set_pixel(cursor_pattern, center + x, center + y, width, color);
        set_pixel(cursor_pattern, center + x, center - y, width, color);
        set_pixel(cursor_pattern, center - x, center + y, width, color);
        set_pixel(cursor_pattern, center - x, center - y, width, color);
        set_pixel(cursor_pattern, center + y, center + x, width, color);
        set_pixel(cursor_pattern, center + y, center - x, width, color);
        set_pixel(cursor_pattern, center - y, center + x, width, color);
        set_pixel(cursor_pattern, center - y, center - x, width, color);

        y += 1;

        if decision <= 0 {
            decision += 2 * y + 1;
        } else {
            x -= 1;
            decision += 2 * (y - x) + 1;
        }
    }
}

fn set_pixel(cursor_pattern: &mut Vec<u8>, x: i32, y: i32, width: i32, color: u32) {
    let idx = ((y * width + x) * 4) as usize;
    cursor_pattern[idx] = ((color >> 16) & 0xFF) as u8; // R (extract red component)
    cursor_pattern[idx + 1] = ((color >> 8) & 0xFF) as u8; // G (extract green component)
    cursor_pattern[idx + 2] = (color & 0xFF) as u8; // B (extract blue component)
    cursor_pattern[idx + 3] = ((color >> 24) & 0xFF) as u8; // A (extract alpha component)
}
