use std::collections::HashSet;

use bevy::{ecs::resource::Resource, math::Vec2};
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const TILE_SIZE: f32 = 32.0;
pub const TILE_GAP: f32 = 2.0;
pub const TILE_SIZE_WITH_GAP: f32 = TILE_SIZE + TILE_GAP;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub revealed: bool,
    pub bomb: bool,
    pub flagged: bool,
    pub number: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Action {
    Flag,
    Reveal,
}

#[derive(Clone, Serialize, Deserialize, Resource)]
pub struct Game {
    pub board: Vec<Vec<Tile>>,
    pub game_over: bool,
    pub game_won: bool,
    pub top_left: Vec2,
    pub width: usize,
    pub height: usize,
    pub bombs: usize,
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Cannot modify a visible tile")]
    CannotModifyVisibleTile,
    #[error("Cannot reveal a flagged tile")]
    CannotRevealFlaggedTile,
    #[error("Game is over")]
    GameAlreadyOver,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    GameOver,
    GameWon,
    Flag,
    Unflag,
    Reveal(HashSet<(usize, usize)>),
}

impl Game {
    pub fn new(width: usize, height: usize, bombs: usize) -> Game {
        let board = Self::initialize_board(width, height, bombs);
        let game_over = false;
        let game_won = false;
        let top_left = Vec2::new(
            -TILE_SIZE_WITH_GAP * width as f32 / 2.0,
            TILE_SIZE_WITH_GAP * height as f32 / 2.0,
        );

        Game {
            board,
            game_over,
            game_won,
            top_left,
            width,
            height,
            bombs,
        }
    }

    pub fn initialize_board(width: usize, height: usize, bombs: usize) -> Vec<Vec<Tile>> {
        // Create the tiles.
        let mut tiles: Vec<Vec<Tile>> = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| Tile {
                        revealed: false,
                        bomb: false,
                        flagged: false,
                        number: 0,
                    })
                    .collect()
            })
            .collect();

        // Place the bombs randomly.
        let mut rng = rand::rng();
        for _ in 0..bombs {
            let mut placed = false;
            while !placed {
                let x = rng.random_range(0..width);
                let y = rng.random_range(0..height);
                if !tiles[y][x].bomb {
                    tiles[y][x].bomb = true;
                    placed = true;
                }
            }
        }

        // Calculate the numbers for each tile.
        for y in 0..height {
            for x in 0..width {
                if tiles[y][x].bomb {
                    continue;
                }
                let mut count = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                            if tiles[ny as usize][nx as usize].bomb {
                                count += 1;
                            }
                        }
                    }
                }
                tiles[y][x].number = count;
            }
        }

        tiles
    }

    pub fn reset(&mut self) {
        self.board = Self::initialize_board(self.width, self.height, self.bombs);
        self.game_over = false;
        self.game_won = false;
    }

    pub fn tile_position(&self, x: usize, y: usize) -> Vec2 {
        Vec2::new(
            self.top_left.x + (x as f32 * TILE_SIZE_WITH_GAP),
            self.top_left.y - (y as f32 * TILE_SIZE_WITH_GAP),
        )
    }

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        self.board[y][x]
    }

    pub fn tile_number(&self, x: usize, y: usize) -> u8 {
        if x < self.width && y < self.height {
            self.board[y][x].number
        } else {
            0
        }
    }

    pub fn perform_action(&mut self, x: usize, y: usize, action: Action) -> Result<Response, GameError> {
        if self.game_over {
            return Err(GameError::GameAlreadyOver);
        }

        let response = match action {
            Action::Flag => {
                if self.board[y][x].revealed {
                    return Err(GameError::CannotModifyVisibleTile);
                }
                self.board[y][x].flagged = !self.board[y][x].flagged;
                if self.board[y][x].flagged {
                    Response::Flag
                } else {
                    Response::Unflag
                }
            }
            Action::Reveal => {
                if self.board[y][x].flagged {
                    return Err(GameError::CannotRevealFlaggedTile);
                }
                if self.board[y][x].bomb {
                    self.finish_game(false);
                    Response::GameOver
                } else {
                    let mut tiles = HashSet::new();
                    self.reveal_tiles_recursively(&mut tiles, x, y);
                    if self.all_tiles_revealed() {
                        self.finish_game(true);
                        Response::GameWon
                    } else {
                        Response::Reveal(tiles)
                    }
                }
            }
        };
        Ok(response)
    }

    pub fn all_tiles_revealed(&self) -> bool {
        for row in &self.board {
            for tile in row {
                if !tile.revealed && !tile.bomb {
                    return false;
                }
            }
        }
        true
    }

    pub fn reveal_tiles_recursively(
        &mut self,
        tiles: &mut HashSet<(usize, usize)>,
        x: usize,
        y: usize,
    ) {
        if self.board[y][x].revealed {
            return;
        }

        self.board[y][x].revealed = true;
        tiles.insert((x, y));

        if self.board[y][x].number == 0 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                        self.reveal_tiles_recursively(tiles, nx as usize, ny as usize);
                    }
                }
            }
        }
    }

    pub fn finish_game(&mut self, won: bool) {
        self.game_over = true;
        self.game_won = won;
    }

    pub fn window_to_world(
        &self,
        window: &bevy::window::Window,
        cursor_position: bevy::math::Vec2,
    ) -> bevy::math::Vec2 {
        let world_x = cursor_position.x - (window.width() / 2.0);
        let world_y = -(cursor_position.y - (window.height() / 2.0)); // Invert the y-coordinate
        bevy::math::Vec2::new(world_x, world_y)
    }

    pub fn world_to_tile(&self, world_position: bevy::math::Vec2) -> Option<(usize, usize)> {
        let adjusted_x = world_position.x + (TILE_SIZE_WITH_GAP * self.width as f32) / 2.0;
        let adjusted_y = world_position.y - (TILE_SIZE_WITH_GAP * self.height as f32) / 2.0;

        let x = (adjusted_x / TILE_SIZE_WITH_GAP).round();
        let y = (-adjusted_y / TILE_SIZE_WITH_GAP).round();

        if x < 0.0 || y < 0.0 {
            return None; // Out of bounds
        }
        if x >= self.width as f32 || y >= self.height as f32 {
            return None; // Out of bounds
        }

        Some((x as usize, y as usize))
    }
}
