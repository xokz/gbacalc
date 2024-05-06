#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

extern crate alloc;

mod display;
mod mathengine;
mod ui;

use agb::{
    display::{
        object::{self, Object, Sprite},
        tiled::{
            MapLoan, RegularBackgroundSize, RegularMap, TileFormat, TileSet, TileSetting, Tiled0,
            TiledMap, VRamManager,
        },
        Priority,
    },
    fixnum::Vector2D,
    include_background_gfx, include_wav,
    input::{Button, ButtonController},
    sound::mixer::{Frequency, SoundChannel},
    syscall::halt,
    timer::{self, Timer},
};

use agb::{
    display::object::{Graphics, Tag},
    include_aseprite,
};
use alloc::*;
use alloc::{
    string::{String, ToString},
    vec::{self, Vec},
};
use mathengine::core::{Calc, CalcEngine};
use ui::core::Interface;

use crate::{
    display::display::Drawing,
    ui::{calculator::calc_ui, core::UI},
};

pub type Cursor<'a> = (Object<'a>, Object<'a>, Object<'a>, Object<'a>);
pub const UI_POSITION: Vector2D<u16> = Vector2D::new(0, 15);

pub fn run(mut gba: agb::Gba) -> ! {
    // initialize gameboy
    let (gfx, mut vram) = gba.display.video.tiled0();
    let vblank = agb::interrupt::VBlank::get();
    let mut input = ButtonController::new();
    let mut mixer = gba.mixer.mixer(Frequency::Hz10512);
    mixer.enable();

    // initialize background tileset
    agb::include_background_gfx!(text_tiles, tiles => "gfx/tile_sheet.png");
    let tileset = text_tiles::tiles.tiles;
    vram.set_background_palettes(text_tiles::PALETTES);

    // create sprites
    const GRAPHICS: &Graphics = include_aseprite!("gfx/sprites.aseprite");
    let object = gba.display.object.get_managed();
    let mut cursor: Cursor = (
        object.object_sprite(GRAPHICS.tags().get("cursor").sprite(0)),
        object.object_sprite(GRAPHICS.tags().get("cursor").sprite(1)),
        object.object_sprite(GRAPHICS.tags().get("cursor").sprite(2)),
        object.object_sprite(GRAPHICS.tags().get("cursor").sprite(3)),
    );

    // create sound
    const BUTTON_PRESS: &[u8] = include_wav!("sfx/key_press.wav");
    const BUTTON_RELEASE: &[u8] = include_wav!("sfx/key_release.wav");

    // background
    // for UI tiles
    let mut bg = gfx.background(
        Priority::P1,
        RegularBackgroundSize::Background32x32,
        tileset.format(),
    );

    // foreground
    // for text
    let mut fg = gfx.background(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        tileset.format(),
    );

    // clear background and foreground
    bg.fill(&mut vram, &tileset, 16 * 6 + 9);
    fg.fill(&mut vram, &tileset, 0);
    bg.commit(&mut vram);
    fg.commit(&mut vram);
    bg.set_visible(true);
    fg.set_visible(true);

    let mut calculator = Calculator {
        interface: calc_ui::make_ui(),
        history: core::array::from_fn(|_| b"".to_vec()),
        user_input: b"".to_vec(),
        engine: CalcEngine::new(),
    };

    let mut update_screen: bool = { true };

    {
        // wait for vblank, then commit all graphics to screen
        vblank.wait_for_vblank();

        // draw all tiles to  foregrond and background
        calculator.interface.draw_interface(
            UI_POSITION,
            &mut bg,
            &mut fg,
            &mut vram,
            &tileset,
            &mut cursor,
        );

        calculator.draw_screen(UI_POSITION, &mut bg, &mut fg, &mut vram, &tileset);

        // commit gfx to screen
        bg.commit(&mut vram);
        fg.commit(&mut vram);
        object.commit();
    }

    loop {
        // update and handle UI input
        input.update();
        let (user_pressed_button, command) = calculator.interface.handle_input(&mut input);

        if user_pressed_button != 0 {
            if user_pressed_button == 3 {
                calculator.user_input.pop();
            } else {
                calculator.handle_command(command);
            }
        }

        update_screen = {
            if user_pressed_button != 0 {
                true
            } else {
                false
            }
        };

        // if the user has changed something onscreen, then draw
        if update_screen {
            // wait for vblank, then commit all graphics to screen
            vblank.wait_for_vblank();

            // draw all tiles to  foregrond and background
            calculator.interface.draw_interface(
                UI_POSITION,
                &mut bg,
                &mut fg,
                &mut vram,
                &tileset,
                &mut cursor,
            );

            calculator.draw_screen(UI_POSITION, &mut bg, &mut fg, &mut vram, &tileset);

            // commit gfx to screen
            bg.commit(&mut vram);
            fg.commit(&mut vram);
            object.commit();
        }

        // do audio
        mixer.frame();
        if user_pressed_button == 1 {
            let mut channel = SoundChannel::new(BUTTON_PRESS);
            channel.stereo();
            let _ = mixer.play_sound(channel);
        }
        if user_pressed_button == 2 || user_pressed_button == 3 {
            let mut channel = SoundChannel::new(BUTTON_RELEASE);
            channel.stereo();
            let _ = mixer.play_sound(channel);
        }
    }
}

struct Calculator {
    interface: Interface,
    history: [Vec<u8>; 14],
    user_input: Vec<u8>,
    engine: CalcEngine,
}

pub trait CalculatorStuff {
    fn draw_screen(
        &self,
        pos: Vector2D<u16>,
        bg: &mut MapLoan<'_, RegularMap>,
        fg: &mut MapLoan<'_, RegularMap>,
        vram: &mut VRamManager,
        tileset: &TileSet<'_>,
    );
    fn handle_command(&mut self, command: Vec<u8>);
}

impl CalculatorStuff for Calculator {
    fn draw_screen(
        &self,
        pos: Vector2D<u16>,
        bg: &mut MapLoan<'_, RegularMap>,
        fg: &mut MapLoan<'_, RegularMap>,
        vram: &mut VRamManager,
        tileset: &TileSet<'_>,
    ) {
        // draw bar that seperates keyboard and screen
        const SEPERATOR_Y: u16 = 14;
        const SEPERATOR: [u16; 30] = [
            122, 124, 124, 123, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124,
            124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 124, 125,
        ];
        const RAD_TILES: (u16, u16) = (16 * 8 + 12, 16 * 8 + 13);
        const DEG_TILES: (u16, u16) = (16 * 8 + 10, 16 * 8 + 11);
        for i in 0..SEPERATOR.len() {
            bg.set_tile(
                vram,
                Vector2D::new(i as u16, SEPERATOR_Y),
                tileset,
                TileSetting::from_raw(SEPERATOR[i]),
            );
        }
        if self.engine.use_radians {
            bg.set_tile(
                vram,
                Vector2D::new(1, SEPERATOR_Y),
                tileset,
                TileSetting::from_raw(RAD_TILES.0),
            );
            bg.set_tile(
                vram,
                Vector2D::new(2, SEPERATOR_Y),
                tileset,
                TileSetting::from_raw(RAD_TILES.1),
            );
        } else {
            bg.set_tile(
                vram,
                Vector2D::new(1, SEPERATOR_Y),
                tileset,
                TileSetting::from_raw(DEG_TILES.0),
            );
            bg.set_tile(
                vram,
                Vector2D::new(2, SEPERATOR_Y),
                tileset,
                TileSetting::from_raw(DEG_TILES.1),
            );
        }

        // draw history
        for i in 0..self.history.len() {
            fg.print(
                vram,
                tileset,
                b"                              ",
                &Vector2D::new(0, SEPERATOR_Y - 2 - (i as u16)),
            );
            fg.print(
                vram,
                tileset,
                &self.history[i],
                &Vector2D::new(0, SEPERATOR_Y - 2 - (i as u16)),
            );
        }

        // draw current user input
        fg.print(
            vram,
            tileset,
            b"                              ",
            &Vector2D::new(0, SEPERATOR_Y - 1),
        );
        fg.print(
            vram,
            tileset,
            &self.user_input,
            &Vector2D::new(0, SEPERATOR_Y - 1),
        );
    }

    fn handle_command(&mut self, command: Vec<u8>) {
        // have to covert to a string literal first because it just be like that sometime
        match String::from_utf8(command.clone()).unwrap().as_str() {
            "cmd_enter" => {
                self.history.rotate_right(2);
                // the string the user answered
                self.history[1] = self.user_input.clone();
                // the answer
                let equation_result = self.engine.eval(self.user_input.clone());
                match equation_result {
                    Ok(answer) => {
                        self.history[0] = answer.to_string().as_bytes().to_vec();
                    }
                    Err(e) => {
                        self.history[0] = e.as_bytes().to_vec();
                    }
                }
                self.history[0].insert(0, b'>');
                self.user_input.clear();
            }
            "cmd_set_angle_radians" => {
                self.engine.use_radians = true;
            }
            "cmd_set_angle_degrees" => {
                self.engine.use_radians = false;
            }
            _ => {
                for c in &command {
                    self.user_input.push(*c);
                }
            }
        }
    }
}
