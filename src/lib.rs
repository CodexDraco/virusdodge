use wasm_bindgen::prelude::*;
use web_sys::console;

pub const MAP_WIDTH:u32 = 20;
pub const MAP_HEIGHT:u32 = 11;
//const WORLD_START:u8 = 0;
//const WORLD_END:u8 = 8;
pub const TILE_WIDTH:u32 = 32;
pub const TILE_HEIGHT:u32 = 32;
pub const TILESET_WIDTH:u32 = 4*TILE_WIDTH;
pub const TILESET_HEIGHT:u32 = TILE_HEIGHT;
pub const CANVAS_WIDTH:u32 = 640;
pub const CANVAS_HEIGHT:u32 = 360;

const KEY_W:u8 = 1;
const KEY_A:u8 = 2;
const KEY_S:u8 = 4;
const KEY_D:u8 = 8;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
	Entrance,
	Floor,
	Wall,
	Exit,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
	Neutral,
	Left,
	LeftDown,
	Down,
	DownRight,
	Right,
	RightUp,
	Up,
}

struct Map {
	tiles: Vec<Tile>,
}

struct Player {
	x: u32,
	y: u32,
	direction: Direction,
}

#[wasm_bindgen]
pub struct World {
	level: u8,
	map: Map,
	player: Player,
	pixel_data: Vec<u8>,
	tiles_data: Vec<u8>,
	keymap: u8,
}

#[wasm_bindgen]
impl World {
	pub fn level(&self) -> u8 {
		self.level
	}

	pub fn pixel_data_ptr(&self) -> *const u8 {
		self.pixel_data.as_ptr()
	}

	pub fn pixel_data_len(&self) -> usize {
		self.pixel_data.len()
	}

	pub fn tiles_data_ptr(&self) -> *const u8 {
		self.tiles_data.as_ptr()
	}

	pub fn tiles_data_len(&self) -> usize {
		self.tiles_data.len()
	}

	pub fn keydown(&mut self, keymap: u8) {
		self.keymap |= keymap;
	}

	pub fn keyup(&mut self, keymap: u8) {
		self.keymap &= !keymap;
	}

	pub fn new() -> World {
		let tiles = (0..MAP_WIDTH*MAP_HEIGHT)
			.map(|i| {
				let x = i % MAP_WIDTH;
				let y = (i / MAP_WIDTH) % MAP_HEIGHT;
				if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1 {
					Tile::Wall
				}
				else {
					Tile::Floor
				}
			})
			.collect();

		World {
			level: 0,
			map: Map {
				tiles,
			},
			player: Player {
				x: 64,
				y: 64,
				direction: Direction::Right,
			},
			pixel_data: vec![0; (CANVAS_WIDTH * CANVAS_HEIGHT * 4) as usize],
			tiles_data: vec![0; (TILESET_WIDTH * TILESET_HEIGHT * 4) as usize],
			keymap: 0,
		}
	}

	pub fn tick(&mut self) {
		if self.keymap & KEY_W == KEY_W {
			self.player.direction = Direction::Up;
		} else if self.keymap & KEY_A == KEY_A {
			self.player.direction = Direction::Left;
		} else if self.keymap & KEY_S == KEY_S {
			self.player.direction = Direction::Down;
		} else if self.keymap & KEY_D == KEY_D {
			self.player.direction = Direction::Right;
		} else {
			self.player.direction = Direction::Neutral;
		}
		
		match self.player.direction {
			Direction::Neutral => (),
			Direction::Left => {
				self.player.x -= 2;
				if self.player.x < TILE_WIDTH {
					self.player.x = TILE_WIDTH;
				}
			},
			Direction::LeftDown => (),
			Direction::Down => {
				self.player.y += 2;
				if self.player.y > (MAP_HEIGHT - 2) * TILE_HEIGHT {
					self.player.y = (MAP_HEIGHT - 2) * TILE_HEIGHT;
				}
			},
			Direction::DownRight => (),
			Direction::Right => {
				self.player.x += 2;
				if self.player.x > (MAP_WIDTH - 2) * TILE_WIDTH {
					self.player.x = (MAP_WIDTH - 2) * TILE_WIDTH;
				}
			},
			Direction::RightUp => (),
			Direction::Up => {
				self.player.y -= 2;
				if self.player.y < TILE_HEIGHT {
					self.player.y = TILE_HEIGHT;
				}
			},
		};
	}

	pub fn render(&mut self) {
		// Draws all the walls
		for i in 0..MAP_WIDTH*MAP_HEIGHT {
			let x = (i % MAP_WIDTH) * TILE_WIDTH;
			let y = (i / MAP_WIDTH) * TILE_HEIGHT;
			let x_offset = if self.map.tiles[i as usize] == Tile::Floor {
				32
			} else {
				64
			};
			draw_tile(&mut self.pixel_data[0..], &self.tiles_data[0..], x, y, x_offset, 0);
		}

		// Draws the player.
		draw_tile(&mut self.pixel_data[0..], &self.tiles_data[0..], self.player.x, self.player.y, 0, 0);
	}
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

fn draw_pixel(pixel_data: &mut [u8], x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
	let i = ((y * CANVAS_WIDTH + x) * 4) as usize;
	pixel_data[i    ] = r;
	pixel_data[i + 1] = g;
	pixel_data[i + 2] = b;
	pixel_data[i + 3] = a;
}

fn draw_tile(pixel_data: &mut[u8], tiles_data: &[u8], x: u32, y: u32, x_offset: u32, y_offset: u32) {
	for tile_x in 0..TILE_WIDTH {
		for tile_y in 0..TILE_HEIGHT {
			let tile_i = (((y_offset + tile_y) * TILESET_WIDTH + x_offset + tile_x) * 4) as usize;
			// Skips transparent pixels
			if tiles_data[tile_i + 3] == 0 {
				continue;
			}
			let pixel_i = (((y + tile_y) * CANVAS_WIDTH + x + tile_x) * 4) as usize;
			pixel_data[pixel_i    ] = tiles_data[tile_i    ];
			pixel_data[pixel_i + 1] = tiles_data[tile_i + 1];
			pixel_data[pixel_i + 2] = tiles_data[tile_i + 2];
			pixel_data[pixel_i + 3] = tiles_data[tile_i + 3];
		}
	}
}
