//! Binary assets for use with `nih_plug_vizia`. These fonts first need to be registered using their
//! associated registration function.

use vizia::prelude::*;

// This module provides a re-export and simple font wrappers around the re-exported fonts.

use crate::editor::my_fonts;





pub const RED_ROSE: &str = "Red Rose";

pub fn register_red_rose_regular(cx: &mut Context) {
    cx.add_font_mem(my_fonts::RED_ROSE_REGULAR);
}
pub fn register_red_rose_bold(cx: &mut Context) {
    cx.add_font_mem(my_fonts::RED_ROSE_BOLD);
}
pub fn register_red_rose_light(cx: &mut Context) {
    cx.add_font_mem(my_fonts::RED_ROSE_LIGHT);
}
pub fn register_red_rose_semi_bold(cx: &mut Context) {
    cx.add_font_mem(my_fonts::RED_ROSE_SEMI_BOLD);
}
pub fn register_red_rose_medium(cx: &mut Context) {
    cx.add_font_mem(my_fonts::RED_ROSE_MEDIUM);
}
pub fn register_red_rose_variable_weight(cx: &mut Context) {
    cx.add_font_mem(my_fonts::RED_ROSE_VARIABLE_FONT_WEIGHT);
}
