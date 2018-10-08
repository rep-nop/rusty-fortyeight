// rendering everything

// namespacing
use std::ops::{Index, IndexMut};
use quicksilver::{
	Future,
	geom::{Rectangle},
	graphics::{Image},
	lifecycle::{Asset},
};

// struct to hold all of the image assets
pub struct SpriteSheet {
	sprites: Vec<Asset<Image>>,
}

// allow for indexing of the sprites
impl Index<usize> for SpriteSheet {
	type Output = Asset<Image>;

	fn index(&self, idx: usize) -> &Asset<Image> {
		&self.sprites[idx]
	}
}

// allow for indexing of mutable sprites
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

		SpriteSheet { sprites }
	}
}