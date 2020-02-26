pub struct DoomEngine{
    renderWidth: u32,
    renderHeight: u32,
    isOver: bool,
//    episodeMap: Map
}

impl DoomEngine{

    pub fn new() -> DoomEngine{
//        let map = Map{
//            name:"e1m1", .. };

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

    pub fn render(mut canvas: sdl2::render::Canvas<sdl2::video::Window>){
        canvas.set_draw_color(sdl2::pixels::Color::RGB(100,100,100));
        canvas.clear();
        canvas.present();
    }

    pub fn keyPressed(e: &mut sdl2::EventPump){
        //see named loops in rust
        'main: loop{
            for event in e.poll_iter(){
                match event{
                    Event::Quit {..} => break 'main,
                    Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                        println!("key is pressed up");
                    },
                    Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                        println!("key is pressed down");
                    },
                    _ => {}
                }
            }
        }

    }

    pub fn keyReleased(){

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