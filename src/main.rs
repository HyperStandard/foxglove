use bracket_terminal::prelude::*;
use log::{info, warn};

struct FoxPosition {
    x: i64,
    y: i64,
    z: i64,
}

struct FoxDia {
    active: bool,
    text: String,
    has_input: bool,
}

struct State {
    life: i32,
    position: FoxPosition,
    name: String,
    dias: Vec<FoxDia>,
    conw: i32,
    conh: i32,
    input: String,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();
        let mut box_width: i32 = 0;
        let mut box_height: i32 = 0;
        let mut input = INPUT.lock();
        let mouse_pixels = input.mouse_pixel_pos();
        let keyset = input.key_pressed_set();



        ctx.cls();
        draw_batch.target(0);
        draw_batch.cls();

        for key in keyset {
            if key
         draw_batch.print(Point::new(1, 3), text)   
        }

        draw_batch.print(
            Point::new(1, 1),
            &format!(
                "Hello. We are at {a}x{b}, and have {c} life. How long can this sentence be?",
                a = self.position.x,
                b = self.position.y,
                c = self.life
            ),
        );
        draw_batch.target(1);
        draw_batch.cls();
        box_height = 9i32;
        box_width = self.conw - 1;
        draw_batch.draw_double_box(
            Rect::with_size(0, self.conh - box_height - 1, box_width, box_height),
            ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(1, self.conh - box_height),
            &format!("FPS: {}", ctx.fps),
            ColorPair::new(RGB::named(RED), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(1, self.conh - box_height + 1),
            &format!("Frame Time: {} ms", ctx.frame_time_ms),
            ColorPair::new(RGB::named(ORANGE), RGB::named(BLACK)),
        );
        box_height = 5i32;
        box_width = 65;
        draw_batch.draw_double_box(
            Rect::with_size(
                (self.conw - box_width) / 2,
                (self.conh - box_height) / 2,
                box_width,
                box_height,
            ),
            ColorPair::new(RGB::named(GREEN), RGB::named(BLACK)),
        );

        draw_batch.print_color(
            Point::new(
                (self.conw - box_width) / 2 + 1,
                (self.conh - box_height) / 2 + 1,
            ),
            &format!("What is your name?"),
            ColorPair::new(RGB::named(BLUE), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(
                (self.conw - box_width) / 2 + 1,
                (self.conh - box_height) / 2 + 4,
            ),
            &format!("> {b}", b = self.input),
            ColorPair::new(RGB::named(BLUE), RGB::named(GREY)),
        );

        draw_batch.submit(0).expect("Batch error");

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let foxw: i32 = 120;
    let foxh: i32 = 80;

    let context = BTermBuilder::simple(foxw, foxh)?
        .with_title("Foxglone Alpha")
        .with_sparse_console(foxw, foxh, "terminal8x8.png")
        .build()?;

    let gs: State = State {
        input: String::new(),
        conw: foxw,
        conh: foxh,
        life: 60,
        position: FoxPosition { x: 1, y: 6, z: 0 },
        name: String::new(),
        dias: vec![FoxDia {
            active: true,
            has_input: true,
            text: String::from("What is your name?"),
        }],
    };
    main_loop(context, gs)
}
