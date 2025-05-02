use std::collections::HashSet;

use anyhow::Result;
use bevy::{ecs::component::Component, math::Vec2};
use chrono::prelude::*;
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub bombs: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, bombs: usize) -> Board {
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

        Board {
            tiles,
            width,
            height,
            bombs,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Action {
    Flag,
    Reveal,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub when: DateTime<Utc>,
    pub action: Action,
}

#[derive(Clone, Serialize, Deserialize, Component)]
pub struct Game {
    pub board: Board,
    pub started: DateTime<Utc>,
    pub finished: Option<DateTime<Utc>>,
    pub game_over: bool,
    pub game_won: bool,
    pub log: Vec<LogEntry>,
    pub top_left: Vec2,
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
        let board = Board::new(width, height, bombs);
        let started = Utc::now();
        let finished = None;
        let game_over = false;
        let game_won = false;
        let log = Vec::new();
        let top_left = Vec2::new(
            -TILE_SIZE_WITH_GAP * width as f32 / 2.0,
            TILE_SIZE_WITH_GAP * height as f32 / 2.0,
        );

        Game {
            board,
            started,
            finished,
            game_over,
            game_won,
            log,
            top_left,
        }
    }

    pub fn tile_position(&self, x: usize, y: usize) -> Vec2 {
        Vec2::new(
            self.top_left.x + (x as f32 * TILE_SIZE_WITH_GAP),
            self.top_left.y - (y as f32 * TILE_SIZE_WITH_GAP),
        )
    }

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        self.board.tiles[y][x]
    }

    pub fn tile_number(&self, x: usize, y: usize) -> u8 {
        if x < self.board.width && y < self.board.height {
            self.board.tiles[y][x].number
        } else {
            0
        }
    }

    pub fn perform_action(&mut self, x: usize, y: usize, action: Action) -> Result<Response> {
        if self.game_over {
            return Err(GameError::GameAlreadyOver.into());
        }

        let response = match action {
            Action::Flag => {
                if self.board.tiles[y][x].revealed {
                    return Err(GameError::CannotModifyVisibleTile.into());
                }
                self.board.tiles[y][x].flagged = !self.board.tiles[y][x].flagged;
                if self.board.tiles[y][x].flagged {
                    Response::Flag
                } else {
                    Response::Unflag
                }
            }
            Action::Reveal => {
                if self.board.tiles[y][x].flagged {
                    return Err(GameError::CannotRevealFlaggedTile.into());
                }
                if self.board.tiles[y][x].bomb {
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

        self.log.push(LogEntry {
            when: Utc::now(),
            action: action.clone(),
        });

        Ok(response)
    }

    pub fn all_tiles_revealed(&self) -> bool {
        for row in &self.board.tiles {
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
        if self.board.tiles[y][x].revealed {
            return;
        }

        self.board.tiles[y][x].revealed = true;
        tiles.insert((x, y));

        if self.board.tiles[y][x].number == 0 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0
                        && nx < self.board.width as isize
                        && ny >= 0
                        && ny < self.board.height as isize
                    {
                        self.reveal_tiles_recursively(tiles, nx as usize, ny as usize);
                    }
                }
            }
        }
    }

    pub fn finish_game(&mut self, won: bool) {
        self.game_over = true;
        self.game_won = won;
        self.finished = Some(Utc::now());
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
        let adjusted_x = world_position.x + (TILE_SIZE_WITH_GAP * self.board.width as f32) / 2.0;
        let adjusted_y = world_position.y - (TILE_SIZE_WITH_GAP * self.board.height as f32) / 2.0;

        let x = (adjusted_x / TILE_SIZE_WITH_GAP).round();
        let y = (-adjusted_y / TILE_SIZE_WITH_GAP).round();

        if x < 0.0 || y < 0.0 {
            return None; // Out of bounds
        }
        if x >= self.board.width as f32 || y >= self.board.height as f32 {
            return None; // Out of bounds
        }

        Some((x as usize, y as usize))
    }
}
