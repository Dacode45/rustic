use ggez::graphics::Rect;

pub fn generate_uvs(i_width: f32, i_height: f32, t_width: f32, t_height: f32) -> Vec<Rect> {
    let width = t_width / i_width;
    let height = t_height / i_height;
    let cols = i_width / t_width;
    let rows = i_height / t_height;

    let mut ux: f32 = 0.0;
    let mut uy: f32 = 0.0;

    let mut uvs = Vec::new();

    for _ in 0..(rows as u32) {
        for _ in 0..(cols as u32) {
            uvs.push(Rect::new(ux, uy, width, height));
            ux += width;
        }
        ux = 0.0;
        uy += height;
    }
    return uvs;
}
