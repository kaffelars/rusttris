use ggez::*;
use ggez::event::{KeyCode, KeyMods};
extern crate rand;
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

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
    lastupdate : Instant,
}

struct block {
    x:i32,
    y:i32,
    color:i32,
}

struct brick {
    active : bool,
    tileid : i32,
    flipped : i32,
    blocks : Vec<(i32,i32)>,//lazy times
    bounds : (i32, i32),
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
                board.push(thread_rng().gen_range(0.0, 10.99) as i32);
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

        let blocks = vec![(0,0),(0,1),(0,-1),(1,0),(-1,0),(0,2),(0,-2),(2,0),(-2,0)];
        let tilo = tile{blocks:blocks, color:8};
        tiles.push(tilo);
        gameboard {board: board, tiles: tiles, activetile: brick{active: false, tileid: 0, flipped: 0, blocks: Vec::new(), bounds:(0,0), x: 0, y: 0}, lastupdate : Instant::now()}
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

    fn movebrick(&mut self, x:i32) {
        self.activetile.x += x;

        if self.activetile.x < self.activetile.bounds.0 {
            self.activetile.x = self.activetile.bounds.0;
        }

        if self.activetile.x > BOARDSIZE.0-self.activetile.bounds.1-1 {
            self.activetile.x = BOARDSIZE.0-self.activetile.bounds.1-1;
        }
    }

    fn flipbrick(&mut self) {
        let pepsimaxi = self.activetile.flipped;
        self.activetile.flipped += 1; //sjekk om det er lov å flippe
        if self.activetile.flipped > 3 {self.activetile.flipped = 0}

        self.fillactivetile();

        if self.iscrash() { //ikke lov å flippe dersom det blir kræsj
            self.activetile.flipped = pepsimaxi;
            self.fillactivetile();
        }
    }

    fn iscrash(&mut self) -> bool {
        for bb in &self.activetile.blocks {
            if self.gettileid((self.activetile.x+bb.0, self.activetile.y+bb.1)) > 0 {
                //kræsj
                return true;
            }
        }

        return false;
    }

    fn fillactivetile(&mut self) {
        self.activetile.blocks.clear();
        self.activetile.bounds = (0,0);

        for bb in &self.tiles[self.activetile.tileid as usize].blocks {
            let mut gg = *bb;

            match self.activetile.flipped {
                1 => {gg = (-gg.1, gg.0)},
                2 => {gg = (-gg.0, -gg.1)}, //rett
                3 => {gg = (gg.1, -gg.0)},
                _ => {},
            }

            if gg.0 < self.activetile.bounds.0 {self.activetile.bounds.0 = gg.0}
            if gg.0 > self.activetile.bounds.1 {self.activetile.bounds.1 = gg.0}
            self.activetile.blocks.push(gg);
        }
    }

    fn newactivetile(&mut self, tileid:i32) {
        
        self.activetile.flipped = thread_rng().gen_range(0.0, 3.0) as i32;;
        self.activetile.tileid = tileid;
        self.activetile.bounds = (0,0);

        self.fillactivetile();

        self.activetile.y = 2;
        self.activetile.active = true;
        self.activetile.x = thread_rng().gen_range(2.0, (BOARDSIZE.0 as f32)-2.0) as i32;//BOARDSIZE.0 / 2;
        
    }

    fn updateboard(&mut self, updatenow : bool) {
        if updatenow == true || Instant::now() - self.lastupdate >= Duration::from_millis(152) {
            
            if self.activetile.active {
                self.activetile.y += 1;
                //sjekk om kræsj

                let krasj = self.iscrash();

                if krasj == true {
                    self.activetile.y -= 1;
                    for bb in &self.activetile.blocks {
                        self.board[gameboard::get1dcoords((self.activetile.x+bb.0, self.activetile.y+bb.1)) as usize] = self.tiles[self.activetile.tileid as usize].color;
                    }
                    self.activetile.active = false;
                }

            } else {
                self.newactivetile(thread_rng().gen_range(0.0, 7.0) as i32);
            }
            self.lastupdate = Instant::now();
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
            for bb in &self.activetile.blocks {
                blockstorender.push(block{x:self.activetile.x+bb.0, y:self.activetile.y+bb.1, color:color});
            }
        }


        blockstorender
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.gboard.updateboard(false);
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

    fn key_down_event(&mut self,ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {self.gboard.flipbrick()},
            KeyCode::Down => {self.gboard.updateboard(true)},
            KeyCode::Left => {self.gboard.movebrick(-1)},
            KeyCode::Right => self.gboard.movebrick(1),
            _ => {},
        }
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