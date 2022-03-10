use ducat::core::drawing::{draw_cubic_bezier_curve_mut, draw_line_segment_mut, draw_text_mut};
use ducat::core::entity::rect::Rect;
use image::{ImageBuffer, Rgba};
use rusttype::{point, Font, Scale};
use std::cmp::max;
use std::path::Path;

fn text_size(scale: Scale, font: &Font, text: &str) -> (i32, i32) {
    let v_metrics = font.v_metrics(scale);

    let (mut w, mut h) = (0, 0);

    for g in font.layout(text, scale, point(0.0, v_metrics.ascent)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = max(w, bb.max.x);
            h = max(h, bb.max.y);
        }
    }

    (w, h)
}

pub struct ComCanvas {
    pub path: String,
    pub sz: (i32, i32),
    pub background: Rgba<u8>,
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl ComCanvas {
    pub fn new(path: String, sz: (i32, i32), bg: Option<Rgba<u8>>) -> Self {
        let mut default_bg = Rgba([255u8, 255u8, 255u8, 255u8]);
        match bg {
            Some(b) => default_bg = b,
            None => {}
        }

        let image = ImageBuffer::from_pixel(1600, 1600, default_bg);

        ComCanvas {
            path: path.clone(),
            sz: sz,
            background: default_bg,
            image: image,
        }
    }

    pub fn draw_save(&mut self) {
        let arg = self.path.clone();
        let path = Path::new(&arg);
        self.image.save(path).unwrap();
    }

    pub fn draw_rect(&mut self, off: (i32, i32), width: usize, text_data: &str) -> Rect {
        let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();

        let text_height = 100;

        let height = text_height as f32 * 0.25;
        let scale = Scale {
            x: height * 1.0,
            y: height,
        };

        let text = text_data;

        let (w, h) = text_size(scale, &font, text);
        let rect_w = w + 20;
        let rect_h = 40;

        let rect = Rect::at(off.0, off.1).of_size(rect_w as u32, rect_h as u32);
        self.draw_raw_rect(rect, width, text_data, (w, h), &scale, &font);

        return rect;
    }

    pub fn draw_raw_rect<'a>(
        &mut self,
        rect: Rect,
        width: usize,
        text_data: &str,
        text_sz: (i32, i32),
        scale: &Scale,
        font: &'a Font<'a>,
    ) {
        let default_color = Rgba([97u8, 142u8, 207u8, 255u8]);

        let left = rect.left() as f32;
        let right = rect.right() as f32;
        let top = rect.top() as f32;
        let bottom = rect.bottom() as f32;

        let delta = ((width as f32) * 0.5) as usize;
        let point_a = (left - delta as f32, top - delta as f32);
        let point_b = (right + delta as f32, top - delta as f32);
        let point_c = (right + delta as f32, bottom + delta as f32);
        let point_d = (left - delta as f32, bottom + delta as f32);

        let center = ((point_a.0 + point_b.0) * 0.5, (point_b.1 + point_c.1) * 0.5);
        let (w, h) = (text_sz.0, text_sz.1);

        let text_point_x = center.0 as i32 - (w as f32 * 0.5) as i32;
        let text_point_y = center.1 as i32 - (h as f32 * 0.5) as i32;
        draw_text_mut(
            &mut self.image,
            Rgba([0u8, 0u8, 0u8, 255u8]),
            text_point_x,
            text_point_y,
            *scale,
            &font,
            text_data,
        );

        let ratio = delta * 2;

        //draw ab
        let ab_start = (point_a.0, point_a.1);
        let ab_end = (point_b.0, point_b.1);
        for i in 0..ratio {
            let cur_start = (ab_start.0, ab_start.1 + i as f32);
            let cur_end = (ab_end.0, ab_end.1 + i as f32);
            draw_line_segment_mut(&mut self.image, cur_start, cur_end, default_color);
        }

        //draw bc
        let bc_start = (
            point_b.0 - (ratio - 1) as f32,
            point_b.1 + (ratio - 1) as f32,
        );
        let bc_end = (
            point_c.0 - (ratio - 1) as f32,
            point_c.1 - (ratio - 1) as f32,
        );
        for i in 0..ratio {
            let cur_start = (bc_start.0 + i as f32, bc_start.1);
            let cur_end = (bc_end.0 + i as f32, bc_end.1);
            draw_line_segment_mut(&mut self.image, cur_start, cur_end, default_color);
        }

        //draw dc
        let dc_start = (point_d.0, point_d.1 - (ratio - 1) as f32);
        let dc_end = (point_c.0, point_c.1 - (ratio - 1) as f32);
        for i in 0..ratio {
            let cur_start = (dc_start.0, dc_start.1 + i as f32);
            let cur_end = (dc_end.0, dc_end.1 + i as f32);
            draw_line_segment_mut(&mut self.image, cur_start, cur_end, default_color);
        }

        //draw ad
        let ad_start = (point_a.0, point_a.1 + (ratio - 1) as f32);
        let ad_end = (point_d.0, point_d.1 - (ratio - 1) as f32);
        for i in 0..ratio {
            let cur_start = (ad_start.0 + i as f32, ad_start.1);
            let cur_end = (ad_end.0 + i as f32, ad_end.1);
            draw_line_segment_mut(&mut self.image, cur_start, cur_end, default_color);
        }
    }

    pub fn draw_rect_diag<'a>(&mut self, up: bool, off: (i32, i32), width: usize, height: usize) {
        let rect = Rect::at(off.0, off.1).of_size(width as u32, height as u32);
        let default_color = Rgba([97u8, 142u8, 207u8, 255u8]);

        let left = rect.left() as f32;
        let right = rect.right() as f32;
        let top = rect.top() as f32;
        let bottom = rect.bottom() as f32;

        let start;
        let end;
        let control_a;
        let control_b;
        if up {
            start = (left, bottom);
            end = (right, top);
            control_a = ((left + right) * 0.5, bottom);
            control_b = ((left + right) * 0.5, top);
        } else {
            start = (left, top);
            end = (right, bottom);
            control_a = ((left + right) * 0.5, top);
            control_b = ((left + right) * 0.5, bottom);
        }

        draw_cubic_bezier_curve_mut(
            &mut self.image,
            start,
            end,
            control_a,
            control_b,
            default_color,
        );
    }

    pub fn draw_con_diag<'a>(&mut self, start: (i32, i32), end: (i32, i32)) {
        if start.0 > end.0 {
            return;
        }

        let default_color = Rgba([97u8, 142u8, 207u8, 255u8]);
        if start.1 == end.1 || start.0 == end.0 {
            draw_line_segment_mut(
                &mut self.image,
                (start.0 as f32, start.1 as f32),
                (end.0 as f32, end.1 as f32),
                default_color,
            );
            return;
        }

        let mut off = (start.0, start.1);
        let width = end.0 - start.0;
        let height;
        let mut up = false;
        if start.1 > end.1 {
            up = true;
            off = (start.0, end.1);
            height = start.1 - end.1;
        } else {
            height = end.1 - start.1;
        }

        let rect = Rect::at(off.0, off.1).of_size(width as u32, height as u32);

        let default_color = Rgba([97u8, 142u8, 207u8, 255u8]);

        let left = rect.left() as f32;
        let right = rect.right() as f32;
        let top = rect.top() as f32;
        let bottom = rect.bottom() as f32;

        let new_start;
        let new_end;
        let control_a;
        let control_b;
        if up {
            new_start = (left, bottom);
            new_end = (right, top);
            control_a = ((left + right) * 0.5, bottom);
            control_b = ((left + right) * 0.5, top);
        } else {
            new_start = (left, top);
            new_end = (right, bottom);
            control_a = ((left + right) * 0.5, top);
            control_b = ((left + right) * 0.5, bottom);
        }

        draw_cubic_bezier_curve_mut(
            &mut self.image,
            new_start,
            new_end,
            control_a,
            control_b,
            default_color,
        );
    }
}
