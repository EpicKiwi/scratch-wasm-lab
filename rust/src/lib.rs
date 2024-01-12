use cstr::cstr;
use std::{ffi::{CStr, CString}, ptr};

const NAMES: [&CStr; 69] = [
    cstr!("Abricot"),
    cstr!("Airelle"),
    cstr!("Alkékenge"),
    cstr!("Amande"),
    cstr!("Amélanche"),
    cstr!("Ananas"),
    cstr!("Arbouse"),
    cstr!("Argouse"),
    cstr!("Asimine"),
    cstr!("Avocat"),
    cstr!("Banane"),
    cstr!("Bergamote"),
    cstr!("Orange amère"),
    cstr!("Bleuet"),
    cstr!("Canneberge"),
    cstr!("Cantaloup"),
    cstr!("Cassis"),
    cstr!("Cerise"),
    cstr!("Châtaigne"),
    cstr!("Citron"),
    cstr!("Clémentine"),
    cstr!("Coing"),
    cstr!("Cornouiller du Canada"),
    cstr!("Cynorrhodon"),
    cstr!("Datte"),
    cstr!("Épine-vinette"),
    cstr!("Feijoa"),
    cstr!("Figue"),
    cstr!("Figue de barbarie"),
    cstr!("Fraise"),
    cstr!("Framboise"),
    cstr!("Grenade"),
    cstr!("Griotte"),
    cstr!("Groseille"),
    cstr!("Jujube"),
    cstr!("Kaki"),
    cstr!("Kiwaï"),
    cstr!("Kiwi"),
    cstr!("Lime"),
    cstr!("Mandarine"),
    cstr!("Marron"),
    cstr!("Melon"),
    cstr!("Mûre"),
    cstr!("Myrtille"),
    cstr!("Nèfle"),
    cstr!("Nèfle du Japon"),
    cstr!("Noisette"),
    cstr!("Noix"),
    cstr!("Olive"),
    cstr!("Orange"),
    cstr!("Pamplemousse"),
    cstr!("Pastèque"),
    cstr!("Pêche"),
    cstr!("Brugnon"),
    cstr!("nectarine"),
    cstr!("pavie"),
    cstr!("Coqueret du Pérou"),
    cstr!("Pistache"),
    cstr!("Plaquebière ou chicouté"),
    cstr!("Poire"),
    cstr!("Pomme"),
    cstr!("Pomélos"),
    cstr!("Prunes"),
    cstr!("Pruneaux"),
    cstr!("Mirabelle"),
    cstr!("Quetsche"),
    cstr!("Reine-claude"),
    cstr!("prune citron"),
    cstr!("Raisin"),
];

static mut CURRENT_NAME: usize = 0;

#[no_mangle]
pub fn reset() {
    unsafe {
        CURRENT_NAME = 0;
    }
}

#[no_mangle]
#[export_name = "nextFruit"]
pub fn next_fruit() -> *const i8 {
    let next = unsafe {
        CURRENT_NAME += 1;
        CURRENT_NAME
    };

    if next < NAMES.len() {
        return NAMES[next].as_ptr();
    } else {
        return ptr::null();
    }
}