pub struct Game {
    renderWidth: u32,
    renderHeight: u32,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    doomEngine: DoomEngine,
}

impl Game {
    pub fn new() -> Game {
        let renderWidth = 1280;
        let renderHeight = 800;
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window: sdl2::video::Window = video_subsystem.window("Game",
                                            renderWidth, renderHeight).resizable().build()
            .unwrap();

        //Canvas:
        // Manages and owns a target (Surface or Window) and allows drawing in it.
        let canvas = window.into_canvas().present_vsync().build().unwrap();
        Game {
            renderWidth,
            renderHeight,
            canvas,
            doomEngine: DoomEngine::new(),
        }
    }

    pub fn init(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(100, 100, 100));
        self.canvas.set_logical_size(self.doomEngine.renderWidth, self.doomEngine.renderHeight);
    }

    pub fn render(&mut self) {
        self.doomEngine.render(&mut self.canvas);
    }
}