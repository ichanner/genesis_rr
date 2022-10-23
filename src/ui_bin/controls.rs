use piston_window::*;
use crate::*;
use std::borrow::Cow;

pub struct Button<F: FnMut(), U: FnMut() -> bool, H: Fn() -> bool> {
    last_draw_area: [f64; 4],
    press_action: F,
    update_action: U,
    hide_action: H,
    name: String,
    highlighted: bool,
}

impl<F: FnMut(), U: FnMut() -> bool, H: Fn() -> bool> Button<F, U, H> {
    pub fn new(name: &str, press_action: F, update_action: U, hide_action: H) -> Button<F, U, H> {
        
        Button {
            last_draw_area: [0.0; 4],
            press_action: press_action,
            update_action: update_action,
            hide_action: hide_action,
            name: name.to_owned(),
            highlighted: false,
        }
    }
}

impl<F: FnMut(), U: FnMut() -> bool, H: Fn() -> bool> MenuItem for Button<F, U, H> {
    fn draw(&mut self, area: [f64; 4], state: &mut MenuState, c: &Context, g: &mut G2d) {
        self.last_draw_area = area;

        let char_count = self.text().chars().count() as u32;
        let mut text_size = (area[2] * 1.84) / char_count as f64;
        if text_size > area[3] * 0.6 {
            text_size = area[3] * 0.6;
        }
        let align_left_transform = c.transform.trans_pos(
            [area[0] + 12.0,
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]);

        text(
            state.theme.item_text_color,
            text_size as u32,
            self.text().as_ref(),
            &mut state.glyph_cache,
            align_left_transform,
            g,
        ).unwrap();
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn drawing_area(&self) -> Option<[f64; 4]> {
        if !self.hide() {
            return Some(self.last_draw_area);
        }
        None
    }

    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, value: bool) {
        self.highlighted = value;
    }

    fn on_click(&mut self) {
        (self.press_action)();
    }

    fn update(&mut self) -> bool {
        (self.update_action)()
    }

    fn hide(&self) -> bool {
        (self.hide_action)()
    }
}

pub struct Toggle<F: FnMut(bool) -> bool, U: FnMut() -> (bool, Option<bool>), H: Fn() -> bool> {
    last_draw_area: [f64; 4],
    toggle_action: F,
    update_action: U,
    hide_action: H,
    name: String,
    highlighted: bool,
    current_state: bool,
}

impl<F: FnMut(bool) -> bool, U: FnMut() -> (bool, Option<bool>), H: Fn() -> bool> Toggle<F, U, H> {
    pub fn new(name: &str, toggle_action: F, update_action: U, hide_action: H, start_state: bool) -> Toggle<F, U, H> {
        
        Toggle {
            last_draw_area: [0.0; 4],
            toggle_action: toggle_action,
            update_action: update_action,
            hide_action: hide_action,
            name: name.to_owned(),
            highlighted: false,
            current_state: start_state,
        }
    }
}

impl<F: FnMut(bool) -> bool, U: FnMut() -> (bool, Option<bool>), H: Fn() -> bool> MenuItem for Toggle<F, U, H> {
    fn draw(&mut self, area: [f64; 4], state: &mut MenuState, c: &Context, g: &mut G2d) {
        self.last_draw_area = area;

        //text
        let char_count = self.text().chars().count() as u32;
        let mut text_size = (area[2] * 1.84) / char_count as f64;
        if text_size > area[3] * 0.6 {
            text_size = area[3] * 0.6;
        }
        let align_left_transform = c.transform.trans_pos(
            [area[0] + 12.0,
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]);

        text(
            state.theme.item_text_color,
            text_size as u32,
            self.text().as_ref(),
            &mut state.glyph_cache,
            align_left_transform,
            g,
        ).unwrap();

        let on_off_text_transfrom = c.transform.trans_pos(
            [area[0] + (area[2] * 0.9),
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]
        );
        if self.current_state {
            text(
                [0.0, 1.0, 0.0, 1.0],
                text_size as u32,
                "On",
                &mut state.glyph_cache,
                on_off_text_transfrom,
                g,
            ).unwrap();
        }
        else {
            text(
                [1.0, 0.0, 0.0, 1.0],
                text_size as u32,
                "Off",
                &mut state.glyph_cache,
                on_off_text_transfrom,
                g,
            ).unwrap();
        }
        
    }

    fn drawing_area(&self) -> Option<[f64; 4]> {
        if !self.hide() {
            return Some(self.last_draw_area);
        }
        None
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, value: bool) {
        self.highlighted = value;
    }

    fn on_click(&mut self) {
        self.current_state = (self.toggle_action)(!self.current_state);
    }

    fn update(&mut self) -> bool {
        let result = (self.update_action)();
        if let Some(new_state) = result.1 {
            self.current_state = new_state;
        }
        result.0
    }

    fn hide(&self) -> bool {
        (self.hide_action)()
    }
}

pub struct Switch<T: std::fmt::Display, F: FnMut(&T), U: FnMut() -> bool, H: Fn() -> bool> {
    last_draw_area: [f64; 4],
    name: String,
    highlighted: bool,
    selectable_options: Vec<T>,
    selected_index: usize,
    switch_action: F,
    update_action: U,
    hide_action: H,
}

impl<T: std::fmt::Display, F: FnMut(&T), U: FnMut() -> bool, H: Fn() -> bool> Switch<T,F,U,H> {
    pub fn new<S: AsRef<str>>(name: S, options: Vec<T>, on_switch: F, on_update: U, on_hide: H, initial_index: usize) -> Switch<T,F,U,H> {
        Switch {
            last_draw_area: [0.0; 4],
            name: name.as_ref().to_owned(),
            highlighted: false,
            selectable_options: options,
            selected_index: initial_index,
            switch_action: on_switch,
            update_action: on_update,
            hide_action: on_hide,
        }
    }
}

impl<T: std::fmt::Display, F: FnMut(&T), U: FnMut() -> bool, H: Fn() -> bool> MenuItem for Switch<T,F,U,H> {
    fn draw(&mut self, area: [f64; 4], state: &mut MenuState, c: &Context, g: &mut G2d) {
        self.last_draw_area = area;
        let draw_text = format!("{}   <{}>", self.text(), self.selectable_options[self.selected_index]);

        //text
        let char_count = draw_text.chars().count() as u32;
        let mut text_size = (area[2] * 1.84) / char_count as f64;
        if text_size > area[3] * 0.6 {
            text_size = area[3] * 0.6;
        }
        let align_left_transform = c.transform.trans_pos(
            [area[0] + 12.0,
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]);

        text(
            state.theme.item_text_color,
            text_size as u32,
            &draw_text,
            &mut state.glyph_cache,
            align_left_transform,
            g,
        ).unwrap();
    }

    fn drawing_area(&self) -> Option<[f64;4]> {
        if !self.hide() {
            return Some(self.last_draw_area)
        }
        None
    }
    
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, value: bool) {
        self.highlighted = value;
    }

    fn on_click(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.selectable_options.len();
        (self.switch_action)(&self.selectable_options[self.selected_index]);
    }

    fn update(&mut self) -> bool {
        (self.update_action)()
    }

    fn hide(&self) -> bool {
        (self.hide_action)()
    }
}

pub struct Category<F: FnMut() -> MenuFrame, U: FnMut() -> bool, H: Fn() -> bool> {
    last_draw_area: [f64; 4],
    name: String,
    highlighted: bool,
    frame_generator: F,
    update_action: U,
    hide_action: H,
}

impl<F: FnMut() -> MenuFrame, U: FnMut() -> bool, H: Fn() -> bool> Category<F, U, H> {
    pub fn new(name: &str, frame_generator: F, update_action: U, hide_action: H) -> Category<F, U, H> {
        Category {
            last_draw_area: [0.0; 4],
            name: "> ".to_owned() + name,
            highlighted: false,
            frame_generator: frame_generator,
            update_action: update_action,
            hide_action: hide_action,
        }
    }
}

impl<F: FnMut() -> MenuFrame, U: FnMut() -> bool, H: Fn() -> bool> MenuItem for Category<F, U, H> {
    fn draw(&mut self, area: [f64; 4], state: &mut MenuState, c: &Context, g: &mut G2d) {
        self.last_draw_area = area;

        //text
        let char_count = self.text().chars().count() as u32;
        let mut text_size = (area[2] * 1.84) / char_count as f64;
        if text_size > area[3] * 0.6 {
            text_size = area[3] * 0.6;
        }
        let align_left_transform = c.transform.trans_pos(
            [area[0] + 12.0,
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]);

        text(
            state.theme.item_text_color,
            text_size as u32,
            self.text().as_ref(),
            &mut state.glyph_cache,
            align_left_transform,
            g,
        ).unwrap();
        
    }

    fn drawing_area(&self) -> Option<[f64; 4]> {
        if !self.hide() {
            return Some(self.last_draw_area);
        }
        None
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, value: bool) {
        self.highlighted = value;
    }

    fn on_click_ex(&mut self) -> Option<MenuFrame> {
        Some((self.frame_generator)())
    }

    fn update(&mut self) -> bool {
        (self.update_action)()
    }

    fn hide(&self) -> bool {
        (self.hide_action)()
    }
}

pub struct SpawnImageButton {
    last_draw_area: [f64; 4],
    name: String,
    full_path: String,
    highlighted: bool,
}


impl SpawnImageButton {
    pub fn new(img_path: &std::path::PathBuf) -> SpawnImageButton {
        let mut result = SpawnImageButton {
            last_draw_area: [0.0; 4],
            name: "failed to resolve".to_owned(),
            full_path: "".to_owned(),
            highlighted: false,
        };
        
        if let Ok(abs_path) = std::fs::canonicalize(img_path) {
            result.name = abs_path.file_name().unwrap().to_string_lossy().to_string();
            result.full_path = abs_path.to_string_lossy().to_string();
        }

        result
    }
}

impl MenuItem for SpawnImageButton {
    fn draw(&mut self, area: [f64; 4], state: &mut MenuState, c: &Context, g: &mut G2d) {
        self.last_draw_area = area;

        let char_count = self.text().chars().count() as u32;
        let mut text_size = (area[2] * 1.84) / char_count as f64;
        if text_size > area[3] * 0.6 {
            text_size = area[3] * 0.6;
        }
        let align_left_transform = c.transform.trans_pos(
            [area[0] + 12.0,
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]);

        text(
            state.theme.item_text_color,
            text_size as u32,
            self.text().as_ref(),
            &mut state.glyph_cache,
            align_left_transform,
            g,
        ).unwrap();
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn drawing_area(&self) -> Option<[f64; 4]> {
        if !self.hide() {
            return Some(self.last_draw_area);
        }
        None
    }

    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, value: bool) {
        self.highlighted = value;
    }

    fn on_click(&mut self) {
        ipc::spawn_local_img(&self.full_path);
    }
}

pub struct OnlinePlayerCategory {
    last_draw_area: [f64; 4],
    player_name: String,
    display_text: String,
    photon_player_ptr: usize,
    highlighted: bool,
}

impl OnlinePlayerCategory {
    pub fn new<S: AsRef<str>>(player_name: S, photon_player_ptr: usize) -> OnlinePlayerCategory {
        OnlinePlayerCategory {
            last_draw_area: [0.0; 4],
            player_name: player_name.as_ref().to_owned(),
            display_text: "> ".to_owned() + player_name.as_ref(),
            photon_player_ptr: photon_player_ptr,
            highlighted: false,
        }
    }

    fn skin_selector(player_ptr: usize) -> MenuFrame {
        use std::collections::HashMap;

        let raw_skin_file = include_str!("assets/skins.txt");
        let lines = raw_skin_file.lines();

        let mut prefabs: HashMap<String, HashMap<String, String>> = HashMap::new();

        for line in lines {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();

            match prefabs.get_mut(parts[0]) {
                Some(skin_map) => { skin_map.insert(parts[1].to_owned(), parts[2].to_owned()); },
                None => {
                    let mut skin_map = HashMap::new();
                    skin_map.insert(parts[1].to_owned(), parts[2].to_owned()); 
                    prefabs.insert(parts[0].to_owned(), skin_map);
                }
            }
        }

        let mut frame_items: Vec<Box<dyn MenuItem>> = Vec::with_capacity(prefabs.len());

        for (prefab_name, skins) in prefabs {
            let mut selection_items: Vec<controls::SkinSelectionItem> = Vec::with_capacity(skins.len());
            let mut initial_index: usize = 0;
            let mut skin_index: usize = 0;
            let applied_skin = ipc::get_applied_skin(&prefab_name, player_ptr);

            selection_items.push(controls::SkinSelectionItem {name: "default".to_owned(), guid: "none".to_owned(), prefab_name: prefab_name.clone()});
            for (skin_name, skin_guid) in skins {
                skin_index += 1;
                if skin_guid == applied_skin {
                    initial_index = skin_index;
                }
                selection_items.push(controls::SkinSelectionItem {name: skin_name, guid: skin_guid, prefab_name: prefab_name.clone()});
            }
            frame_items.push(Box::new(controls::Switch::new(prefab_name, selection_items, move |selected_item| {
                ipc::set_apply_skin(selected_item.guid != "none", &selected_item.prefab_name, &selected_item.guid, player_ptr);
            }, || {true}, || {false}, initial_index)));
        }
        

        MenuFrame {
            title: "Give skins".to_owned(),
            items: frame_items,
        }
    }
}

impl MenuItem for OnlinePlayerCategory {
    fn draw(&mut self, area: [f64; 4], state: &mut MenuState, c: &Context, g: &mut G2d) {
        self.last_draw_area = area;

        //text
        let char_count = self.text().chars().count() as u32;
        let mut text_size = (area[2] * 1.84) / char_count as f64;
        if text_size > area[3] * 0.6 {
            text_size = area[3] * 0.6;
        }
        let align_left_transform = c.transform.trans_pos(
            [area[0] + 12.0,
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]);

        text(
            state.theme.item_text_color,
            text_size as u32,
            self.text().as_ref(),
            &mut state.glyph_cache,
            align_left_transform,
            g,
        ).unwrap();
        
    }

    fn drawing_area(&self) -> Option<[f64; 4]> {
        if !self.hide() {
            return Some(self.last_draw_area);
        }
        None
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.display_text)
    }

    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, value: bool) {
        self.highlighted = value;
    }

    fn on_click_ex(&mut self) -> Option<MenuFrame> {
        use std::borrow::*;

        let ptr = self.photon_player_ptr;
        let items: Vec<Box<dyn MenuItem>> = vec![
            Box::new(Category::new("Spawn object at player", move || {
                let prefab_names_raw = String::from_utf8_lossy(include_bytes!("assets/prefab_names.txt"));
                let mut prefab_names = prefab_names_raw.split("\r\n");

                let mut buttons: Vec<Box<dyn MenuItem>> = Vec::new();

                while let Some(prefab_name) = prefab_names.next() {
                    let cow_prefab_name: Cow<String> = Cow::Owned(prefab_name.to_owned());
                    buttons.push(Box::new(Button::new(prefab_name, move || {ipc::spawn_object_for_player(ptr, cow_prefab_name.as_ref())}, || {true}, || {false})));
                }

                MenuFrame {
                    title: "Spawn object at player".to_owned(),
                    items: buttons,
                }
            }, || {true}, || {false})),
            Box::new(Category::new("Give skins", move || {OnlinePlayerCategory::skin_selector(ptr)}, || {true}, || {false})),
            Box::new(Button::new("Teleport to player", move || {ipc::teleport_to_player(ptr)}, || {true}, || {false})),
            Box::new(Button::new("Exterminate player", move || {ipc::exterminate_player(ptr)}, || {true}, || {false})),
        ];

        Some(MenuFrame {
            title: self.player_name.clone(),
            items: items,
        })
    }

    fn update(&mut self) -> bool {
        ipc::get_player_exists(self.photon_player_ptr)
    }

    fn hide(&self) -> bool {
        false
    }
}

pub struct OnlinePlayersCategory {
    last_draw_area: [f64; 4],
    highlighted: bool,
}

impl OnlinePlayersCategory {
    pub fn new() -> OnlinePlayersCategory {
        OnlinePlayersCategory {
            last_draw_area: [0.0; 4],
            highlighted: false,
        }
    }
}

impl MenuItem for OnlinePlayersCategory {
    fn on_click_ex(&mut self) -> Option<MenuFrame> {
        let players = ipc::get_current_player_list();
        let mut items: Vec<Box<dyn MenuItem>> = Vec::new();

        for player in players {
            items.push(Box::new(OnlinePlayerCategory::new(player.0, player.1)));
        }

        Some(MenuFrame {
            title: "Online players".to_owned(),
            items: items,
        })
    }

    fn draw(&mut self, area: [f64; 4], state: &mut MenuState, c: &Context, g: &mut G2d) {
        self.last_draw_area = area;

        //text
        let char_count = self.text().chars().count() as u32;
        let mut text_size = (area[2] * 1.84) / char_count as f64;
        if text_size > area[3] * 0.6 {
            text_size = area[3] * 0.6;
        }
        let align_left_transform = c.transform.trans_pos(
            [area[0] + 12.0,
            area[1] + (area[3] / 2.0) + (text_size / 2.5)]
        );

        text(
            state.theme.item_text_color,
            text_size as u32,
            self.text().as_ref(),
            &mut state.glyph_cache,
            align_left_transform,
            g,
        ).unwrap();
        
    }

    fn drawing_area(&self) -> Option<[f64; 4]> {
        if !self.hide() {
            return Some(self.last_draw_area);
        }
        None
    }

    fn text(&self) -> Cow<str> {
        Cow::Borrowed("> Online players")
    }

    fn highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, value: bool) {
        self.highlighted = value;
    }
}

pub struct SkinSelectionItem {
    pub name: String,
    pub guid: String,
    pub prefab_name: String,
}

impl std::fmt::Display for SkinSelectionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}
