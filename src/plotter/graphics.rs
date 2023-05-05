use bracket_lib::prelude::*;

add_wasm_support!();

pub struct Window {
    height: i8,
    width: i8,
}

impl Window {
    pub fn new(height: i8, width: i8) -> Self {
        Self { height, width }
    }

    pub fn render(&self) -> BError {
        let context = BTermBuilder::simple(self.width, self.height)?
            .with_fancy_console(self.width, self.height, "terminal8x8.png")
            .with_title("Plotter")
            .with_vsync(true)
            .build()?;

        main_loop(context, Window::new(80, 50))
    }
}

impl GameState for Window {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        //ctx.set_translation_mode(0, CharacterTranslationMode::Unicode);
        //ctx.cls();

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(1);
        draw_batch.cls();

        //Test
        draw_batch.draw_double_box(
            Rect::with_size(1, 1, 20, 4),
            ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(2, 2),
            &format!("* f(x) = "),
            ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(2, 4),
            &format!("* Range = "),
            ColorPair::new(RGB::named(ORANGE), RGB::named(BLACK)),
        );

        // Submission
        draw_batch.submit(0).expect("Batch error");
        ctx.cls_bg(NAVY);
        render_draw_buffer(ctx).expect("Render error");
    }
}
