// the game board

// namespacing
use std::ops::{Index, IndexMut};
use rand::prelude::*;   
use rand::Rng;
use quicksilver::{
    graphics::Image,
    lifecycle::Asset,
};

// for returning with new tiles
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    // create a new coord   
    fn new(dim: (usize, usize)) -> Self {
        Coord {
            x: dim.0,
            y: dim.1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MoveOpt {
    Up,
    Down,
    Left,
    Right,
    Undo,
}

// represents tiles and their values
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Two = 2, 
    Four = 4,
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
    SixtyFour = 64,
    OneTwentyEight = 128,
    TwoFiftySix = 256,
    FiveTwelve = 512,
    //OneThousandTwentyFour = 1024,
    //TwoThousandFortyEight = 2048,
}

impl Tile {
    // creates a new tile
    fn new(board: &Board) -> (Tile, Coord) {
        let mut rng = thread_rng();
        let tile = Tile::Two;
        let mut coord = Coord::new((rng.gen_range(0, 3), rng.gen_range(0, 3)));

        while !board.tile_exists(&coord) {
            coord = Coord::new((rng.gen_range(0, 3), rng.gen_range(0, 3)));
        }

        (tile, coord)
    }
}

// all of the image assets to be used by the game board
#[allow(dead_code)]
struct Assets {
    empty: Asset<Image>,
    two: Asset<Image>,
    four: Asset<Image>,
    eight: Asset<Image>,
    sixteen: Asset<Image>,
    thirtytwo: Asset<Image>,
    sixtyfour: Asset<Image>,
    onetwentyeight: Asset<Image>,
    twofiftysix: Asset<Image>,
    fivetwelve: Asset<Image>,
    //onethousandtwentyfour: Asset<Image>,
    //twothousandfortyeight: Asset<Image>,
}

impl Assets {
    // loads all images to assets struct
    fn load() -> Self {
        Assets {
            empty: Asset::new(Image::load("static/img/empty.png")),
            two: Asset::new(Image::load("static/img/two.png")),
            four: Asset::new(Image::load("static/img/four.png")),
            eight: Asset::new(Image::load("static/img/eight.png")),
            sixteen: Asset::new(Image::load("static/img/sixteen.png")),
            thirtytwo: Asset::new(Image::load("static/img/thirtytwo.png")),
            sixtyfour: Asset::new(Image::load("static/img/sixtyfour.png")),
            onetwentyeight: Asset::new(Image::load("static/img/onetwentyeight.png")),
            twofiftysix: Asset::new(Image::load("static/img/twofiftysix.png")),
            fivetwelve: Asset::new(Image::load("static/img/fivetwelve.png")),
            //onethousandtwentyfour: Asset::new(Image::load("static/img/empty.png")),     // WTF WHERE DID MY FILE GO
            //twothousandfortyeight: Asset::new(Image::load("static/img/empty.png")),     // WTF WHERE DID MY FILE GO
        }
    }
}

// struct to represent the game board
pub struct Board {
    assets: Assets,
    dimensions: (u8, u8),
    current: Vec<Tile>,
    last: Vec<Tile>,
}

// allow indexing board
impl Index<(usize, usize)> for Board {
    type Output = Tile;

    fn index(&self, idx: (usize, usize)) -> &Tile {
        &self.current[(idx.1 * (self.dimensions.0 as usize) + idx.0)]
    }
}

// allow indexing of mutable board
impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Tile {
        &mut self.current[(idx.1 * (self.dimensions.0 as usize) + idx.0)]
    }
}

// board methods
impl Board {
    // creates a new board with initial
    pub fn new(dimensions: (u8, u8)) -> Self {
        let current = vec![Tile::Empty; (dimensions.0 * dimensions.1) as usize];
        let assets = Assets::load();
        let last = current.clone();
        Board {
            assets,
            dimensions,
            current,
            last,
        }
    }

    // compares a new tile to the existing board
    fn tile_exists(&self, coord: &Coord) -> bool {
        let tile = &self[(coord.x as usize, coord.y as usize)];
        if tile != &Tile::Empty {
            false
        } else {
            true
        }
    }

    // writes a tile to the board
    fn write_tile(&mut self, (new_tile, coord): (Tile, Coord)) -> &mut Self {
        self[(coord.x, coord.y)] = new_tile;
        self
    }

    // returns asset for tile type
    fn asset_from_tile(&self, tile: Tile) -> &Asset<Image> {
        match tile {
            Tile::Empty => &self.assets.empty,
            Tile::Two => &self.assets.two,
            Tile::Four => &self.assets.four,
            Tile::Eight => &self.assets.eight,
            Tile::Sixteen => &self.assets.sixteen,
            Tile::ThirtyTwo => &self.assets.thirtytwo,
            Tile::SixtyFour => &self.assets.sixtyfour,
            Tile::OneTwentyEight => &self.assets.onetwentyeight,
            Tile::TwoFiftySix => &self.assets.twofiftysix,
            Tile::FiveTwelve => &self.assets.fivetwelve,
            //Tile::OneThousandTwentyFour => &self.assets.onethousandtwentyfour,
            //Tile::TwoThousandFortyEight => &self.assets.twothousandfortyeight,
        }
    }

    // randomly generates the first tiles
    pub fn starting_tiles(&mut self) -> &mut Self {
        let new1 = Tile::new(&self);
        let new2 = Tile::new(&self);
        
        self.write_tile(new1);
        self.write_tile(new2);

        self
    }

    // compares tile to board returns if a tile exists, and if the tile is == returns new tile type
    fn compare(&self, tile: &Tile, mov: &Option<MoveOpt>, loc: Coord) -> (bool, Option<Tile>) {
        let target_tile = match mov {
            Some(MoveOpt::Up) => &self[(loc.x, loc.y - 1)],
            Some(MoveOpt::Down) => &self[(loc.x, loc.y + 1)],
            Some(MoveOpt::Left) => &self[(loc.x - 1, loc.y)],
            Some(MoveOpt::Right) => &self[(loc.x + 1, loc.y)],

            _ => panic!("this should not happen"),
        };

        if target_tile == &Tile::Empty {
            return (false, None)
        } else if target_tile == tile {
            return match tile {
                Tile::Two => (true, Some(Tile::Four)),
                Tile::Four => (true, Some(Tile::Eight)),
                Tile::Eight => (true, Some(Tile::Sixteen)),
                Tile::Sixteen => (true, Some(Tile::ThirtyTwo)),
                Tile::ThirtyTwo => (true, Some(Tile::SixtyFour)),
                Tile::SixtyFour => (true, Some(Tile::OneTwentyEight)),
                Tile::OneTwentyEight => (true, Some(Tile::TwoFiftySix)),
                Tile::TwoFiftySix => (true, Some(Tile::FiveTwelve)),
                //Some(Tile::FiveTwelve) => (true, Some(Tile::OneThousandTwentyFour)),
                //Some(Tile::OneThousandTwentyFour) => (true, Some(Tile::twothousandfortyeight)),

                _ => panic!("this should not happen"),
            }
        } else {
            (true, None)
        }
    }

    // move (oh boy this one's gonna be fucking nasty)
    pub fn make_move(&mut self, move_opt: &Option<MoveOpt>) -> &mut Self {
        match move_opt {
            Some(MoveOpt::Up) => {
                for x in 0..self.dimensions.0 {
                    for y in 0..self.dimensions.1 {
                        if y > 0 {
                            let tile = &self[(x as usize, y as usize)];
                            self.compare(tile, &move_opt, Coord::new((x as usize, y as usize)));
                        }
                    }
                }
                println!("Up");
            },
            Some(MoveOpt::Down) => {
                for x in 0..self.dimensions.0 {
                    for y in 0..self.dimensions.1 {
                        if y < (self.dimensions.1 - 1) {
                            let tile = &self[(x as usize, y as usize)];
                            self.compare(tile, &move_opt, Coord::new((x as usize, y as usize)));
                        }
                    }
                }
                println!("Down");
            },
            Some(MoveOpt::Left) => {
                for y in 0..self.dimensions.1 {
                    for x in 0..self.dimensions.0 {
                        if x > 0 {
                            let tile = &self[(x as usize, y as usize)];
                            self.compare(tile, &move_opt, Coord::new((x as usize, y as usize)));
                        }
                    }
                }
                println!("Left");
            },
            Some(MoveOpt::Right) => {
                for y in 0..self.dimensions.1 {
                    for x in 0..self.dimensions.0 {
                        if x < (self.dimensions.0 - 1) {
                            let tile = &self[(x as usize, y as usize)];
                            self.compare(tile, &move_opt, Coord::new((x as usize, y as usize)));
                        }
                    }
                }
                println!("Right");
            },
            Some(MoveOpt::Undo) => {
                let tmp_current = self.current.clone();
                let tmp_last = self.last.clone();
                self.current = tmp_last;
                self.last = tmp_current;
                println!("Undo");
            },
            None => {
                panic!("this should not happen");
            },
        }

        self
    }
}