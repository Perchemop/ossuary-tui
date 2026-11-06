//! Render a [`Frame`] to a standalone SVG.
//!
//! This lets the look of the app be captured and reviewed without a terminal —
//! used to build the preview images in `docs/images/`, and fully testable in
//! the headless core (guide §K.13).

use crate::engine::Frame;

/// Advance width of one monospace cell, in SVG user units.
const CELL_W: f64 = 8.4;
/// Line height, in SVG user units.
const LINE_H: f64 = 16.0;
/// Font size, in SVG user units.
const FONT_SIZE: f64 = 14.0;

/// Colour for a brightness band (empty cells are skipped).
fn color_for(intensity: u8) -> &'static str {
    match intensity {
        1 => "#4d4d4d",
        2 => "#9a9a9a",
        _ => "#f2f2f2",
    }
}

/// Append `ch` to `out`, escaping XML special characters.
fn push_escaped(ch: char, out: &mut String) {
    match ch {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        '"' => out.push_str("&quot;"),
        '\'' => out.push_str("&apos;"),
        other => out.push(other),
    }
}

/// Escape a whole string for use in XML text or an attribute value.
fn escape_str(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        push_escaped(ch, &mut out);
    }
    out
}

/// Render `frame` as an SVG document with the given `title`.
///
/// Each row becomes one `<text>` element; runs of equal brightness become
/// `<tspan>`s positioned at an absolute `x`, so alignment does not depend on
/// the viewer's font metrics.
pub fn render_svg(frame: &Frame, title: &str) -> String {
    let width = frame.width() as f64 * CELL_W;
    let height = frame.height() as f64 * LINE_H;
    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width:.0} {height:.0}\" \
         width=\"{width:.0}\" height=\"{height:.0}\" role=\"img\" aria-label=\"{}\">\n",
        escape_str(title)
    ));
    svg.push_str("<rect width=\"100%\" height=\"100%\" fill=\"#0b0b0b\"/>\n");
    svg.push_str(&format!(
        "<g font-family=\"DejaVu Sans Mono, Menlo, Consolas, monospace\" \
         font-size=\"{FONT_SIZE}\" xml:space=\"preserve\">\n"
    ));
    for y in 0..frame.height() {
        let baseline = (y as f64 + 0.8) * LINE_H;
        svg.push_str(&format!("<text x=\"0\" y=\"{baseline:.1}\">"));
        let mut x = 0;
        while x < frame.width() {
            let intensity = frame.cell(x, y).intensity;
            let start = x;
            let mut run = String::new();
            while x < frame.width() && frame.cell(x, y).intensity == intensity {
                run.push(frame.cell(x, y).ch);
                x += 1;
            }
            if intensity == 0 {
                continue; // blanks need no element; the next run sets its own x
            }
            let fill = color_for(intensity);
            let escaped = escape_str(&run);
            svg.push_str(&format!(
                "<tspan x=\"{:.1}\" fill=\"{fill}\">{escaped}</tspan>",
                start as f64 * CELL_W
            ));
        }
        svg.push_str("</text>\n");
    }
    svg.push_str("</g>\n</svg>\n");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::Cell;

    fn frame_with(cells: &[(usize, usize, char, u8)], w: usize, h: usize) -> Frame {
        let mut frame = Frame::new(w, h);
        for &(x, y, ch, intensity) in cells {
            frame.set(x, y, Cell { ch, intensity });
        }
        frame
    }

    #[test]
    fn renders_svg_root_and_background() {
        let frame = frame_with(&[(0, 0, 'x', 3)], 3, 2);
        let svg = render_svg(&frame, "example");
        assert!(svg.starts_with("<svg"));
        assert!(svg.trim_end().ends_with("</svg>"));
        assert!(svg.contains("aria-label=\"example\""));
        assert!(svg.contains("fill=\"#0b0b0b\""));
    }

    #[test]
    fn draws_glyphs_with_their_band_colour() {
        let frame = frame_with(&[(0, 0, 'A', 3), (2, 0, 'b', 1)], 4, 1);
        let svg = render_svg(&frame, "t");
        assert!(svg.contains(">A</tspan>"));
        assert!(svg.contains(">b</tspan>"));
        assert!(svg.contains("#f2f2f2")); // bright band for the head
        assert!(svg.contains("#4d4d4d")); // dim band
    }

    #[test]
    fn escapes_xml_special_characters() {
        let frame = frame_with(&[(0, 0, '&', 3), (1, 0, '<', 3), (2, 0, '>', 3)], 3, 1);
        let svg = render_svg(&frame, "a & b");
        assert!(svg.contains("&amp;"));
        assert!(svg.contains("&lt;"));
        assert!(svg.contains("&gt;"));
        // The raw characters must not leak into the document.
        assert!(!svg.contains(">&<"));
    }

    #[test]
    fn empty_cells_emit_no_text() {
        let frame = Frame::new(5, 1);
        let svg = render_svg(&frame, "blank");
        assert!(!svg.contains("<tspan"));
    }
}
