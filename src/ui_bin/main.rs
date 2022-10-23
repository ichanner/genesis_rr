#![windows_subsystem = "windows"]
extern crate piston_window;
extern crate base64;

use piston_window::*;

mod controls;
mod ipc;

fn main() {
    const WIDTH: f64 = 400.0;
    const HEIGHT: f64 = 640.0;
    const ITEMS_ON_SCREEN: f64 = 9.0;
    const MARGIN: f64 = 10.0;
    const ITEM_HEIGHT: f64 = (HEIGHT - (MARGIN * ITEMS_ON_SCREEN)) / (ITEMS_ON_SCREEN + 2.9);
    const TITLE_HEIGHT: f64 = ITEM_HEIGHT * 1.4;
    const TITLE_TEXT_SIZE: f64 = (TITLE_HEIGHT * 1.8) / 3.0;
    const NAVBAR_HEIGHT: f64 = ITEM_HEIGHT * 1.5;
    const NAVBAR_START: f64 = MARGIN + TITLE_HEIGHT + (ITEM_HEIGHT * ITEMS_ON_SCREEN) + (MARGIN * ITEMS_ON_SCREEN);
    const NAVBAR_BUTTON_WIDTH: f64 = (WIDTH - (2.0 * MARGIN)) / 4.0;
    const LINE_RAD: f64 = 1.0;

    let mut window: PistonWindow = WindowSettings::new("", [WIDTH, HEIGHT])
        .decorated(false)
        .build()
        .unwrap();

    window.set_position([0; 2]);
    let mut menu = MenuState::new(&mut window);
    let mut frame_stack: Vec<MenuFrame> = Vec::new();
    let mut current_frame = MenuFrame::default();
    
    let back_button_rect = [MARGIN, NAVBAR_START, NAVBAR_BUTTON_WIDTH, NAVBAR_HEIGHT];
    let mut back_button_hover = false;

    let previous_page_rect = [MARGIN + (NAVBAR_BUTTON_WIDTH * 2.0), NAVBAR_START, NAVBAR_BUTTON_WIDTH, NAVBAR_HEIGHT];
    let mut previous_page_hover = false;
    let previous_page_enabled = |page_num: u32| {
        page_num > 0
    };

    let next_page_rect = [MARGIN + (NAVBAR_BUTTON_WIDTH * 3.0), NAVBAR_START, NAVBAR_BUTTON_WIDTH, NAVBAR_HEIGHT];
    let mut next_page_hover = false;
    let next_page_enabled = |frame: &MenuFrame, page: u32| {
        let last_item = frame.items.len();
        last_item as u32 > (page + 1) * ITEMS_ON_SCREEN as u32
    };
    
    let mut item_update_counter = 0;

    //message loop
    while let Some(event) = window.next() {
        if let Some(_) = event.update_args() {
            if item_update_counter != 100 {
                item_update_counter += 1;
            }
            else {
                item_update_counter = 0;
                let mut i = 0;
                while i != current_frame.items.len() {
                    if current_frame.items[i].update() {
                        i += 1;
                    }
                    else {
                        current_frame.items.remove(i);
                        if ((current_frame.items.len() as f64 / ITEMS_ON_SCREEN).ceil() as u32) < menu.current_page {
                            menu.current_page -= 1;
                        }
                    }
                }
            }
        }
        if let Some(new_pos) = event.mouse_cursor_args() { 
            //assuming sizes will be positive
            fn collides(point: [f64; 2], rect: [f64; 4]) -> bool {
                point[0] >= rect[0] &&
                point[1] >= rect[1] &&
                point[0] <= (rect[0] + rect[2]) &&
                point[1] <= (rect[1] + rect[3])
            }

            let first_item = menu.current_page as usize * ITEMS_ON_SCREEN as usize;
            let mut counter = 0;
            for item in &mut current_frame.items[first_item..] {
                if item.hide() {
                    continue;
                }
                if counter >= ITEMS_ON_SCREEN as i32 {
                    break;
                }
                counter += 1;
                if let Some(area) = item.drawing_area() {
                    item.set_highlighted(collides(new_pos, area));
                }
            }

            if frame_stack.len() > 0 {
                back_button_hover = collides(new_pos, back_button_rect);
            }
            else {
                back_button_hover = false;
            }
            if previous_page_enabled(menu.current_page) {
                previous_page_hover = collides(new_pos, previous_page_rect);
            }
            else {
                previous_page_hover = false;
            }
            if next_page_enabled(&current_frame, menu.current_page) {
                next_page_hover = collides(new_pos, next_page_rect);
            }
            else {
                next_page_hover = false;
            }
        }
        if let Some(pressed_button) = event.button_args() {
            if let Button::Mouse(mouse_button) = pressed_button.button {
                if mouse_button == MouseButton::Left && pressed_button.state == ButtonState::Release {
                    if back_button_hover && frame_stack.len() > 0 {
                        menu.current_page = 0;
                        current_frame = frame_stack.remove(frame_stack.len() - 1);
                    }
                    else if previous_page_hover && previous_page_enabled(menu.current_page) {
                        menu.current_page -= 1;
                    }
                    else if next_page_hover && next_page_enabled(&current_frame, menu.current_page) {
                        menu.current_page += 1;
                    }
                    else {
                        for item in &mut current_frame.items {
                            if item.highlighted() {
                                if let Some(next_frame) = item.on_click_ex() {
                                    frame_stack.push(current_frame);
                                    current_frame = next_frame;
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }

        window.draw_2d(&event, |context, graphics, device| {
            clear(menu.theme.background_color, graphics);

            //draw frame title
            let title_box_sizes = [MARGIN, MARGIN, WIDTH - (2.0 * MARGIN), TITLE_HEIGHT];
            rectangle(menu.theme.title_box_color, title_box_sizes, context.transform, graphics);

            let title_transform = context.transform.trans_pos(
                [MARGIN * 2.0,
                ((title_box_sizes[3] / 2.0) + MARGIN) + (TITLE_TEXT_SIZE / 2.5)]);

            text(
                menu.theme.title_text_color,
                TITLE_TEXT_SIZE as u32,
                &current_frame.title,
                &mut menu.glyph_cache,
                title_transform,
                graphics,
            ).unwrap();

            //page number
            let last_page_display = (current_frame.items.len() as f64 / ITEMS_ON_SCREEN).ceil() as u32;
            text(
                menu.theme.title_text_color,
                TITLE_TEXT_SIZE as u32,
                &((menu.current_page + 1).to_string() + "/" + &last_page_display.to_string()),
                &mut menu.glyph_cache,
                context.transform.trans_pos([MARGIN + NAVBAR_BUTTON_WIDTH, HEIGHT - MARGIN]),
                graphics,
            ).unwrap();
                
            //draw the frame's items, with paging if the item count exceeds ITEMS_ON_SCREEN
            let mut draw_area = [MARGIN, TITLE_HEIGHT + MARGIN, WIDTH - (2.0 * MARGIN), ITEM_HEIGHT];
            let mut counter = 0;
            let first_item = menu.current_page as usize * ITEMS_ON_SCREEN as usize;
            for item in &mut current_frame.items[first_item..] {
                if item.hide() {
                    continue;
                }
                if counter >= ITEMS_ON_SCREEN as i32 {
                    break;
                }
                counter += 1;
                item.draw(draw_area, &mut menu, &context, graphics);

                if item.highlighted() {
                    rectangle(menu.theme.item_highlighted_color,
                        draw_area,
                        context.transform,
                        graphics
                    )
                }
                    
                let y_coord = draw_area[1] + draw_area[3] + (MARGIN / 2.0);
                line(
                    menu.theme.item_line_color, 
                    LINE_RAD, 
                    [MARGIN, y_coord, //point 1
                    WIDTH - MARGIN, y_coord], //point 2
                    context.transform,
                    graphics
                );

                draw_area[1] += ITEM_HEIGHT + MARGIN;
            }

            //navbar buttons
            if back_button_hover {
                rectangle(menu.theme.item_highlighted_color,
                    back_button_rect,
                    context.transform,
                    graphics
                )
            }

            if previous_page_hover {
                rectangle(menu.theme.item_highlighted_color,
                    previous_page_rect,
                    context.transform,
                    graphics
                )
            }

            if next_page_hover {
                rectangle(menu.theme.item_highlighted_color,
                    next_page_rect,
                    context.transform,
                    graphics
                )
            }
            
            menu.glyph_cache.factory.encoder.flush(device);
        });
    }
}

struct MenuState {
    theme: MenuTheme,
    glyph_cache: Glyphs,
    current_page: u32,
}

impl MenuState {
    fn new(window: &mut PistonWindow) -> MenuState {
        MenuState {
            theme: MenuTheme::default(),
            glyph_cache: Glyphs::from_bytes(
                include_bytes!("assets/pakenham rg.ttf"), 
                window.create_texture_context(), 
                TextureSettings::new()
            ).unwrap(),
            current_page: 0,
        }
    }
}

struct MenuTheme {
    background_color: types::Color,

    title_box_color: types::Color,
    title_text_color: types::Color,

    item_line_color: types::Color,
    item_text_color: types::Color,
    item_highlighted_color: types::Color,

    switch_selection_color: types::Color,
}

impl MenuTheme {
    fn default() -> MenuTheme {
        MenuTheme {
            background_color: [0.2, 0.2, 0.2, 1.0],

            title_box_color: [0.5, 0.5, 0.5, 1.0],
            title_text_color: [197.0 / 255.0, 252.0 / 255.0, 58.0 / 255.0, 1.0], //#c5fc3a

            item_line_color: [1.0, 1.0, 1.0, 0.1],
            item_text_color: [252.0 / 255.0, 210.0 / 255.0, 58.0 / 255.0, 1.0], //#fcd23a
            item_highlighted_color: [0.7, 1.0, 0.7, 0.05],

            switch_selection_color: [245.0 / 255.0, 240.0 / 255.0, 58.0 / 255.0, 1.0]
        }
    }
}

pub struct MenuFrame {
    title: String,
    items: Vec<Box<dyn MenuItem>>,
}

impl<'ui> MenuFrame {
    fn default() -> MenuFrame {
        let default_items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(controls::Category::new("Player mods", MenuFrame::player_mods, || {true}, || {false})),
            Box::new(controls::OnlinePlayersCategory::new()),
            Box::new(controls::Category::new("Weapon mods", MenuFrame::weapon_mods, || {true}, || {false})),
            Box::new(controls::Category::new("Spawn local images", MenuFrame::local_img_spawner, || {true}, || {false})),
            Box::new(controls::Category::new("This room", || {MenuFrame::this_room()}, || {true}, || {false})),
            //Box::new(controls::Button::new("test", || {println!("")}, || {true}, || {false})),

        ];

        MenuFrame {
            title: "Genesis RR by 567".to_owned(),
            items: default_items,
        }
    }

    fn player_mods() -> MenuFrame {
        MenuFrame {
            title: "Player mods".to_owned(),
            items: vec![
                //Box::new(controls::Category::new("Room permissions", || {MenuFrame::permissions(true)}, || {true}, || {false})),
                //Box::new(controls::Category::new("Room roles", || {MenuFrame::roles(true)}, || {true}, || {false})),
                Box::new(controls::Category::new("Spawn tokens (visual)", || {MenuFrame::token_box_spawner()}, || {true}, || {false})),
                //Box::new(controls::Toggle::new("Flying", |state| {ipc::set_local_flight_persistent_enabled(state)}, || {(true, None)}, || {false}, ipc::get_local_flight_persistent_enabled())),
                Box::new(controls::Toggle::new("God mode", |state| {ipc::set_local_godmode_enabled(state)}, || {(true, None)}, || {!ipc::local_godmode_usable()}, ipc::get_local_godmode_enabled())),
                Box::new(controls::Toggle::new("Ghost mode", |state| {ipc::set_local_player_invisible(state)}, || {(true, Some(ipc::get_local_player_invisible()))}, || {false}, ipc::get_local_player_invisible())),
            ],
        }
    }

    fn weapon_mods() -> MenuFrame {
        MenuFrame {
            title: "Weapon mods".to_owned(),
            items: vec![
                Box::new(controls::Toggle::new("Rapid fire", |_| {ipc::rapid_fire(true)}, || {(true, None)}, || {false}, ipc::rapid_fire(false))),
                Box::new(controls::Toggle::new("Infinite ammo", |_| {ipc::inf_ammo(true)}, || {(true, None)}, || {false}, ipc::inf_ammo(false))),
            ],
        }
    }

    fn local_img_spawner() -> MenuFrame {
        use std::ffi::OsStr;
        let mut items: Vec<Box<dyn MenuItem>> = Vec::new();
        let paths = if let Ok(x) = std::fs::read_dir("./img") {x} 
            else { return MenuFrame { title: "no img folder".to_owned(),items: items,}};

        for path in paths {
            let tmp = if let Ok(x) = path {x.path()} else {continue;};
            if let Some(extension) = tmp.extension() {
                if extension == OsStr::new("jpg") || extension == OsStr::new("jpeg") {
                    items.push(Box::new(controls::SpawnImageButton::new(&tmp)));
                }
            }
        }

        MenuFrame {
            title: "Spawn local images".to_owned(),
            items: items,
        }
    }

    fn this_room() -> MenuFrame {
        MenuFrame {
            title: "This room".to_owned(),
            items: vec![
                Box::new(controls::Category::new("Edit permissions", || {
                    MenuFrame::permissions(false)
                }, || {true}, || {false})),
                Box::new(controls::Category::new("Edit roles", || {
                    MenuFrame::roles(false)
                }, || {true}, || {false})),
                Box::new(controls::Category::new("Play sound", || {
                    use std::ffi::OsStr;
                    let mut items: Vec<Box<dyn MenuItem>> = Vec::new();
                    let paths = if let Ok(x) = std::fs::read_dir("./wav") {x} 
                        else { return MenuFrame { title: "no wav folder".to_owned(),items: items,}};

                    for path in paths {
                        let tmp = if let Ok(x) = path {x.path()} else {continue;};
                        if let Some(extension) = tmp.extension() {
                            if extension == OsStr::new("wav") {
                                if let Ok(abs_path) = std::fs::canonicalize(tmp) {
                                    let full_path = abs_path.to_string_lossy().to_string();
                                    items.push(Box::new(controls::Button::new(&abs_path.file_name().unwrap().to_string_lossy().to_string(), 
                                        move || {ipc::online_play_sound(&full_path)}, || {true}, || {false})));
                                }
                            }
                        }           
                    }

                    MenuFrame {
                        title: "Play sound".to_owned(),
                        items: items,
                    }
                }, || {true}, || {false})),
            ],
        }
    }

    fn permissions(local: bool) -> MenuFrame {
        let title = if local {"Room permissions".to_owned()} else {"Edit permissions".to_owned()};

        MenuFrame {
            title: title,
            items: vec![
                Box::new(controls::Toggle::new("Can view gadgets", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x5, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x5, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x5, local as u8)
                )),
                // Box::new(controls::Toggle::new("Can use maker pen", 
                //     move |state| { 
                //         if state {ipc::change_player_perm(local as u8, 0, 0xD, &[0x08, 0x01, 0x10, 0x01]) && state} 
                //         else {ipc::change_player_perm(local as u8, 0, 0xD, &[0x08, 0x01]) && state}
                //     }, 
                //     || {(true, None)}, 
                //     || {false}, 
                //     ipc::get_perm_bool_value(0, 0xD, local as u8)
                // )),
                // Box::new(controls::Toggle::new("Can use 'delete all' button", 
                //     move |state| { 
                //         if state {ipc::change_player_perm(local as u8, 0, 0xE, &[0x08, 0x01, 0x10, 0x01]) && state} 
                //         else {ipc::change_player_perm(local as u8, 0, 0xE, &[0x08, 0x01]) && state}
                //     }, 
                //     || {(true, None)}, 
                //     || {false}, 
                //     ipc::get_perm_bool_value(0, 0xE, local as u8)
                // )),
                // Box::new(controls::Toggle::new("Can save inventions", 
                //     move |state| { 
                //         if state {ipc::change_player_perm(local as u8, 0, 0xF, &[0x08, 0x01, 0x10, 0x01]) && state} 
                //         else {ipc::change_player_perm(local as u8, 0, 0xF, &[0x08, 0x01]) && state}
                //     }, 
                //     || {(true, None)}, 
                //     || {false}, 
                //     ipc::get_perm_bool_value(0, 0xF, local as u8)
                // )),
                Box::new(controls::Toggle::new("Can invite", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x4, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x4, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x4, local as u8)
                )),
                Box::new(controls::Toggle::new("Can talk", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x6, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x6, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x6, local as u8)
                )),
                Box::new(controls::Toggle::new("Disable mic auto mute", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x10, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x10, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x10, local as u8)
                )),
                Box::new(controls::Toggle::new("Can use share camera", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x12, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x12, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x12, local as u8)
                )),
                Box::new(controls::Toggle::new("Can print photos", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x7, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x7, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x7, local as u8)
                )),
                Box::new(controls::Toggle::new("Can revive self", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x9, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x9, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x9, local as u8)
                )),
                Box::new(controls::Toggle::new("Can start games", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x8, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x8, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x8, local as u8)
                )),
                Box::new(controls::Toggle::new("Can end games early", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0x11, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0x11, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0x11, local as u8)
                )),
                Box::new(controls::Toggle::new("Can change gamemode", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 0, 0xC, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 0, 0xC, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(0, 0xC, local as u8)
                )),
            ],
        }
    }

    fn roles(local: bool) -> MenuFrame {
        MenuFrame {
            title: if local {"Room roles".to_owned()} else {"Edit roles".to_owned()},
            items: vec![
                Box::new(controls::Toggle::new("Can move", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 1, 4, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 1, 4, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 4, local as u8)
                )),
                Box::new(controls::Toggle::new("Can fly", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 1, 0x8, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 1, 0x8, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 8, local as u8)
                )),
                Box::new(controls::Toggle::new("Can switch teams", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 1, 0xd, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 1, 0xd, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 0xd, local as u8)
                )),
                Box::new(controls::Toggle::new("Hide name", 
                    move |state| { 
                        if state {ipc::change_player_perm(local as u8, 1, 20, &[0x08, 0x01, 0x10, 0x01]) && state} 
                        else {ipc::change_player_perm(local as u8, 1, 20, &[0x08, 0x01]) && state}
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 20, local as u8)
                )),
                Box::new(controls::Toggle::new("Can sprint", 
                    move |state| { 
                        if state {
                            ipc::change_player_perm(local as u8, 1, 0x16, &[0x08, 0x01, 0x10, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x19, &[0x08, 0x01, 0x10, 0x01]) && state
                        } 
                        else {
                            ipc::change_player_perm(local as u8, 1, 0x16, &[0x08, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x19, &[0x08, 0x01]) && state
                        }
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 0x16, local as u8) && ipc::get_perm_bool_value(1, 0x19, local as u8)
                )),
                Box::new(controls::Toggle::new("Can wallrun", 
                    move |state| { 
                        if state {
                            ipc::change_player_perm(local as u8, 1, 0x30, &[0x08, 0x01, 0x10, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x31, &[0x08, 0x01, 0x10, 0x01]) && state
                        } 
                        else {
                            ipc::change_player_perm(local as u8, 1, 0x30, &[0x08, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x31, &[0x08, 0x01]) && state
                        }
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 0x30, local as u8) && ipc::get_perm_bool_value(1, 0x31, local as u8)
                )),
                Box::new(controls::Toggle::new("Can get pushed", 
                    move |state| { 
                        if state {
                            ipc::change_player_perm(local as u8, 1, 0x32, &[0x08, 0x01, 0x10, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x33, &[0x08, 0x01, 0x10, 0x01]) && state
                        } 
                        else {
                            ipc::change_player_perm(local as u8, 1, 0x32, &[0x08, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x33, &[0x08, 0x01]) && state
                        }
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 0x32, local as u8) && ipc::get_perm_bool_value(1, 0x33, local as u8)
                )),
                Box::new(controls::Toggle::new("Can ram", 
                    move |state| { 
                        if state {
                            ipc::change_player_perm(local as u8, 1, 0x34, &[0x08, 0x01, 0x10, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x35, &[0x08, 0x01, 0x10, 0x01]) && state
                        } 
                        else {
                            ipc::change_player_perm(local as u8, 1, 0x34, &[0x08, 0x01]) &&
                            ipc::change_player_perm(local as u8, 1, 0x35, &[0x08, 0x01]) && state
                        }
                    }, 
                    || {(true, None)}, 
                    || {false}, 
                    ipc::get_perm_bool_value(1, 0x34, local as u8) && ipc::get_perm_bool_value(1, 0x35, local as u8)
                )),
            ],
        }
    }

    fn token_box_spawner() -> MenuFrame {
        MenuFrame {
            title: "Spawn tokens (visual)".to_owned(),
            items: vec![
                Box::new(controls::Button::new("1 token", || {ipc::spawn_token_box("from: 567", 1)}, || {true}, || {false})),
                Box::new(controls::Button::new("10 tokens", || {ipc::spawn_token_box("from: 567", 10)}, || {true}, || {false})),
                Box::new(controls::Button::new("40 tokens", || {ipc::spawn_token_box("from: 567", 50)}, || {true}, || {false})),
                Box::new(controls::Button::new("500 tokens", || {ipc::spawn_token_box("from: 567", 500)}, || {true}, || {false})),
                Box::new(controls::Button::new("1 000 tokens", || {ipc::spawn_token_box("from: 567", 1000)}, || {true}, || {false})),
                Box::new(controls::Button::new("10 000 tokens", || {ipc::spawn_token_box("from: 567", 10000)}, || {true}, || {false})),
                Box::new(controls::Button::new("100 000 tokens", || {ipc::spawn_token_box("from: 567", 100000)}, || {true}, || {false})),
                Box::new(controls::Button::new("1 000 000 tokens", || {ipc::spawn_token_box("from: 567", 1000000)}, || {true}, || {false})),
                Box::new(controls::Button::new("MAXIMUM tokens", || {ipc::spawn_token_box("from: 567", 0x7fffffff)}, || {true}, || {false})),
                Box::new(controls::Button::new("funni tokens", || {ipc::spawn_token_box("from: 567", 69696969)}, || {true}, || {false})),
                Box::new(controls::Button::new("funni tokens 2", || {ipc::spawn_token_box("from: 567", 6942069)}, || {true}, || {false})),
                Box::new(controls::Button::new("funni tokens 3", || {ipc::spawn_token_box("from: 567", 420420420)}, || {true}, || {false})),
                Box::new(controls::Button::new("Max for proper animation", || {ipc::spawn_token_box("from: 567", 35000)}, || {true}, || {false})),
            ],
        }
    }
}



trait MenuItem {
    fn update(&mut self) -> bool {
        true
    }
    fn hide(&self) -> bool {
        false
    }
    fn on_click(&mut self) {}
    fn on_click_ex<'ui>(&'ui mut self) -> Option<MenuFrame> {
        self.on_click();
        None
    }

    fn draw(&mut self, area: [f64;4], state: &mut MenuState, c: &Context, g: &mut G2d);
    fn drawing_area(&self) -> Option<[f64;4]>;
    fn text(&self) -> std::borrow::Cow<str>;

    fn highlighted(&self) -> bool;
    fn set_highlighted(&mut self, value: bool);
}