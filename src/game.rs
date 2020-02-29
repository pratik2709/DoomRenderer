pub struct Game {
    renderWidth: u32,
    renderHeight: u32,
    sdl: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    //    window: sdl2::video::Window,
    doomEngine: DoomEngine,
}

impl Game {
    pub fn new() -> Game {
        let doomEngine = DoomEngine::new();
        let renderWidth = 1280;
        let renderHeight = 800;
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window: sdl2::video::Window = video_subsystem.window(doomEngine.getName(),
                                                                 renderWidth, renderHeight).resizable().build()
            .unwrap();

        //Canvas:
        // Manages and owns a target (Surface or Window) and allows drawing in it.
        let canvas = window.into_canvas().present_vsync().build().unwrap();
        Game {
            renderWidth,
            renderHeight,
            sdl,
            canvas,
//            window,
            doomEngine,
        }
    }

    pub fn init(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        self.doomEngine.init();
        self.canvas.set_logical_size(self.doomEngine.renderWidth, self.doomEngine.renderHeight);
    }

    pub fn processInput(&self) {
        //see named loops in rust
        let mut eventPump = self.sdl.event_pump().unwrap();
        'main: loop {
            for event in eventPump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                        self.doomEngine.keyPressed();
                        break 'main
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                        self.doomEngine.keyPressed();
                        break 'main
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn render(&mut self) {
        self.doomEngine.render(&mut self.canvas);
    }

    pub fn update(&self) {
        self.doomEngine.update();
    }

    pub fn isOver(&self) -> bool {
        self.doomEngine.isOver()
    }

    pub fn delay(&self){
        self.doomEngine.getTimePerFrame();
    }
}