// rendering everything

// namespacing
use std::ops::{Index, IndexMut};
use quicksilver::{
    Future,
    geom::{Rectangle, Shape},
    graphics::{Image, Background::Img},
    lifecycle::{Asset, Window},
};
use GameState;

// struct to hold all of the image assets
pub struct SpriteSheet {
    sprite_dim: (u8, u8),
    sprites: Vec<Asset<Image>>,
}

// allow for (nicer) indexing of the sprites
impl Index<usize> for SpriteSheet {
    type Output = Asset<Image>;

    fn index(&self, idx: usize) -> &Asset<Image> {
        &self.sprites[idx]
    }
}

// allow for (nicer) indexing of mutable sprites
impl IndexMut<usize> for SpriteSheet {
    fn index_mut(&mut self, idx: usize) -> &mut Asset<Image> {
        &mut self.sprites[idx]
    }
}

impl SpriteSheet {
    // creates and loads a new sprite sheet (sprite_dim in px, sheet_dim in #)
    pub fn new(sprite_dim: (u8, u8), sheet_dim: (u8, u8), filename: &'static str) -> Self {
        let mut sprites: Vec<Asset<Image>> = Vec::new();
        for x in 0..sheet_dim.0 {
            for y in 0..sheet_dim.1 {
                let asset = Asset::new(
                    Image::load(filename)
                        .map(move |image| image.subimage(
                            Rectangle::new((x * sprite_dim.0, y * sprite_dim.1), (sprite_dim.0, sprite_dim.1))))
                );
                sprites.push(asset);
            }
        }

        println!("loaded images ok!");
        SpriteSheet { sprites, sprite_dim }
    }
}

impl GameState {
    pub fn render_board(&mut self, window: &mut Window) {
        unimplemented!()
    }

    pub fn render_board_bg(&mut self, window: &mut Window) {
        let sprite_dim = self.sprites.sprite_dim;
        let board_dim = self.board.dimensions;

        for x in 0..self.board.dimensions.0 {
            for y in 0..self.board.dimensions.1 {
                let tile = (x, y);
                if tile == (0, 0) {
                    let _ = self.sprites[0].execute(|image| {
                        window.draw(&image.area().with_center(sprite_dim), Img(&image));
                        Ok(())
                    });
                } else if tile == (board_dim.0, 0) {
                    let _ = self.sprites[1].execute(|image| {
                        window.draw(&image.area().with_center(sprite_dim), Img(&image));
                        Ok(())
                    });
                } else if tile == (0, board_dim.1) {
                    let _ = self.sprites[2].execute(|image| {
                        window.draw(&image.area().with_center(sprite_dim), Img(&image));
                        Ok(())
                    });
                } else if tile == (board_dim.0, board_dim.1) {
                    let _ = self.sprites[3].execute(|image| {
                        window.draw(&image.area().with_center(sprite_dim), Img(&image));
                        Ok(())
                    });
                } else {
                    println!("soon:tm:");
                }
            }
        }
    }
}