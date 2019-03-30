use ggez::*;
extern crate rand;
use rand::{thread_rng, Rng};

const BOARDSIZE: (i32, i32) = (20, 35);
const TILESIZE: i32 = 24;
const NUMBEROFTILES: i32 = BOARDSIZE.1 * BOARDSIZE.0;


struct State {
    gboard: gameboard,
}

struct tile {
    blocks : Vec<(i32,i32)>,
    color: i32,
}

struct gameboard {
    board: Vec<i32>,
    tiles: Vec<tile>,
    activetile: brick,//Option<brick>,
}

struct block {
    x:i32,
    y:i32,
    color:i32,
}

struct brick {
    active : bool,
    tileid : i32,
    x:i32,
    y:i32,
}

impl brick {
    fn movedown(&mut self) {
        self.y += 1;
    }
}

impl gameboard {
    fn create() -> Self {
        let mut board = Vec::new();

        for x in 0..NUMBEROFTILES {
            if x > NUMBEROFTILES - 91 {
                board.push(thread_rng().gen_range(0.0, 8.0) as i32);
            } else {
                board.push(0);
            }
        }

        //lage tiles
        let mut tiles = Vec::new();

        let blocks = vec![(0,0),(-1,0),(0,-1),(0,1)];
        let tilo = tile{blocks:blocks, color:1};
        tiles.push(tilo);

        let blocks = vec![(0,0),(1,0),(1,1),(0,1)];
        let tilo = tile{blocks:blocks, color:2};
        tiles.push(tilo);

        let blocks = vec![(0,0),(1,1),(0,1),(0,-1)];
        let tilo = tile{blocks:blocks, color:3};
        tiles.push(tilo);

        let blocks = vec![(0,0),(-1,1),(0,1),(0,-1)];
        let tilo = tile{blocks:blocks, color:4};
        tiles.push(tilo);

        let blocks = vec![(0,0),(0,1),(0,-1),(0,2)];
        let tilo = tile{blocks:blocks, color:5};
        tiles.push(tilo);

        let blocks = vec![(0,0),(0,1),(1,1),(-1,0)];
        let tilo = tile{blocks:blocks, color:6};
        tiles.push(tilo);

        let blocks = vec![(0,0),(0,1),(-1,1),(1,0)];
        let tilo = tile{blocks:blocks, color:7};
        tiles.push(tilo);

        gameboard {board: board, tiles: tiles, activetile: brick{active: false, tileid: 0, x: 0, y: 0}}
    }

    fn get1dcoords(xy:(i32, i32)) -> i32 {
        xy.0 + (xy.1 * BOARDSIZE.0)
    }

    fn getcolor(id:i32) -> graphics::Color {
        match id {
            0 => graphics::BLACK,
            1 => makecolor(1.0, 1.0, 1.0),
            2 => makecolor(0.8, 0.0, 0.0),
            3 => makecolor(0.8, 0.0, 0.8),
            4 => makecolor(1.0, 0.0, 0.4),
            5 => makecolor(0.2, 0.5, 0.3),
            6 => makecolor(0.0, 0.7, 0.0),
            7 => makecolor(0.0, 0.7, 0.7),
            8 => makecolor(0.5, 0.2, 0.4),
            _ => makecolor(0.1, 0.7, 0.3),
        }
        
    }

    fn gettileid(&self, xy:(i32, i32)) -> i32 {
        self.board[gameboard::get1dcoords(xy) as usize]
    }

    /*fn settileid(&mut self, xy:(i32, i32), tileid:i32) {
        self.board[gameboard::get1dcoords(xy) as usize] = tileid;
    }*/

    fn updateboard(&mut self) {
        if self.activetile.active {
            self.activetile.y += 1;
            //sjekk om kræsj

            let mut krasj = false;

            for bb in &self.tiles[self.activetile.tileid as usize].blocks {
                if self.gettileid((self.activetile.x+bb.0, self.activetile.y+bb.1)) > 0 {
                    //kræsj
                    krasj = true;
                }
            }

            if krasj == true {
                self.activetile.y -= 1;
                for bb in &self.tiles[self.activetile.tileid as usize].blocks {
                    self.board[gameboard::get1dcoords((self.activetile.x+bb.0, self.activetile.y+bb.1)) as usize] = self.tiles[self.activetile.tileid as usize].color;
                }
                self.activetile.active = false;
            }

        } else {
            self.activetile.active = true;
            self.activetile.x = thread_rng().gen_range(2.0, (BOARDSIZE.0 as f32)-2.0) as i32;//BOARDSIZE.0 / 2;
            self.activetile.y = 1;
            self.activetile.tileid = thread_rng().gen_range(0.0, 7.0) as i32;
        }
    }

    fn renderboard(&self) -> Vec<block> {
        let mut ex = 0;
        let mut ey = 0;

        //bruk meshbuilder

        let mut blockstorender = Vec::new();

        for x in 0..NUMBEROFTILES {
            if self.board[x as usize] > 0 {
                blockstorender.push(block{x:ex, y:ey, color:self.board[x as usize]});
            }
            
            ex += 1;
            if ex == BOARDSIZE.0 {
                ex = 0;
                ey += 1;
            }
        }

        //hvis activetile
        if self.activetile.active {
            let color = self.tiles[self.activetile.tileid as usize].color;
            for bb in &self.tiles[self.activetile.tileid as usize].blocks {
                blockstorender.push(block{x:self.activetile.x+bb.0, y:self.activetile.y+bb.1, color:color});
            }
        }


        blockstorender
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.gboard.updateboard();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        //ramme
        let pep = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), 
                ggez::graphics::Rect::new_i32(0, 0, BOARDSIZE.0 * TILESIZE +1, BOARDSIZE.1 * TILESIZE+1), 
                gameboard::getcolor(1)).unwrap();
        
        graphics::draw(ctx, &pep, graphics::DrawParam::default()).unwrap();
        //

        let torender = self.gboard.renderboard();

        for blok in &torender {
            let pep = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), 
                    ggez::graphics::Rect::new_i32(blok.x*TILESIZE+1, blok.y*TILESIZE+1, TILESIZE-2, TILESIZE-2), 
                    gameboard::getcolor(blok.color)).unwrap();

            graphics::draw(ctx, &pep, graphics::DrawParam::default()).unwrap();
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn makecolor(r: f32, g:f32, b:f32) -> graphics::Color {
    graphics::Color::new(r, g, b, 1.0)
}

fn randomcolor() -> graphics::Color {
    makecolor(thread_rng().gen_range(0.0, 1.0), thread_rng().gen_range(0.0, 1.0), thread_rng().gen_range(0.0, 1.0))
}

fn main() {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("balle", "Hylle")
        .window_setup(ggez::conf::WindowSetup::default().title("kaffe"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1400.0, 900.0))
        .build().unwrap();

    let gboard = gameboard::create();

    let state = &mut State {gboard: gboard};

    event::run(ctx, event_loop, state).unwrap();
}