// Copyright 2020, Michael Connor Buchan and the Blockz contributors
// SPDX-License-Identifier: MIT

//! Contains the TileSet struct, responsible for drawing and updating the tile map.

use std::rc::Rc;

use olc_pixel_game_engine::{self as olc, Vi2d};

use super::{Direction, MovableBlock, Player, SolidBlock, Tile, TILE_SIZE};

/// Represents a level of tiles, draws the tiles upon request, and allows this tile map to be updated.
pub struct TileSet {
    /// The tiles themselves. This is anything that implements the Tile trait
    tiles: Vec<Option<Rc<dyn Tile>>>,
    /// The current location of the player in tile-space
    pub player_pos: Vi2d,
    /// The width of the play-field
    width: i32,
    /// The height of the play-field
    height: i32,
}

impl TileSet {
    /// Load a tile set from a string representing the level.
    pub fn load(level: &str) -> Self {
        let width = olc::screen_width() / TILE_SIZE;
        let height = olc::screen_height() / TILE_SIZE;
        let mut player_pos = Vi2d { x: 0, y: 0 };
        let mut tiles: Vec<Option<Rc<dyn Tile>>> = Vec::with_capacity(level.len());
        for (idx, c) in level.chars().enumerate() {
            tiles.push(match c {
                'p' => {
                    player_pos.x = idx as i32 % width as i32;
                    player_pos.y = idx as i32 / width as i32;
                    Some(Rc::new(Player {}))
                }
                '#' => Some(Rc::new(SolidBlock {})),
                '+' => Some(Rc::new(MovableBlock {})),
                _ => None,
            });
        }
        Self {
            tiles,
            width,
            height,
            player_pos,
        }
    }

    /// Draw each tile in the tile set using the Pixel Game Engine
    pub fn draw(&self) {
        olc::clear(olc::BLACK);
        let mut pos = Vi2d { x: 0, y: 0 };
        while pos.y < self.height {
            while pos.x < self.width {
                let idx = (pos.y * self.width + pos.x) as usize;
                if let Some(tile) = &self.tiles[idx] {
                    tile.draw(pos);
                }
                pos.x += 1;
            }
            pos.x = 0;
            pos.y += 1;
        }
    }

    /// Returns an Rc<t> smart pointer pointing to the Tile trait object at the specified position
    pub fn get_tile(&self, pos: Vi2d) -> Option<Rc<dyn Tile>> {
        self.tiles[(pos.y * self.width + pos.x) as usize].clone()
    }

    /// Determines if the player can move in the specified direction. If the player **n** move, this
    /// function returns Some(position) where position is the tile-space position of one tile past the
    /// last tile that needs to be moved. If the player **can not** move, None is returned.
    pub fn can_player_move(&self, dir: Direction) -> Option<Vi2d> {
        let mut last = Vi2d::new(0, 0);
        None
    }
}
