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

    pub fn init(){

    }

    pub fn getFileName() -> &'static str{
        "./DOOM1.wad"
    }

    pub fn render(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100,100,100));
        canvas.clear();
        canvas.present();
    }

    pub fn keyPressed(&self){
        println!("key is pressed in doomengine");
    }

    pub fn keyReleased(&self){

    }

    pub fn quit(mut self){
        self.isOver = true;

    }

    pub fn update(){

    }

    pub fn isOver(&self) -> bool{
        self.isOver
    }

    pub fn getName() -> & 'static str{
        "DIYDoom"
    }

    pub fn getTimePerFrame() -> u32{
        1000/60
    }


}