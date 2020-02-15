impl Map{
    pub fn getName(&self) -> String{
        self.name.clone()
    }

    pub fn addVertex(&mut self, vertex: Vertex){
        self.vertexes.push(vertex);
    }

    pub fn addLinedef(&mut self, linedef: LineDef){
        self.lineDefs.push(linedef);
    }
}