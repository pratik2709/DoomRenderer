pub struct Game {
    renderWidth: u32,
    renderHeight: u32,
    doomEngine: DoomEngine,
}

impl Game {
    pub fn new() -> Game {
        Game {
            renderWidth: 1280,
            renderHeight: 800,
            doomEngine: DoomEngine::new(),
        }
    }

    pub fn init(){
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem.window("Game", 320, 240).resizable().build().unwrap();

        //Canvas:
        // Manages and owns a target (Surface or Window) and allows drawing in it.
        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    }
}