use base64::*;
use std::io::*;

//fails when something goes wrong in conversion
fn response_as_bool(response: String) -> Option<bool> {
    if let Ok(decoded) = decode(response.trim()) {
        if let Ok(converted) = String::from_utf8(decoded) {
            if let Ok(value) = converted.parse::<bool>() {
                return Some(value);
            }
        }
    }
    None
}

pub fn set_local_godmode_enabled(new_state: bool) -> bool {
    println!("0 {}", encode(&new_state.to_string()));

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn get_local_godmode_enabled() -> bool {
    println!("1");

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn local_godmode_usable() -> bool {
    println!("2");

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn spawn_local_img<S: AsRef<str>>(full_path: S) {
    println!("3 {}", base64::encode(full_path.as_ref()));
}

pub fn set_local_flight_enabled(enable: bool) {
    println!("4 {}", encode(&enable.to_string()));
}

pub fn get_local_flight_enabled() -> bool {
    println!("5");

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn set_local_flight_persistent_enabled(enable: bool) -> bool{
    println!("6 {}", encode(&enable.to_string()));

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn get_local_flight_persistent_enabled() -> bool {
    println!("7");

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn set_local_player_invisible(enable: bool) -> bool{
    println!("8 {}", encode(&enable.to_string()));

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn get_local_player_invisible() -> bool {
    println!("9");

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn get_current_player_list() -> Vec<(String, usize)> {
    println!("A");

    let mut players: Vec<(String, usize)> = Vec::new();

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();

    let mut parts = result.split_ascii_whitespace();

    while let Some(name64) = parts.next() {
        let name_bytes = decode(name64).unwrap();
        let name_slice: &[u16] = unsafe { std::slice::from_raw_parts(std::mem::transmute(name_bytes.as_ptr()), name_bytes.len() / 2) };
        let name = String::from_utf16_lossy(name_slice);
        let ptr_vec = decode(parts.next().unwrap()).unwrap();
        let mut ptr_bytes: [u8; 8] = [0; 8];

        for i in 0..8 {
            ptr_bytes[i] = ptr_vec[i];
        }

        let ptr = usize::from_le_bytes(ptr_bytes);

        players.push((name, ptr));
    }

    players
}

pub fn get_player_exists(player_ptr: usize) -> bool {
    println!("B {}", encode(&player_ptr.to_le_bytes()));

    let mut result = String::new();
    if let Ok(_) = stdin().read_line(&mut result) {
        return response_as_bool(result).unwrap();
    }
    false
}

pub fn teleport_to_player(player_ptr: usize) {
    println!("C {}", encode(&player_ptr.to_le_bytes()));
}

pub fn spawn_token_box<S: AsRef<str>>(msg: S, amount: i32) {
    println!("D {} {}", encode(msg.as_ref()), encode(&amount.to_le_bytes()));
}

pub fn change_player_perm<A: AsRef<[u8]>>(is_local: u8, kind: i32, perm: i32, value: &A) -> bool {
    println!("E {} {} {} {}", encode(&[is_local]), encode(&kind.to_le_bytes()), encode(&perm.to_le_bytes()), encode(value));

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn spawn_object_for_player<S: AsRef<str>>(player_ptr: usize, prefab_name: S) {
    println!("F {} {}", encode(&player_ptr.to_le_bytes()), encode(prefab_name.as_ref()));
}

pub fn get_perm_bool_value(manager_kind: i32, perm_id: i32, is_local: u8) -> bool {
    println!("G {} {} {}", encode(&manager_kind.to_le_bytes()), encode(&perm_id.to_le_bytes()), encode(&[is_local]));

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn set_apply_skin<S: AsRef<str>>(enable: bool, prefab_name: S, guid: S, player_ptr: usize) {
    println!("H {} {} {} {}", encode(prefab_name.as_ref()), encode(guid.as_ref()), encode(&[enable as u8]), encode(&player_ptr.to_le_bytes()));
}

pub fn get_applied_skin<S: AsRef<str>>(prefab_name: S, player_ptr: usize) -> String {
    println!("I {} {}", encode(prefab_name.as_ref()), encode(&player_ptr.to_le_bytes()));

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();

    String::from_utf8_lossy(&decode(result.trim()).unwrap()).to_string()
}

pub fn exterminate_player(player_ptr: usize) {
    println!("J {}", encode(&player_ptr.to_le_bytes()));
}

pub fn rapid_fire(toggle: bool) -> bool {
    if toggle {
        println!("K");
    }
    else {
        println!("M");
    }

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn inf_ammo(toggle: bool) -> bool {
    if toggle {
        println!("L");
    }
    else {
        println!("N");
    }

    let mut result = String::new();
    stdin().read_line(&mut result).unwrap();
    response_as_bool(result).unwrap()
}

pub fn online_play_sound<S: AsRef<str>>(file_path: S) {
    println!("O {}", encode(file_path.as_ref()));
}
