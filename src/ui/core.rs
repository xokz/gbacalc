extern crate alloc;
extern crate core;
use crate::display::display::Drawing;
use crate::Cursor;
use crate::Vector2D;
use agb::display::object::{Graphics, Sprite};
use agb::display::tiled::{MapLoan, RegularMap, TileSet, TileSetting, VRamManager};
use agb::include_aseprite;
use agb::input::{Button, ButtonController};
use alloc::format;
use alloc::string::String;
use alloc::vec::{self, Vec};

// Tile indexes (First is the index of the unpressed tile, second is pressed)
pub const BLANK: (u16, u16) = (16 * 6 + 3, 16 * 6 + 3);
pub const SINGLE: (u16, u16) = (16 * 8 + 1, 16 * 8 + 4);
pub const VERTICAL_BAR: (u16, u16) = (16 * 9 + 8, 16 * 9 + 9);
pub const LEFT_BAR: (u16, u16) = (16 * 7 + 6, 16 * 8 + 6);
pub const HORIZONTAL_BAR: (u16, u16) = (16 * 9 + 6, 16 * 9 + 7);
pub const RIGHT_BAR: (u16, u16) = (16 * 7 + 7, 16 * 8 + 7);
pub const UPPER_LEFT_CORNER: (u16, u16) = (16 * 7 + 0, 16 * 7 + 3);
pub const UPPER_RIGHT_CORNER: (u16, u16) = (16 * 7 + 2, 16 * 7 + 5);
pub const LOWER_LEFT_CORNER: (u16, u16) = (16 * 9 + 0, 16 * 9 + 3);
pub const LOWER_RIGHT_CORNER: (u16, u16) = (16 * 9 + 2, 16 * 9 + 5);
pub const UP_BAR: (u16, u16) = (16 * 7 + 8, 16 * 7 + 9);
pub const DOWN_BAR: (u16, u16) = (16 * 8 + 8, 16 * 8 + 9);
pub const LEFT_WALL_BAR: (u16, u16) = (16 * 8 + 0, 16 * 8 + 3);
pub const RIGHT_WALL_BAR: (u16, u16) = (16 * 8 + 2, 16 * 8 + 5);
pub const UP_WALL_BAR: (u16, u16) = (16 * 7 + 1, 16 * 7 + 4);
pub const DOWN_WALL_BAR: (u16, u16) = (16 * 9 + 1, 16 * 9 + 4);

#[derive(Clone)]
pub struct Interface {
    pub cells: Vec<Vec<Cell>>,
    pub cursor: Vector2D<u16>,
}

#[derive(Clone)]
pub struct Controller {
    pub size: Vector2D<u16>,
    pub name: Vec<u8>,
    pub action: Vec<u8>,
}

#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub tile_index: (u16, u16),
    pub is_pressed: bool,
}

#[derive(Clone)]
pub enum CellType {
    Blank,
    Pointer(Vector2D<u16>),
    Manager(Controller),
}

fn get_tile_index(size: Vector2D<u16>, pos: Vector2D<u16>) -> (u16, u16) {
    let up = if pos.y > 0 { true } else { false };

    let down = if pos.y < (size.y - 1) { true } else { false };

    let left = if pos.x > 0 { true } else { false };

    let right = if pos.x < (size.x - 1) { true } else { false };

    let surrounding_tiles = [up, down, left, right];

    let tile_index = match surrounding_tiles {
        [false, false, false, false] => SINGLE,
        [false, false, false, true] => LEFT_BAR,
        [false, false, true, false] => RIGHT_BAR,
        [false, false, true, true] => HORIZONTAL_BAR,
        [false, true, false, false] => UP_BAR,
        [false, true, false, true] => UPPER_LEFT_CORNER,
        [false, true, true, false] => UPPER_RIGHT_CORNER,
        [false, true, true, true] => UP_WALL_BAR,
        [true, false, false, false] => DOWN_BAR,
        [true, false, false, true] => LOWER_LEFT_CORNER,
        [true, false, true, false] => LOWER_RIGHT_CORNER,
        [true, false, true, true] => DOWN_WALL_BAR,
        [true, true, false, false] => VERTICAL_BAR,
        [true, true, false, true] => LEFT_WALL_BAR,
        [true, true, true, false] => RIGHT_WALL_BAR,
        [true, true, true, true] => BLANK,
        _ => unreachable!(),
    };

    tile_index
}

pub trait UI {
    fn add_new_button(&mut self, pos: Vector2D<u16>, button: &Cell);
    fn add_manager(&mut self, pos: (u16, u16), size: (u16, u16), name: &[u8], action: &[u8]);
    fn draw_interface(
        &self,
        pos: Vector2D<u16>,
        bg: &mut MapLoan<'_, RegularMap>,
        fg: &mut MapLoan<'_, RegularMap>,
        vram: &mut VRamManager,
        tileset: &TileSet<'_>,
        cursor: &mut Cursor,
    );
    fn handle_input(&mut self, input: &mut ButtonController) -> (i32, Vec<u8>);
}

impl UI for Interface {
    fn add_new_button(&mut self, pos: Vector2D<u16>, button: &Cell) {
        match &button.cell_type {
            CellType::Blank => {
                self.cells[pos.x as usize][pos.y as usize] = Cell {
                    cell_type: CellType::Blank,
                    tile_index: BLANK,
                    is_pressed: false,
                }
            }
            CellType::Manager(controller) => {
                for x in 0..controller.size.x {
                    for y in 0..controller.size.y {
                        self.cells[(x + pos.x) as usize][(y + pos.y) as usize] = Cell {
                            cell_type: CellType::Pointer(pos),
                            tile_index: get_tile_index(controller.size, Vector2D::new(x, y)),
                            is_pressed: false,
                        }
                    }
                }
                self.cells[pos.x as usize][pos.y as usize] = Cell {
                    cell_type: CellType::Manager(controller.clone()),
                    tile_index: get_tile_index(controller.size, Vector2D::new(0, 0)), // manager cell will always be in the top left corner of the button
                    is_pressed: false,
                };
            }
            CellType::Pointer(manager_pos) => {
                self.cells[pos.x as usize][pos.y as usize] = Cell {
                    cell_type: CellType::Pointer(manager_pos.clone()),
                    tile_index: HORIZONTAL_BAR,
                    is_pressed: false,
                };
            }
        }
    }

    // quick and dirty function for easy menu creation
    fn add_manager(&mut self, pos: (u16, u16), size: (u16, u16), name: &[u8], action: &[u8]) {
        let controller = Controller {
            size: Vector2D::from(size),
            name: name.to_vec(),
            action: action.to_vec(),
        };
        let cell = Cell {
            cell_type: CellType::Manager(controller),
            tile_index: SINGLE,
            is_pressed: false,
        };
        self.add_new_button(Vector2D::from(pos), &cell);
    }

    fn draw_interface(
        &self,
        pos: Vector2D<u16>,
        bg: &mut MapLoan<'_, RegularMap>,
        fg: &mut MapLoan<'_, RegularMap>,
        vram: &mut VRamManager,
        tileset: &TileSet<'_>,
        cursor: &mut Cursor,
    ) {
        // loop over all UI tiles, and draw them. text is drawn on top
        for x in 0..self.cells.len() {
            for y in 0..self.cells[0].len() {
                match &self.cells[x][y].cell_type {
                    CellType::Blank => {
                        bg.set_tile(
                            vram,
                            Vector2D::from((x as u16 + pos.x, y as u16 + pos.y)),
                            tileset,
                            TileSetting::from_raw(BLANK.0),
                        );
                    }
                    CellType::Pointer(manager_pos) => {
                        if let CellType::Manager(controller) =
                            &self.cells[manager_pos.x as usize][manager_pos.y as usize].cell_type
                        {
                            bg.set_tile(
                                vram,
                                Vector2D::from((x as u16 + pos.x, y as u16 + pos.y)),
                                tileset,
                                TileSetting::from_raw({
                                    if self.cells[x][y].is_pressed {
                                        self.cells[x][y].tile_index.1
                                    } else {
                                        self.cells[x][y].tile_index.0
                                    }
                                }),
                            );
                        }
                    }
                    CellType::Manager(controller) => {
                        fg.print(
                            vram,
                            tileset,
                            &controller.name,
                            &Vector2D::from((x as u16 + pos.x, y as u16 + pos.y)),
                        );
                        bg.set_tile(
                            vram,
                            Vector2D::from((x as u16 + pos.x, y as u16 + pos.y)),
                            tileset,
                            TileSetting::from_raw({
                                if self.cells[x][y].is_pressed {
                                    self.cells[x][y].tile_index.1
                                } else {
                                    self.cells[x][y].tile_index.0
                                }
                            }),
                        );
                    }
                }
            }
        }

        // draw the 4 cursor sprites at the 4 corners of the current button
        let mut button_size: Vector2D<u16> = Vector2D::new(0, 0);
        if let CellType::Manager(controller) =
            &self.cells[self.cursor.x as usize][self.cursor.y as usize].cell_type
        {
            button_size = controller.size - Vector2D::new(1, 1);
        }
        cursor
            .0
            .set_x(((self.cursor.x + pos.x) << 3) - 1)
            .set_y(((self.cursor.y + pos.y) << 3) - 1)
            .show();
        cursor
            .1
            .set_x(((self.cursor.x + pos.x + button_size.x) << 3) + 1)
            .set_y(((self.cursor.y + pos.y) << 3) - 1)
            .show();
        cursor
            .2
            .set_x(((self.cursor.x + pos.x + button_size.x) << 3) + 1)
            .set_y(((self.cursor.y + pos.y + button_size.y) << 3) + 1)
            .show();
        cursor
            .3
            .set_x(((self.cursor.x + pos.x) << 3) - 1)
            .set_y(((self.cursor.y + pos.y + button_size.y) << 3) + 1)
            .show();
    }

    fn handle_input(&mut self, input: &mut ButtonController) -> (i32, Vec<u8>) {
        let mut user_pressed_button = 0;
        let cursor_pos = &mut self.cursor;
        let ui_size: Vector2D<u16> =
            Vector2D::new((self.cells.len()) as u16, (self.cells[0].len()) as u16);
        let mut action: Vec<u8> = Vec::new();

        let mut button_size: Vector2D<u16> = Vector2D::new(1, 1);
        if let CellType::Manager(controller) =
            &self.cells[cursor_pos.x as usize][cursor_pos.y as usize].cell_type
        {
            button_size = controller.size;
        }

        

        if input.is_released(Button::A) {
            if input.is_just_pressed(Button::UP) {
                user_pressed_button = 1;
                if cursor_pos.y == 0 {
                    cursor_pos.y = ui_size.y - 1;
                } else {
                    cursor_pos.y -= 1;
                }
            }

            if input.is_just_pressed(Button::DOWN) {
                user_pressed_button = 1;
                if cursor_pos.y == ui_size.y - 1 {
                    cursor_pos.y = 0;
                } else {
                    cursor_pos.y += button_size.y;
                    if cursor_pos.y > (self.cells[0].len() - 1) as u16 {
                        cursor_pos.y = 0
                    }
                }
            }

            if input.is_just_pressed(Button::LEFT) {
                user_pressed_button = 1;
                if cursor_pos.x == 0 {
                    cursor_pos.x = ui_size.x - 1;
                } else {
                    cursor_pos.x -= 1;
                }
            }

            if input.is_just_pressed(Button::RIGHT) {
                user_pressed_button = 1;
                if cursor_pos.x == ui_size.x - 1 {
                    cursor_pos.x = 0;
                } else {
                    cursor_pos.x += button_size.x;
                    if cursor_pos.x > (self.cells.len() - 1) as u16 {
                        cursor_pos.x = 0
                    }
                }
            }
        }

        if input.is_just_pressed(Button::A) {
            self.cells[cursor_pos.x as usize][cursor_pos.y as usize].is_pressed = true;
            user_pressed_button = 1;
            match self.cells[cursor_pos.x as usize][cursor_pos.y as usize]
                .cell_type
                .clone()
            {
                CellType::Blank => (),
                CellType::Pointer(manager_pos) => {
                    if let CellType::Manager(controller) = self.cells[manager_pos.x as usize]
                        [manager_pos.y as usize]
                        .cell_type
                        .clone()
                    {
                        action.clear();
                        for i in 0..controller.action.len() {
                            action.push(controller.action[i]);
                        }
                        for x in 0..controller.size.x {
                            for y in 0..controller.size.y {
                                self.cells[(manager_pos.x + x) as usize]
                                    [(manager_pos.y + y) as usize]
                                    .is_pressed = true;
                            }
                        }
                    }
                }
                CellType::Manager(controller) => {
                    action.clear();
                    for i in 0..controller.action.len() {
                        action.push(controller.action[i]);
                    }
                    for x in 0..controller.size.x {
                        for y in 0..controller.size.y {
                            self.cells[(cursor_pos.x + x) as usize][(cursor_pos.y + y) as usize]
                                .is_pressed = true;
                        }
                    }
                }
            }
        }

        if input.is_just_released(Button::A) {
            self.cells[cursor_pos.x as usize][cursor_pos.y as usize].is_pressed = false;
            user_pressed_button = 2;

            match self.cells[cursor_pos.x as usize][cursor_pos.y as usize]
                .cell_type
                .clone()
            {
                CellType::Blank => (),
                CellType::Pointer(manager_pos) => {
                    if let CellType::Manager(controller) = self.cells[manager_pos.x as usize]
                        [manager_pos.y as usize]
                        .cell_type
                        .clone()
                    {
                        for x in 0..controller.size.x {
                            for y in 0..controller.size.y {
                                self.cells[(manager_pos.x + x) as usize]
                                    [(manager_pos.y + y) as usize]
                                    .is_pressed = false;
                            }
                        }
                    }
                    cursor_pos.x = manager_pos.x;
                    cursor_pos.y = manager_pos.y;
                }
                CellType::Manager(controller) => {
                    for x in 0..controller.size.x {
                        for y in 0..controller.size.y {
                            self.cells[(cursor_pos.x + x) as usize][(cursor_pos.y + y) as usize]
                                .is_pressed = false;
                        }
                    }
                }
            }
        }

        if let CellType::Pointer(manager_pos) = self.cells[cursor_pos.x as usize]
            [cursor_pos.y as usize]
            .cell_type
            .clone()
        {
            if let CellType::Manager(controller) = self.cells[manager_pos.x as usize]
                [manager_pos.y as usize]
                .cell_type
                .clone()
            {
                for x in 0..controller.size.x {
                    for y in 0..controller.size.y {
                        self.cells[(manager_pos.x + x) as usize][(manager_pos.y + y) as usize]
                            .is_pressed = false;
                    }
                }
            }
            cursor_pos.x = manager_pos.x;
            cursor_pos.y = manager_pos.y;
        }

        if input.is_just_pressed(Button::B) {
            user_pressed_button = 3;
        }

        if input.is_just_pressed(Button::START) {
            user_pressed_button = 2;
            action = b"cmd_enter".to_vec();
        }
        if input.is_just_pressed(Button::SELECT) {
            user_pressed_button = 2;
            action = b"ans".to_vec();
        }

        (user_pressed_button, action)
    }
}
