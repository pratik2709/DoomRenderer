pub struct Game {
    renderWidth: u32,
    renderHeight: u32,
    sdl: sdl2::Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    doomEngine: DoomEngine,
}

impl Game {
    pub fn new() -> Game {
        let doomEngine = DoomEngine::new();
        let renderWidth = 1280;
        let renderHeight = 800;
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem.window(doomEngine.getName(),
                                            renderWidth,
                                            renderHeight)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string()).unwrap();

        //Canvas:
        // Manages and owns a target (Surface or Window) and allows drawing in it.
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string()).unwrap();
        canvas.set_logical_size(doomEngine.renderWidth, doomEngine.renderHeight);
        Game {
            renderWidth,
            renderHeight,
            sdl,
            canvas,
            doomEngine,
        }
    }


    pub fn processInput(&mut self) {
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
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                        self.doomEngine.keyPressed();
                        self.doomEngine.quit();
                    }
                    _ => {}
                }
            }
            self.render();
            self.update();
            self.delay();
            if self.isOver() {
                break 'main
            }
        }

    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 255));
        self.canvas.clear();
        self.doomEngine.render(&mut self.canvas);
        self.canvas.present();
    }

    pub fn init(&mut self){

    }

    pub fn update(&mut self) {
        self.doomEngine.update();
    }

    pub fn isOver(&self) -> bool {
        self.doomEngine.isOver()
    }

    pub fn delay(&self) {
        self.doomEngine.getTimePerFrame();
    }
}