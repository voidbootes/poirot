use image::Rgba;
use poriot::raster::canvas::ComCanvas;

fn main() {
    let mut cc = ComCanvas::new(arg, (1600, 1600), Some(Rgba([255u8, 255u8, 255u8, 255u8])));
    cc.draw_rect((500, 500), 4, "hello, canvas");
}
