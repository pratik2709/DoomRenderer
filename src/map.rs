#[derive(Debug)]
pub struct Map {
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

impl Map {
    pub fn new(name: String, vertexes: Vec<Vertex>,
               lineDefs: Vec<LineDef>, things: Vec<Thing>, player: Player,
    )
               -> Map {
        Map {
            name,
            xMax: std::i16::MIN,
            xMin: std::i16::MAX,
            yMax: std::i16::MIN,
            yMin: std::i16::MAX,
            autoMapScaleFactor: 15,
            vertexes,
            lineDefs,
            things,
            player,
        }
    }

    pub fn getName(&self) -> String {
        self.name.clone()
    }

    pub fn addVertex(&mut self, vertex: Vertex) {
        self.vertexes.push(vertex.clone());

        if self.xMin > vertex.xPosition {
            self.xMin = vertex.xPosition;
        } else if self.xMax < vertex.xPosition {
            self.xMax = vertex.xPosition;
        }

        if self.yMin > vertex.yPosition {
            self.yMin = vertex.yPosition;
        } else if self.yMax < vertex.yPosition {
            self.yMax = vertex.yPosition;
        }
    }

    pub fn addLinedef(&mut self, linedef: LineDef) {
        self.lineDefs.push(linedef);
    }

    pub fn renderAutoMap(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let iXShift = -self.xMin;
        let iYShift = -self.yMin;


        let iRender = canvas.logical_size();
        let mut iRenderXSize = iRender.0 as i16;
        let mut iRenderYSize = iRender.1 as i16;

        self.renderAutoMapWalls(canvas, iXShift, iYShift,
                                iRenderXSize,
                                iRenderYSize);

        self.renderAutoMapPlayer(canvas, iXShift, iYShift,
                                iRenderXSize,
                                iRenderYSize);


    }

    pub fn renderAutoMapWalls(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
                              iXShift: i16, iYShift: i16, mut iRenderXSize: i16, mut iRenderYSize:
                              i16) {
        iRenderXSize -= 1;
        iRenderYSize -= 1;

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for line in &self.lineDefs {
            let vStart = self.vertexes[line.startVertex as usize];
            let vEnd = self.vertexes[line.endVertex as usize];
            let point1 = sdl2::rect::Point::new(((vStart.xPosition + iXShift) / self
                .autoMapScaleFactor) as i32,
                                                (iRenderYSize - (vStart.yPosition + iYShift) / self
                                                    .autoMapScaleFactor) as i32);

            let point2 = sdl2::rect::Point::new(((vEnd.xPosition + iXShift) / self
                .autoMapScaleFactor) as i32,
                                                (iRenderYSize - (vEnd.yPosition + iYShift) / self
                                                    .autoMapScaleFactor) as i32);

            canvas.draw_line(point1, point2);
        }
    }

    pub fn renderAutoMapPlayer(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
                               iXShift: i16, iYShift: i16, mut iRenderXSize: i16, mut iRenderYSize:
                              i16) {

        iRenderXSize -= 1;
        iRenderYSize -= 1;

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));

//        canvas.fill_rect(Rect::new(10, 10, 100, 100));
        let mut direction = vec![(-1, -1), (0, -1), (1, -1),
                                 (-1, 0), (0, 0), (1, 0),
                                 (-1, 1), (0, 1), (1, 1)];

        for i in &direction{
            let x = ((self.player.xPosition + iXShift)/ self
                .autoMapScaleFactor + i.0) as i32;

            let y = (iRenderYSize - ((self.player.yPosition + iYShift)/self.autoMapScaleFactor + i
                .1)) as i32 ;

            let p = sdl2::rect::Point::new(x,y);
            canvas.draw_point(p);
        }
    }

    pub fn addThing(&mut self, thing: Thing) {
        self.things.push(thing);
    }
}