#[derive(Debug)]
pub struct Map{
    name: String,
    xMin: u32,
    xMax: u32,
    yMin: u32,
    yMax: u32,
    autoMapScalefactor: u32,
    vertexes: Vec<Vertex>,
    lineDefs: Vec<LineDef>
}

impl Map{

    pub fn new(name: String, vertexes: Vec<Vertex>, lineDefs: Vec<LineDef>)
        -> Map{

        Map{
            name,
            xMax: std::u32::MAX,
            xMin: std::u32::MIN,
            yMax: std::u32::MAX,
            yMin: std::u32::MIN,
            autoMapScalefactor: 15,
            vertexes,
            lineDefs
        }
    }

    pub fn getName(&self) -> String{
        self.name.clone()
    }

    pub fn addVertex(&mut self, vertex: Vertex){
        self.vertexes.push(vertex);

    }

    pub fn addLinedef(&mut self, linedef: LineDef){
        self.lineDefs.push(linedef);
    }

    pub fn renderAutoMap(){

    }
}