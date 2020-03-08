#[derive(Debug)]
pub struct Map{
    name: String,
    xMin: i16,
    xMax: i16,
    yMin: i16,
    yMax: i16,
    autoMapScaleFactor: i16,
    vertexes: Vec<Vertex>,
    lineDefs: Vec<LineDef>,
    things: Vec<Thing>,
    player: Player,

}

impl Map{

    pub fn new(name: String, vertexes: Vec<Vertex>,
               lineDefs: Vec<LineDef>, things: Vec<Thing>, player: Player
    )
        -> Map{

        Map{
            name,
            xMax: std::i16::MIN,
            xMin: std::i16::MAX,
            yMax: std::i16::MIN,
            yMin: std::i16::MAX,
            autoMapScaleFactor: 15,
            vertexes,
            lineDefs,
            things,
            player
        }
    }

    pub fn getName(&self) -> String{
        self.name.clone()
    }

    pub fn addVertex(&mut self, vertex: Vertex){
        self.vertexes.push(vertex.clone());

        if self.xMin > vertex.xPosition{
            self.xMin = vertex.xPosition;
        }

        else if self.xMax < vertex.xPosition {
            self.xMax = vertex.xPosition;
        }

        if self.yMin > vertex.yPosition{
            self.yMin = vertex.yPosition;
        }

        else if self.yMax < vertex.yPosition {
            self.yMax = vertex.yPosition;
        }

    }

    pub fn addLinedef(&mut self, linedef: LineDef){
        self.lineDefs.push(linedef);
    }

    pub fn renderAutoMap(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        let iXShift = -self.xMin;
        let iYShift = -self.yMin;

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for line in &self.lineDefs{
            let vStart = self.vertexes[line.startVertex as usize];
            let vEnd = self.vertexes[line.endVertex as usize];
            let point1 = sdl2::rect::Point::new(((vStart.xPosition + iXShift) / self
                .autoMapScaleFactor) as i32,
                                                ((vStart.yPosition + iYShift) / self
                                                    .autoMapScaleFactor) as i32);

            let point2 = sdl2::rect::Point::new(((vEnd.xPosition + iXShift) / self
                .autoMapScaleFactor) as i32,
                                                ((vEnd.yPosition + iYShift) / self.autoMapScaleFactor) as i32);

            canvas.draw_line(point1, point2);
        }
    }

    pub fn addThing(&mut self, thing: Thing){
        self.things.push(thing);
    }
}