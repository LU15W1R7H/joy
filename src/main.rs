use ggez::event::{self, *};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    value: u32,
}

impl Cell {
    fn get_color(&self) -> graphics::Color {
        graphics::Color::from_rgb_u32(self.value)
    }
}

pub struct CellRenderer {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl CellRenderer {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.height + x) as usize
    }
}

impl CellRenderer {
    fn new(_ctx: &mut Context, width: u32, height: u32) -> CellRenderer {
        let cells = (0..width * height)
            .map(|_i| {
                let val = rand::thread_rng().gen_range(0, std::u32::MAX);
                Cell { value: val }
            })
            .collect();

        CellRenderer {
            width,
            height,
            cells,
        }
    }

    fn tick(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // for x in 0..self.width {
        //     for y in 0..self.height {
        //         let idx = self.get_index(x, y);
        //         let cell = &mut self.cells[idx];

        //         cell.value = rand::thread_rng().gen_range(0,std::u32::MAX);
        //     }
        // }

        let idx = rand::thread_rng().gen_range(0, self.width * self.height) as usize;
        let cell = &mut self.cells[idx];
        cell.value = rand::thread_rng().gen_range(0, std::u32::MAX);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let screen_coords = graphics::screen_coordinates(ctx);
        let cell_width = screen_coords.w / self.width as f32;
        let cell_height = screen_coords.h / self.height as f32;

        let mb = &mut graphics::MeshBuilder::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];

                let rect = graphics::Rect {
                    x: x as f32 * cell_width,
                    y: y as f32 * cell_height,
                    w: cell_width,
                    h: cell_height,
                };

                mb.rectangle(graphics::DrawMode::fill(), rect, cell.get_color());
            }
        }
        let mesh = mb.build(ctx)?;

        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }
}

pub struct AppConfig {
    width: u32,
    height: u32,
}

pub struct App {
    cell_renderer: CellRenderer,
}

impl App {
    pub fn new(_ctx: &mut Context, config: &AppConfig) -> App {
        let cell_renderer = CellRenderer::new(_ctx, config.width, config.height);
        App { cell_renderer }
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.cell_renderer.tick(_ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.cell_renderer.draw(ctx)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Q => {
                ggez::event::quit(ctx);
            }

            _ => (),
        }
    }
}

fn main() {
    let window_setup = ggez::conf::WindowSetup::default().title("Joy");
    let window_mode = ggez::conf::WindowMode::default().resizable(true);

    let (mut ctx, mut event_loop) = ContextBuilder::new("joy_app", "lwirth")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("ggez could not be started");

    let app_config = AppConfig {
        width: 64,
        height: 64,
    };

    let mut app = App::new(&mut ctx, &app_config);

    // start gameloop; returns error during execution
    match event::run(&mut ctx, &mut event_loop, &mut app) {
        Ok(_) => println!("clean exit"),
        Err(e) => println!("error occured during execution: {}", e),
    }
}
