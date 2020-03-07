pub struct DoomEngine {
    renderWidth: u32,
    renderHeight: u32,
    isOver: bool,
    map: Map,
}

impl DoomEngine {
    pub fn new() -> DoomEngine {
        let wadFile = Wad::loadFileUsingPath(DoomEngine::getFileName());
        let w = Wad::getWadData(&wadFile);
        let mapName = String::from("E1M1");
        let mapData = w.loadMapData(&wadFile, mapName);

        DoomEngine {
            isOver: false,
            renderWidth: 320,
            renderHeight: 200,
            map: mapData,
        }
    }

    pub fn init(&self) {}

    pub fn getFileName() -> &'static str {
        "./DOOM1.wad"
    }

    pub fn render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
//        self.map.renderAutoMap(canvas);
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

    pub fn getName(&self) -> &'static str {
        "DIYDoom"
    }

    pub fn getTimePerFrame(&self) -> u32 {
        1000 / 60
    }
}