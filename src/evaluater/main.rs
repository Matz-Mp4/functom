use std::io::{self, Write};
use termion::{clear, color, cursor};

struct Point {
    x: u16,
    y: u16,
}

fn plot_points(points: &[Point]) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Clear the terminal
    write!(handle, "{}", clear::All)?;

    // Set the cursor to the top-left corner
    write!(handle, "{}", cursor::Goto(1, 1))?;

    // Define the dimensions of the terminal
    let (term_width, term_height) = termion::terminal_size()?;
    let x_axis_y = term_height / 2; // Y-coordinate of the x-axis
    let y_axis_x = term_width / 2; // X-coordinate of the y-axis

    // Draw x-axis
    write!(handle, "{}", color::Fg(color::White))?;
    for x in 1..=term_width {
        write!(
            handle,
            "{}─",
            cursor::Goto(x, x_axis_y + 1) // Add 1 to adjust for terminal coordinates
        )?;
    }

    // Draw y-axis
    for y in 1..=term_height {
        write!(
            handle,
            "{}│",
            cursor::Goto(y_axis_x + 1, y) // Add 1 to adjust for terminal coordinates
        )?;
    }

    // Plot each point
    write!(handle, "{}", color::Fg(color::Red))?;
    for point in points {
        write!(
            handle,
            "{}{}",
            cursor::Goto(point.x + y_axis_x + 1, -point.y + x_axis_y + 1), // Add 1 to adjust for terminal coordinates
            "•"
        )?;
    }

    // Draw lines between points
    write!(handle, "{}", color::Fg(color::Cyan))?;
    if points.len() >= 2 {
        for i in 0..(points.len() - 1) {
            let start = &points[i];
            let end = &points[i + 1];
            draw_line(
                &mut handle,
                start.x + y_axis_x + 1,
                -start.y + x_axis_y + 1,
                end.x + y_axis_x + 1,
                -end.y + x_axis_y + 1,
            )?;
        }
    }

    // Reset the cursor and color
    write!(handle, "{}{}", cursor::Goto(1, 1), color::Fg(color::Reset))?;

    // Flush the output to make sure it's printed immediately
    handle.flush()?;

    Ok(())
}

fn draw_line<W>(handle: &mut W, x1: u16, y1: u16, x2: u16, y2: u16) -> io::Result<()>
where
    W: Write,
{
    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;

    let (x_step, y_step) = if dx.abs() > dy.abs() {
        (if dx < 0 { -1 } else { 1 }, (dy as f32 / dx as f32).abs())
    } else {
        ((dx as f32 / dy as f32).abs(), if dy < 0 { -1 } else { 1 })
    };

    let mut x = x1 as f32;
    let mut y = y1 as f32;

    for _ in 0..=dx.abs().max(dy.abs()) {
        write!(handle, "{} ", cursor::Goto(x as u16, y as u16))?;
        x += x_step;
        y += y_step;
    }

    Ok(())
}
fn main() {
    let num_points = 100;
    let amplitude = 10;
    let frequency = 0.1;

    let mut points = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let x = i as f32;
        let y = (x * frequency * 2.0 * PI).sin() * amplitude;
        points.push(Point { x, y.round() as u16 });
    }

    if let Err(err) = plot_points(&points) {
        eprintln!("Error plotting points: {}", err);
    }
}
