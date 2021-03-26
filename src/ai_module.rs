/* * * * * 
 * Ai module containing all elements and sub elements of ai
 * * * * */

//IMPORTANT NOTE: remember to reset o variable after every calculation

//IMPORTANT NOTE: i can accelerate progress by initially connecting all nodes horizontally,
//                so that the outpu will be the input at first, and from there it will mutate

use rand::*;
const _SUB_NODES: usize = 4;
//-> => ==>
//basic structure:  Nodes ==> Ai
//advance structure: Nodes => SuperNodes -> Ai

#[derive(Clone)]
enum Type {
    Add,
    Mult, //will probably have to be if > then multiply or something
    InvAdd,
    InvMult,
    Ifs, //experimental 0/1 if statement, always Add and then 0-1
}
#[derive(Clone)]
struct Node {
    inp: Vec<f32>, //input multipliers (treshold for Type::Ifs)
    out: f32, //this one is changed every calculation
    out_base: f32, //only read once every calculation, never changed
    action: Type,
}
#[derive(Clone)]
struct SuperNode {
    inp: Vec<f32>, //input multipliers/treshold
    out: f32,
    sub: Vec<Node>
}
#[derive(Clone)]
pub struct Ai {
    inp: Vec<f32>, // will be used as storage, but never read by struct impl
    brain: Vec<Vec<Node>>, //Vec<Layers<Nodes>>
    out: Vec<f32>, //may be unnecesary
}

//will .launch() cluster and it will manage Ais and lanch them on separate threads
pub struct _Cluster {
    
}

impl Node {
    //how many inputs / previous nodes
    fn new(i: &mut u8, n: usize) -> Node {
        let a: Type = {
            *i += 1_u8;
            if *i > 4 { *i = *i % 5 };
            match *i {
                0 => Type::Add,
                1 => Type::Mult,
                2 => Type::InvAdd,
                3 => Type::InvMult,
                4 => Type::Ifs,
                _ => {panic!("Unexpected number at node creation")}
            }

        };

        let x = match a {
            Type::Add => 0.0,
            Type::Mult => 1.0,
            Type::InvAdd => 0.0,
            Type::InvMult => 1.0,
            Type::Ifs => 0.0
        };
        
        Node {
            inp: {
                let mut vec: Vec<f32> = Vec::new();
                
                for _ in 0..n {
                    vec.push(0.0);
                };

                vec
            },
            out: x,
            out_base: x,
            action: a,
        }
    }
    //for recovery, used exclusivly by struct 'Ai' method '.recover()'
    fn _from(t: Type, n: usize) -> Node {

        let x = match t {
            Type::Add => 0.0,
            Type::Mult => 1.0,
            Type::InvAdd => 0.0,
            Type::InvMult => 1.0,
            Type::Ifs => 0.0
        };

        Node {
            inp: {
                let mut vec: Vec<f32> = Vec::new();
                
                for _ in 0..n {
                    vec.push(0.0);
                };

                vec
            },
            out: x,
            out_base: x,
            action: t,
        }

    }
    //this is a total bodge, but its way more efficient to take whole node as inp.
    fn calculate(&mut self, inp: Vec<Node>) -> f32 {
        //out is changed after every calculation so i need to reset it
        let mut o = self.out_base.clone();
        //NOTE: get brain coords here and check for "-1" layer's all nodes
        for i in 0..inp.len() {
            
            let elem = inp[i].out * self.inp[i];
            
            match self.action {
                Type::Add | Type::InvAdd | Type::Ifs => {
                    o += elem;
                },
                Type::Mult | Type::InvMult => {
                    o = o * elem;
                }, 
            }
        }
        //special case
        match self.action {
            Type::Ifs => {
                if o > 0.5 {
                    o = 1.0;
                }
                else {
                    o = 0.0;
                }
            },
            Type::InvAdd | Type::InvMult => {
                o = o * -1.0;
            },
            _ => {} //unnecesary
        } 
        
        //returns and overwrites output, this is important
        //for maybe more efficient mutation amplification
        self.out = o.clone();
        o
    }
}

//WIP, not used, but can be initiated, mainly used so that ai can "choose" the node type
impl SuperNode {
    fn _new() -> SuperNode {
        let mut rng = rand::thread_rng();
        SuperNode {
            inp: {
                //note: for subnodes, inputs are all nodes above/before them
                let vec: Vec<f32> = Vec::new();
                
                //WIP     
                
                vec
            },
            out: 0.0,
            sub: {
                let mut vec: Vec<Node> = Vec::new(); 
                    for i in 0.._SUB_NODES {
                        let mut r: u8 = rng.gen();
                        vec.push(Node::new(&mut r, i+1)); 
                    }
                vec
            }
        }
    }
}
impl Ai {
    pub fn new(n: usize, l: usize) -> Ai {
        Ai {
            inp: Vec::new(),
            brain: {
                let mut vec: Vec<Vec<Node>> = Vec::new();
                let mut inb: Vec<Node> = Vec::new(); 
                //by spliting this into 2 seperate im
                //changing bigO from (n*m) to (n+m) which is great
                let mut f: u8 = 0;
                for _ in 0..n {
                    inb.push(Node::new(&mut f, n));
                }
                for _ in 0..l {
                    vec.push(inb.clone());
                }

                vec
            },
            out: Vec::new()
        }
    }
    //it will be easiest to just add to front/back,
    //as both inp of front and out on back is fed/gathered
    //by seperate function, that doesnt care about brain.len()
    fn _extrude() {
        //WIP 
    }
    pub fn calculate(&mut self, inp: Vec<f32>) -> Vec<f32> {
        //running ai
        // | 'inp' in | 'o' out |
        let mut o: Vec<f32> = Vec::new();

        //set ins
        for node in 0..self.brain[0].len() {
            //flipped situation, usually its X * mul.
            //here its 1 * X as an input. Could cause problems later, too bad!
            self.brain[0][node].inp = inp.clone();
        }

        //activate all nodes
        for layer in 0..self.brain.len() {
            let prev = { //prev layer to pass to nodes
                if layer == 0 { // (layer) = (input layer)
                    //inp to nodes
                    let mut vec: Vec<Node> = Vec::new();
                    for indx_inp in 0..inp.len() {
                        let mut inp_node = Node::new(&mut 0, 0);
                        inp_node.out = inp[indx_inp].clone();
                        vec.push(inp_node);
                    }
                    vec
                } else {
                    //previous node
                    
                    self.brain[layer-1].clone()
                }
            };
            for node in 0..self.brain[layer].len() {
                //previous layer is input
                self.brain[layer][node].calculate(prev.clone());
            }
        }
        
        //gather output
        let end = self.brain.len();
        for node in 0..self.brain[end].len() {
            o.push(self.brain[end][node].out);
        }
        //return output
        o
    }

    //DEBUG
    pub fn list(&self) {
        println!("DEBUG // AI_NODES");
        for layer in 0..self.brain.len() {
            println!("LAYER_{}", layer);
            for node in 0..self.brain[layer].len() {
                let debug_type = match self.brain[layer][node].action {
                    Type::Add => "Type::Add",
                    Type::InvAdd => "Type::InvAdd",
                    Type::Ifs => "Type::Ifs",
                    Type::Mult => "Type::Mult",
                    Type::InvMult => "Type::InvMult",
                };
                println!("\t{}\to:{}",
                    debug_type,   
                    self.brain[layer][node].out
                );
                println!("\ti: {:?}", 
                    self.brain[layer][node].inp
                );
            } 
        }
    }
    /* temporary disabled
    fn recover(&self, s: &str) -> Ai {
        //recover ai's old state
    }
    */
}
