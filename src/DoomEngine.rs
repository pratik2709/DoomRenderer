pub struct DoomEngine {
    renderWidth: u32,
    renderHeight: u32,
    isOver: bool,
    map: Map,
}

impl DoomEngine {
    pub fn new(mut canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>) -> DoomEngine {
        let wadFile = Wad::loadFileUsingPath(DoomEngine::getFileName());
        let w = Wad::getWadData(&wadFile);
        let mapName = String::from("E1M1");
        let player = Player::new(1);
        let renderWidth = 320;
        let renderHeight = 240;

        canvas.borrow_mut().set_logical_size(renderWidth, renderHeight);

        let mapData = w.loadMapData(&wadFile, mapName, player, canvas);


        DoomEngine {
            isOver: false,
            renderWidth,
            renderHeight,
            map: mapData,
        }
    }

    pub fn init(&self) {}

    pub fn getFileName() -> &'static str {
        "./DOOM1.wad"
    }

    pub fn render(&mut self) {
        self.map.renderAutoMap();
    }

    pub fn keyPressed(&self) {
        println!("key is pressed in doomengine");
    }

    pub fn keyReleased(&self) {}

    pub fn quit(&mut self) {
        println!("quitt");
        self.isOver = true;
    }

    pub fn update(&self) {}

    pub fn isOver(&self) -> bool {
        self.isOver
    }

    pub fn getName() -> &'static str {
        "DIYDoom"
    }

    pub fn getTimePerFrame(&self) -> u32 {
        1000 / 60
    }
}