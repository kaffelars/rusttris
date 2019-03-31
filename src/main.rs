use ggez::*;
use ggez::event::{KeyCode, KeyMods};
extern crate rand;
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};
extern crate cgmath;
extern crate mint;

const BOARDSIZE: (i32, i32) = (12, 22);
const TILESIZE: i32 = 36;
const NUMBEROFTILES: i32 = BOARDSIZE.1 * BOARDSIZE.0;


struct State {
    gboard: gameboard,
    score:i32,
    font: graphics::Font,
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
    gamespeed:u64,
    gameover:bool,
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
            board.push(0);

            /*if x > NUMBEROFTILES - 91 {
                board.push(thread_rng().gen_range(0.0, 8.0) as i32);
            } else {
                board.push(0);
            }*/
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

        /*let blocks = vec![(0,0),(0,1),(0,-1),(1,0),(-1,0),(0,2),(0,-2),(2,0),(-2,0),(-2,-1),(-2,-2),(2,1),(2,2),(-1,2),(-2,2),(1,-2),(2,-2)];
        let tilo = tile{blocks:blocks, color:8};
        tiles.push(tilo);*/
        gameboard {board: board, tiles: tiles, activetile: brick{active: false, tileid: 0, flipped: 0, blocks: Vec::new(), bounds:(0,0), x: 0, y: 0}, lastupdate : Instant::now(), gamespeed: 120, gameover: false}
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
        if xy.0 < 0 || xy.0 >= BOARDSIZE.0 || xy.1 < 0 || xy.1 >= BOARDSIZE.1 {
            1 //out of bounds
        } else {
            self.board[gameboard::get1dcoords(xy) as usize]
        }
    }

    fn movebrick(&mut self, x:i32) {
        self.activetile.x += x;

        if self.iscrash() {
            self.activetile.x -= x;
        }

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

            if -gg.0 > self.activetile.bounds.0 {self.activetile.bounds.0 = -gg.0}
            if gg.0 > self.activetile.bounds.1 {self.activetile.bounds.1 = gg.0}
            self.activetile.blocks.push(gg);
        }
    }

    fn gameover(&mut self) {
        self.gameover = true;
    }

    fn newactivetile(&mut self, tileid:i32) {
        
        self.activetile.flipped = thread_rng().gen_range(0.0, 3.0) as i32;;
        self.activetile.tileid = tileid;
        self.activetile.bounds = (0,0);

        self.fillactivetile();

        self.activetile.y = 2;
        self.activetile.active = true;
        self.activetile.x = thread_rng().gen_range(2.0, (BOARDSIZE.0 as f32)-2.0) as i32;//BOARDSIZE.0 / 2;

        if self.iscrash() {
            self.gameover();
        }
        
    }

    fn removerows(&mut self) -> i32 {
        let mut removedrows = 0;

        for y in 0..BOARDSIZE.1 { //sløs
            let mut hole = 0;
            for x in 0..BOARDSIZE.0 {
                if self.board[gameboard::get1dcoords((x, y)) as usize] == 0 {
                    hole = 1;
                }
            }
            if hole == 0 {
                removedrows += 1;
                for yy in (1..y+1).rev() {
                    for xx in 0..BOARDSIZE.0 {
                        self.board[gameboard::get1dcoords((xx, yy)) as usize] = self.board[gameboard::get1dcoords((xx, yy-1)) as usize];
                    }
                }

                for xx in 0..BOARDSIZE.0 {
                    self.board[gameboard::get1dcoords((xx, 0)) as usize] = 0;
                }
            }
        }

        removedrows
    }

    fn updateboard(&mut self, updatenow : bool, score: &mut i32) {
        if self.gameover == false && (updatenow == true || Instant::now() - self.lastupdate >= Duration::from_millis(self.gamespeed)) {
            
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

                    *score+=1;

                    let removedrows = self.removerows();

                    if removedrows > 0 {
                        *score += match removedrows {
                            1 => 10,
                            2 => 25,
                            3 => 40,
                            4 => 75,
                            _ => 3,
                        }
                    }
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

        //bruk meshbuilder - og ikke oppdater alt for hver frame ffs

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
        self.gboard.updateboard(false, &mut self.score);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        //ui
        let pep = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), 
                ggez::graphics::Rect::new_i32(0, 0, BOARDSIZE.0 * TILESIZE +1, BOARDSIZE.1 * TILESIZE+1), 
                gameboard::getcolor(1)).unwrap();
        
        graphics::draw(ctx, &pep, graphics::DrawParam::default()).unwrap();

        let pep = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.0), 
                ggez::graphics::Rect::new_i32(BOARDSIZE.0 * TILESIZE +25, 25, 100,100), 
                gameboard::getcolor(1)).unwrap();
        
        graphics::draw(ctx, &pep, graphics::DrawParam::default()).unwrap();
        //
        let score = format!("Score: {}", self.score);
        let scorep = graphics::Text::new((score, self.font, 32.0));
        graphics::draw(ctx, &scorep, (mint::Point2{x:(BOARDSIZE.0 * TILESIZE +25) as f32, y:160.0 as f32}, 0.0, makecolor(0.7,0.7, 1.0)));


        let torender = self.gboard.renderboard();

        for blok in &torender {
            let pep = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), 
                    ggez::graphics::Rect::new_i32(blok.x*TILESIZE+1, blok.y*TILESIZE+1, TILESIZE-2, TILESIZE-2), 
                    gameboard::getcolor(blok.color)).unwrap();

            graphics::draw(ctx, &pep, graphics::DrawParam::default()).unwrap();
        }

        if self.gboard.gameover {
            let score = format!("GG no re! Final score: {}", self.score);
            let scorep = graphics::Text::new((score, self.font, 64.0));
            graphics::draw(ctx, &scorep, (mint::Point2{x:25.0 as f32, y:230.0 as f32}, 0.0, makecolor(1.0,0.8, 0.6)));
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self,ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => {self.gboard.flipbrick()},
            KeyCode::Down => {self.gboard.updateboard(true, &mut self.score)},
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

    let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();

    let state = &mut State {gboard: gboard, score: 0, font:font};

    event::run(ctx, event_loop, state).unwrap();
}