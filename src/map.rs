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
    canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
}


impl Map {
    pub fn new(name: String, vertexes: Vec<Vertex>,
               lineDefs: Vec<LineDef>, things: Vec<Thing>,
               nodes: Vec<Node>,
               player: Player, canvas: Rc<RefCell<sdl2::render::Canvas<sdl2::video::Window>>>,
    )
               -> Map {
        let iRender = canvas.borrow_mut().logical_size();
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
            iRenderYSize,
            canvas,
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

    pub fn renderAutoMap(&mut self) {
        let iXShift = -self.xMin;
        let iYShift = -self.yMin;

        self.renderAutoMapWalls();

        self.renderAutoMapPlayer();


        self.renderBSPNodesMain();
    }

    pub fn renderAutoMapWalls(&mut self) {
        self.canvas.borrow_mut().set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for line in &self.lineDefs {
            let vStart = self.vertexes[line.startVertex as usize];
            let vEnd = self.vertexes[line.endVertex as usize];
            let x1 = self.remapXToScreen(vStart.xPosition);
            let y1 = self.remapYToScreen(vStart.yPosition);
            let x2 = self.remapXToScreen(vEnd.xPosition);
            let y2 = self.remapYToScreen(vEnd.yPosition);
            let point1 = sdl2::rect::Point::new(x1, y1);

            let point2 = sdl2::rect::Point::new(x2, y2);

            self.canvas.borrow_mut().draw_line(point1, point2);
        }
    }

    pub fn renderAutoMapPlayer(&mut self) {
        self.canvas.borrow_mut().set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));

        let mut direction = vec![(-1, -1), (0, -1), (1, -1),
                                 (-1, 0), (0, 0), (1, 0),
                                 (-1, 1), (0, 1), (1, 1)];

        for i in &direction {
            let x = self.remapXToScreen(self.player.xPosition) + i.0;
            let y = self.remapYToScreen(self.player.yPosition) + i.1;

            let pp = sdl2::rect::Point::new(x, y);
            self.canvas.borrow_mut().draw_point(pp);
        }
    }

    pub fn renderAutoMapNode(&mut self, node: Node) {
//        let node = self.nodes.last().unwrap();
//        println!("getting delay");
        self.canvas.borrow_mut().set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));


        self.canvas.borrow_mut().draw_rect(Rect::new(self.remapXToScreen(node.rightBoxLeft),
                                                     self.remapYToScreen(node.rightBoxTop),
                                                     (self.remapXToScreen(node.rightBoxRight)
                                                         - self.remapXToScreen(node.rightBoxLeft) + 1) as u32,
                                                     (self.remapYToScreen(node.rightBoxBottom)
                                                         - self.remapYToScreen(node.rightBoxTop) + 1) as u32));

        self.canvas.borrow_mut().present();
        let ten_millis = time::Duration::from_millis(300);

        thread::sleep(ten_millis);

        self.canvas.borrow_mut().set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        self.canvas.borrow_mut().draw_rect(Rect::new(self.remapXToScreen(node.leftBoxLeft),
                                                     self.remapYToScreen(node.leftBoxTop),
                                                     (self.remapXToScreen(node.leftBoxRight)
                                                         - self.remapXToScreen(node.leftBoxLeft) + 1) as u32,
                                                     (self.remapYToScreen(node.leftBoxBottom)
                                                         - self.remapYToScreen(node.leftBoxTop) + 1) as u32));
        self.canvas.borrow_mut().present();
        thread::sleep(ten_millis);

        self.canvas.borrow_mut().set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        let x1 = self.remapXToScreen(node.xPartition);
        let y1 = self.remapYToScreen(node.yPartition);
        let x2 = self.remapXToScreen(node.xPartition + node.changeXPartition);
        let y2 = self.remapYToScreen(node.yPartition + node.changeYPartition);




        self.canvas.borrow_mut().draw_line(sdl2::rect::Point::new(x1, y1),
                                           sdl2::rect::Point::new(x2, y2));


        self.canvas.borrow_mut().present();
        thread::sleep(ten_millis);

        self.canvas.borrow_mut().set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));



        self.canvas.borrow_mut().clear();
        self.renderAutoMapWalls();
        self.renderAutoMapPlayer();

    }

    pub fn addThing(&mut self, thing: Thing) {
        let pid = self.player.playerID;
        match thing.typeOfThing {
            pid => {
                self.player.xPosition = thing.xPosition - 3000;
                self.player.yPosition = thing.yPosition + 600;
                self.player.angleOfPlayer = thing.angleOfThing;
            }
        }
        self.things.push(thing);
    }

    pub fn addNodes(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn setLumpIndex(&mut self, iIndex: u32) {
        self.iLumpIndex = Some(iIndex);
    }

    pub fn remapXToScreen(&self, xMapPosition: i16) -> i32 {
        ((xMapPosition + (-self.xMin)) / self.autoMapScaleFactor) as i32
    }

    pub fn remapYToScreen(&self, yMapPosition: i16) -> i32 {
        (self.iRenderYSize - ((yMapPosition + (-self.yMin)) / self.autoMapScaleFactor)) as i32
    }

    pub fn isPointOnLeftSide(&self, xPosition: i16, yPosition: i16, nodeID: usize) -> bool {
        let dx: i16 = xPosition - self.nodes[nodeID].xPartition;
        let dy: i16 = yPosition - self.nodes[nodeID].yPartition;

        let temp1: i32 = (dx as i32 * self.nodes[nodeID].changeYPartition as i32) ;
        let temp2: i32 = (dy as i32 * self.nodes[nodeID].changeXPartition as i32) ;
        let res = temp1 - temp2;
        return  res <= 0
    }

    pub fn renderBSPNodesMain(&mut self) {
        self.renderBSPNodes(self.nodes.len() - 1);
    }

    pub fn renderBSPNodes(&mut self, nodeID: usize) {
        let newNodeID = nodeID as u16;
        let result = newNodeID & SUBSECTORIDENTIFIER;
        let ss = SUBSECTORIDENTIFIER as i32;
        let result2 = result as i32;

//        self.renderAutoMapNode(self.nodes[nodeID].clone());

        println!("The result is :: {} , {}, {}", newNodeID, SUBSECTORIDENTIFIER, result);
        match result2 {

            0 => {
                self.renderAutoMapNode(self.nodes[nodeID].clone());
                let isOnLeft = self.isPointOnLeftSide(self.player.xPosition,
                                                      self.player.yPosition, newNodeID as usize);

                println!("The result2 is :: {} , {}, {}, {}", self.nodes[nodeID].leftChildID,
                         self.nodes[nodeID].rightChildID, nodeID, isOnLeft);

                match isOnLeft {
                    true => {
                        println!("on left side:: {}", self.nodes[nodeID].leftChildID);
                        self.renderBSPNodes(self.nodes[nodeID].leftChildID as usize);
                        self.renderBSPNodes(self.nodes[nodeID].rightChildID as usize);
                    },
                    false => {
                        println!("on right side:: {}", self .nodes[nodeID].rightChildID);
                        self.renderBSPNodes(self.nodes[nodeID].rightChildID as usize);
                        self.renderBSPNodes(self.nodes[nodeID].leftChildID as usize);
                    }
                }
            }
            ss => {
                println!("in subsector");
                self.renderSubsector(newNodeID & (!SUBSECTORIDENTIFIER));
                return;
            }

        }
    }

    pub fn renderSubsector(&self, subSectorID: u16) {}
}