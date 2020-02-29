pub struct DoomEngine{
    renderWidth: u32,
    renderHeight: u32,
    isOver: bool,
}

impl DoomEngine{

    pub fn new() -> DoomEngine{

        DoomEngine{
            isOver: false,
            renderWidth: 320,
            renderHeight: 200,
        }
    }

    pub fn init(&self){
        let w = Wad::loadPath(self.getFileName());
    }

    pub fn getFileName(&self) -> &'static str{
        "./DOOM1.wad"
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100,100,100));
        canvas.clear();

    }

    pub fn keyPressed(&self){
        println!("key is pressed in doomengine");
    }

    pub fn keyReleased(&self){

    }

    pub fn quit(mut self){
        self.isOver = true;

    }

    pub fn update(&self){

    }

    pub fn isOver(&self) -> bool{
        self.isOver
    }

    pub fn getName(&self) -> & 'static str{
        "DIYDoom"
    }

    pub fn getTimePerFrame(&self) -> u32{
        1000/60
    }


}