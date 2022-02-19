use ggez:: {graphics, Context };
use ggez::mint::Point2;

pub fn format_scoreboard(scoreboard: &Vec<String>) -> String {
    let mut result = String::new();

    for (index,score) in scoreboard.iter().enumerate()
    {
        let formatted = format!("{}) {}\n", index + 1, score);
        result.push_str(&formatted);
    }

    result
}

pub fn draw_text_background(text_pos: Point2<f32>, text_width: f32, text_height: f32, margin: f32, color: graphics::Color, ctx: &mut Context) {
    let left = text_pos.x - margin;
    let right = text_pos.x + text_width + margin;
    let top = text_pos.y - margin;
    let bottom = text_pos.y + text_height + margin;

    let background = graphics::Rect::new(left, top, right - left, bottom - top);
    let draw_mode = graphics::DrawMode::Fill(graphics::FillOptions::DEFAULT);
    let background_mesh = graphics::MeshBuilder::new().
        rectangle(draw_mode, background, color).
        unwrap().
        build(ctx).
        unwrap();

    graphics::draw(ctx, &background_mesh, graphics::DrawParam::default()).unwrap();
}

pub fn translate(pos: &mut Point2<f32>, trans: &Point2<f32>) {
    pos.x += trans.x;
    pos.y += trans.y;
}