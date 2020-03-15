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
    iLumpIndex: Option<u32>,
    nodes: Vec<Node>,
    iRenderXSize: i16,
    iRenderYSize: i16,
}

impl Map {
    pub fn new(name: String, vertexes: Vec<Vertex>,
               lineDefs: Vec<LineDef>, things: Vec<Thing>,
               nodes: Vec<Node>,
               player: Player,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>
    )
               -> Map {
        let iRender = canvas.logical_size();
        let mut iRenderXSize = iRender.0 as i16;
        let mut iRenderYSize = iRender.1 as i16;
        iRenderXSize -= 1;
        iRenderYSize -= 1;

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
            nodes,
            iLumpIndex: None,
            iRenderXSize,
            iRenderYSize
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

        self.renderAutoMapWalls(canvas);

        self.renderAutoMapPlayer(canvas);


    }

    pub fn renderAutoMapWalls(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {


        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for line in &self.lineDefs {
            let vStart = self.vertexes[line.startVertex as usize];
            let vEnd = self.vertexes[line.endVertex as usize];
            let x1 = self.remapXToScreen(vStart.xPosition);
            let y1 = self.remapYToScreen(vStart.yPosition);
            let x2 = self.remapXToScreen(vEnd.xPosition);
            let y2 = self.remapYToScreen(vEnd.yPosition);
            let point1 = sdl2::rect::Point::new(x1, y1);

            let point2 = sdl2::rect::Point::new(x2, y2);

            canvas.draw_line(point1, point2);
        }
    }

    pub fn renderAutoMapPlayer(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {


        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));

        let mut direction = vec![(-1, -1), (0, -1), (1, -1),
                                 (-1, 0), (0, 0), (1, 0),
                                 (-1, 1), (0, 1), (1, 1)];

        for i in &direction{
            let x = self.remapXToScreen(self.player.xPosition) + i.0;
            let y = self.remapYToScreen(self.player.yPosition) + i.1;

            let pp = sdl2::rect::Point::new(x,y);
            canvas.draw_point(pp);
        }

    }

    pub fn addThing(&mut self, thing: Thing) {

        let pid = self.player.playerID;
        match thing.typeOfThing {
            pid => {
                self.player.xPosition = thing.xPosition;
                self.player.yPosition = thing.yPosition;
                self.player.angleOfPlayer = thing.angleOfThing;
            }
        }
        self.things.push(thing);
    }

    pub fn addNodes(&mut self, node: Node) {

        self.nodes.push(node);
    }

    pub fn setLumpIndex(&mut self, iIndex: u32){
        self.iLumpIndex = Some(iIndex);
    }

    pub fn remapXToScreen(&self, xMapPosition: i16) -> i32{
        ((xMapPosition + (-self.xMin)) / self.autoMapScaleFactor) as i32
    }

    pub fn remapYToScreen(&self, yMapPosition: i16) -> i32{
        (self.iRenderYSize - ((yMapPosition + (-self.yMin)) / self.autoMapScaleFactor)) as i32
    }
}