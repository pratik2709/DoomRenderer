#[derive(Debug)]
pub struct Map{
    name: String,
    xMin: i32,
    xMax: i32,
    yMin: i32,
    yMax: i32,
    autoMapScaleFactor: i32,
    vertexes: Vec<Vertex>,
    lineDefs: Vec<LineDef>
}

impl Map{

    pub fn new(name: String, vertexes: Vec<Vertex>, lineDefs: Vec<LineDef>)
        -> Map{

        Map{
            name,
            xMax: std::i32::MAX,
            xMin: std::i32::MIN,
            yMax: std::i32::MAX,
            yMin: std::i32::MIN,
            autoMapScaleFactor: 15,
            vertexes,
            lineDefs
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

//        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for line in &self.lineDefs{
            let vStart = self.vertexes[line.startVertex as usize];
            let vEnd = self.vertexes[line.endVertex as usize];
            let point1 = sdl2::rect::Point::new((vStart.xPosition + iXShift) / self
                .autoMapScaleFactor,
            (vStart.yPosition + iYShift) / self.autoMapScaleFactor);

            let point2 = sdl2::rect::Point::new((vEnd.xPosition + iXShift) / self
                .autoMapScaleFactor,
            (vEnd.yPosition + iYShift) / self.autoMapScaleFactor);

            canvas.draw_line(point1, point2);
        }
    }
}