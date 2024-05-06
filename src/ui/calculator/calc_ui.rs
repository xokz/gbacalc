extern crate alloc;
extern crate core;
use crate::Vector2D;
use alloc::*;

use crate::ui::core::{Cell, CellType, Interface, BLANK, UI};

// generates the calculators UI
// should probably be a compile time macro but whatever it only runs once anyways
pub fn make_ui() -> Interface {
    let mut ui: Interface = Interface {
        cells: vec![
            vec![
                Cell {
                    cell_type: CellType::Blank,
                    tile_index: BLANK,
                    is_pressed: false,
                };
                5
            ];
            30
        ],
        cursor: Vector2D::new(0, 0),
    };

    // number pad
    ui.add_manager((2, 3), (1, 1), b" ", b" ");
    ui.add_manager((0, 3), (1, 1), b".", b".");
    ui.add_manager((1, 3), (1, 1), b"0", b"0");
    ui.add_manager((0, 0), (1, 1), b"1", b"1");
    ui.add_manager((1, 0), (1, 1), b"2", b"2");
    ui.add_manager((2, 0), (1, 1), b"3", b"3");
    ui.add_manager((0, 1), (1, 1), b"4", b"4");
    ui.add_manager((1, 1), (1, 1), b"5", b"5");
    ui.add_manager((2, 1), (1, 1), b"6", b"6");
    ui.add_manager((0, 2), (1, 1), b"7", b"7");
    ui.add_manager((1, 2), (1, 1), b"8", b"8");
    ui.add_manager((2, 2), (1, 1), b"9", b"9");

    // enter button
    ui.add_manager((0, 4), (3, 1), b"==>", b"cmd_enter");

    // operators
    ui.add_manager((4, 0), (1, 1), b"+", b"+");
    ui.add_manager((5, 0), (1, 1), b"=", b"=");
    ui.add_manager((4, 1), (1, 1), b"-", b"-");
    ui.add_manager((5, 1), (1, 1), b"(", b"(");
    ui.add_manager((4, 2), (1, 1), b"*", b"*");
    ui.add_manager((5, 2), (1, 1), b")", b")");
    ui.add_manager((4, 3), (1, 1), b"/", b"/");
    ui.add_manager((5, 3), (1, 1), b",", b",");
    ui.add_manager((4, 4), (1, 1), b"%", b"%");
    ui.add_manager((5, 4), (1, 1), b"^", b"^");

    // alphabet
    ui.add_manager((7, 0), (1, 1), b"a", b"a");
    ui.add_manager((8, 0), (1, 1), b"b", b"b");
    ui.add_manager((9, 0), (1, 1), b"c", b"c");
    ui.add_manager((10, 0), (1, 1), b"d", b"d");
    ui.add_manager((11, 0), (1, 1), b"e", b"e");
    ui.add_manager((7, 1), (1, 1), b"f", b"f");
    ui.add_manager((8, 1), (1, 1), b"g", b"g");
    ui.add_manager((9, 1), (1, 1), b"h", b"h");
    ui.add_manager((10, 1), (1, 1), b"i", b"i");
    ui.add_manager((11, 1), (1, 1), b"j", b"j");
    ui.add_manager((7, 2), (1, 1), b"k", b"k");
    ui.add_manager((8, 2), (1, 1), b"l", b"l");
    ui.add_manager((9, 2), (1, 1), b"m", b"m");
    ui.add_manager((10, 2), (1, 1), b"n", b"n");
    ui.add_manager((11, 2), (1, 1), b"o", b"o");
    ui.add_manager((7, 3), (1, 1), b"p", b"p");
    ui.add_manager((8, 3), (1, 1), b"q", b"q");
    ui.add_manager((9, 3), (1, 1), b"r", b"r");
    ui.add_manager((10, 3), (1, 1), b"s", b"s");
    ui.add_manager((11, 3), (1, 1), b"t", b"t");
    ui.add_manager((7, 4), (1, 1), b"u", b"u");
    ui.add_manager((8, 4), (1, 1), b"v", b"v");
    ui.add_manager((9, 4), (1, 1), b"w", b"w");
    ui.add_manager((10, 4), (1, 1), b"x", b"x");
    ui.add_manager((11, 4), (1, 1), b"y", b"y");
    ui.add_manager((12, 4), (1, 1), b"z", b"z");
    ui.add_manager((12, 0), (1, 1), b" ", b" ");
    ui.add_manager((12, 1), (1, 1), b"(", b"(");
    ui.add_manager((12, 2), (1, 1), b")", b")");
    ui.add_manager((12, 3), (1, 1), b",", b",");

    // functions
    ui.add_manager((14, 0), (4, 1), b"sin", b"sin(");
    ui.add_manager((14, 1), (4, 1), b"cos", b"cos(");
    ui.add_manager((14, 2), (4, 1), b"tan", b"tan(");
    ui.add_manager((19, 0), (4, 1), b"asin", b"asin(");
    ui.add_manager((19, 1), (4, 1), b"acos", b"acos(");
    ui.add_manager((19, 2), (4, 1), b"atan", b"atan(");
    ui.add_manager((14, 3), (4, 1), b"sqrt", b"sqrt(");
    ui.add_manager((14, 4), (4, 1), b"dbug", b"sqr(x) = x * x");
    ui.add_manager((19, 3), (4, 1), b"log", b"log(");
    ui.add_manager((19, 4), (4, 1), b"ln", b"ln(");

    // constants
    ui.add_manager((24, 1), (3, 1), b"pi", b"pi");
    ui.add_manager((24, 2), (3, 1), b"e", b"e");
    ui.add_manager((24, 0), (3, 1), b"tau", b"tau");
    ui.add_manager((27, 0), (3, 1), b"ans", b"ans");

    // settings bar
    ui.add_manager((27, 4), (3, 1), b"RAD", b"cmd_set_angle_radians");
    ui.add_manager((24, 4), (3, 1), b"DEG", b"cmd_set_angle_degrees");

    // return the finished UI
    ui
}
