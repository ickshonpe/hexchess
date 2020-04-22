use ggez;
use ggez::graphics;
use ggez::input::mouse;
use crate::board;

pub const TILE_SIZE: f32 = 30.0;
pub const BOARD_POS: [f32; 2] = [200.0, 70.0];

pub struct App {
    font: ggez::graphics::Font,
    board: board::Board,
    state: AppState,
    last_button: bool
}

#[derive(Clone, Copy, Debug)]
pub enum AppState {
    Waiting(board::Player),
    SelectedPiece(board::Player, usize),
    GameWonBy(board::Player)
}

impl App {
    pub fn new(ctx: &mut ggez::Context) -> App {
        let font = ggez::graphics::Font::new(ctx, "/Topaz-8.ttf").unwrap();
        App {
            state: AppState::Waiting(board::Player::White),
            last_button: mouse::button_pressed(ctx, mouse::MouseButton::Left),
            font,
            board: board::Board::new()
        }
    }
}

impl ggez::event::EventHandler for App {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let pointer_pos = {
            let pointer_pos = mouse::position(ctx);
            if ggez::graphics::screen_coordinates(ctx).contains(pointer_pos) {
                Some(pointer_pos)
            } else { 
                None
            }
        };
        let button_down = mouse::button_pressed(ctx, mouse::MouseButton::Left);
        let button_released = self.last_button && !button_down;
        self.last_button = button_down;
        let pick = if let Some(pointer_pos) = pointer_pos {
            let pick_x = (pointer_pos.x - BOARD_POS[0]) / TILE_SIZE;
            let pick_y = (pointer_pos.y - BOARD_POS[1]) / TILE_SIZE;
            hex_pick([pick_x, pick_y], &self.board.positions)  
        } else {
            None
        };
        match self.state {
            AppState::GameWonBy(_player) => {

            },
            AppState::Waiting(player) => {
                if button_released {
                    if let Some(hex) = pick {
                        if let Some((colour, _piece)) = self.board.hexes[hex] {
                            if player == colour {
                                self.state = AppState::SelectedPiece(player, hex);
                            }
                        }
                    }
                }
            },
            AppState::SelectedPiece(player, selected) => {
                if button_released {
                    if Some(selected) == pick {
                        self.state = AppState::Waiting(player);
                    } else {
                        if let Some(pick) = pick {
                            let mut valid_moves = crate::logic::find_valid_moves(&self.board, selected);
                            valid_moves.retain(|h| crate::logic::is_vulnerable(&self.board, *h));
                            if valid_moves.contains(&pick) {
                                let target = self.board.hexes[pick];
                                if Some((player, crate::pieces::Piece::Pawn)) == self.board.hexes[selected] {
                                    if board::is_queening_hex(pick, player) {
                                        self.board.hexes[pick] = Some((player, crate::pieces::Piece::Queen));
                                        self.board.hexes[selected] = None;
                                    } else {
                                        self.board.hexes[pick] = self.board.hexes[selected];
                                        self.board.hexes[selected] = None;
                                    }
                                } else {
                                    self.board.hexes[pick] = self.board.hexes[selected];
                                    self.board.hexes[selected] = None;
                                }
                                match target {
                                    Some((_, crate::pieces::Piece::King)) => {
                                        self.state = AppState::GameWonBy(player)
                                    },
                                    _ => {
                                        self.state = AppState::Waiting(player.opposite());
                                        self.board.current_turn = player.opposite();
                                        self.board.threats = crate::logic::count_threats(&self.board);

                                    }
                                }

                            } else if let Some((other, _)) = self.board.hexes[pick] {
                                if other == player {
                                    self.state = AppState::SelectedPiece(player, pick);
                                } 
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        
        
        let piece_sz = 32.0;
        let pointer_pos = {
            let pointer_pos = ggez::input::mouse::position(&ctx);
            if ggez::graphics::screen_coordinates(&ctx).contains(pointer_pos) {
                Some(pointer_pos)
            } else { 
                None
            }
        };

        let pick = if let Some(pointer_pos) = pointer_pos {
            let pick_x = (pointer_pos.x - BOARD_POS[0]) / TILE_SIZE;
            let pick_y = (pointer_pos.y - BOARD_POS[1]) / TILE_SIZE;
            hex_pick([pick_x, pick_y], &self.board.positions)  
        } else {
            None
        };

        // draw board
        match self.state {
            AppState::GameWonBy(_player) => {
                let colouring = |_hex|  ggez::graphics::Color::new(0.5, 0.25, 0.0, 1.0);
                draw_hexes(ctx, BOARD_POS, &self.board.positions, TILE_SIZE, 2.0, colouring)?;

            },
            AppState::Waiting(_   ) => {
                let colouring = |hex| match pick {
                    Some(pick) if hex == pick =>  ggez::graphics::Color::new(1.0, 0.5, 0.0, 1.0),
                    _ => ggez::graphics::Color::new(0.5, 0.25, 0.0, 1.0)
                };
                draw_hexes(ctx, BOARD_POS, &self.board.positions, TILE_SIZE, 2.0, colouring)?;
            },
            AppState::SelectedPiece(_, selected_hex) => {   
                let mut valid_moves = crate::logic::find_valid_moves(&self.board, selected_hex);
                valid_moves.retain(|h| crate::logic::is_vulnerable(&self.board, *h));
                let colouring = |hex|
                    if selected_hex == hex { 
                        ggez::graphics::Color::new(1.0, 0.0,0.0, 1.0) 
                    } else if let Some(pick) = pick {
                        if hex == pick && valid_moves.contains(&hex) {
                            ggez::graphics::Color::new(0.0, 1.0, 0.0, 1.0)
                        } else if hex == pick {
                             ggez::graphics::Color::new(1.0, 0.5, 0.0, 1.0)
                        } else if valid_moves.contains(&hex) {
                            ggez::graphics::Color::new(0.0, 0.5, 0.0, 1.0)
                        } else {
                            ggez::graphics::Color::new(0.5, 0.25, 0.0, 1.0)
                        }
                    } else {
                        ggez::graphics::Color::new(0.5, 0.25, 0.0, 1.0)
                    };
                draw_hexes(ctx, BOARD_POS, &self.board.positions, TILE_SIZE, 2.0, colouring)?;
            }
        }

        // draw pieces
        for hex in 0..board::HEX_COUNT {
            if let Some((player, piece)) = self.board.hexes[hex] {
                let pos = self.board.positions[hex];
                let dest = [BOARD_POS[0] - piece_sz / 2.5 + pos[0] * TILE_SIZE, BOARD_POS[1] - piece_sz / 2.5 + pos[1] * TILE_SIZE].into();
                let symbol = ggez::graphics::Text::new((piece.code(), self.font, piece_sz));
                let color = match player {
                    board::Player::White => ggez::graphics::WHITE,
                    board::Player::Black => ggez::graphics::BLACK
                };
                let params = ggez::graphics::DrawParam {
                    dest,
                    color,
                    ..Default::default()
                };
                ggez::graphics::draw(ctx, &symbol, params)?;
            }
        }

        let turn_msg = 
            ggez::graphics::Text::new((
                match self.state {
                    AppState::GameWonBy(player) => {
                        format!("{:?} wins", player)        
                    },
                    _ => {
                        format!("{:?} turn", self.board.current_turn) 
                    }
                }
                 , self.font, 24.0));
        ggez::graphics::draw(ctx, &turn_msg, ([200.0, 8.0],))?;        
        let state_msg = ggez::graphics::Text::new((format!("{:?}", self.state), self.font, 16.0));
        ggez::graphics::draw(ctx, &state_msg, ([200.0, 560.0],))?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn distance2(a: [f32; 2], b: [f32; 2]) -> f32 {
    let d_x = a[0] - b[0];
    let d_y = a[1] - b[1];
    d_x * d_x + d_y * d_y
}

pub fn hex_pick(pick: [f32;2], hex_positions: &[[f32;2]]) -> Option<usize> {
    let mut d = 1.0;
    let mut result = None;
    for (index, hex_pos) in hex_positions.iter().enumerate() {
        let d_sqrd = distance2(pick, *hex_pos);
        if d_sqrd <= d {
            d = d_sqrd;
            result = Some(index);
        }
    }
    result
}

pub fn draw_hexes<F>(ctx: &mut ggez::Context, dest: [f32; 2], hex_positions: &[[f32;2]], tile_sz: f32, border: f32, colouring: F) -> ggez::GameResult<()>
where F: Fn(usize) -> ggez::graphics::Color {
    let hex_mesh = ggez::graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &crate::geometry::HEX_VERTICES,
        graphics::Color::new(1.0, 1.0, 1.0, 1.0)
    )?;
    for (hex, pos) in hex_positions.iter().cloned().enumerate() {
        let target = [dest[0] + pos[0] * tile_sz, dest[1] + pos[1] * tile_sz];
        let color = colouring(hex);
        let params = ggez::graphics::DrawParam {
            dest: target.into(),
            scale: [tile_sz - border, tile_sz - border].into(),
            color,
            .. Default::default()
        };
        graphics::draw(ctx,&hex_mesh, params)?;
    }
    Ok(())
}