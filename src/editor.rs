//use atomic_float::AtomicF32;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};

//use std::sync::atomic::Ordering;
use std::sync::Arc;
//use std::time::Duration;

use crate::PolyModSynthParams;

mod my_assets;

pub mod my_fonts;

#[derive(Lens)]
struct Data {
    params: Arc<PolyModSynthParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 300))
}

pub(crate) fn create(
    params: Arc<PolyModSynthParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        //assets::register_noto_sans_light(cx);
        //assets::register_noto_sans_thin(cx);
        //vizia_assets::register_roboto_bold(cx);
        //vizia_assets::register_roboto_italic(cx);
        //vizia_assets::register_roboto(cx);


        cx.add_font_mem(&my_fonts::RED_ROSE_REGULAR);

        my_assets::register_red_rose_regular(cx);
        my_assets::register_red_rose_bold(cx);
        my_assets::register_red_rose_light(cx);
        my_assets::register_red_rose_semi_bold(cx);
        my_assets::register_red_rose_medium(cx);
        my_assets::register_red_rose_variable_weight(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Sample Synth GUI")
                .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
                .font_weight(FontWeightKeyword::Bold)
                .font_size(30.0)
                .height(Pixels(70.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0))
                .color(RGBA::rgb(225,255,255));

            Label::new(cx, "Gain")
            .color(RGBA::rgb(225,0,0))
            .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
            .font_weight(FontWeightKeyword::SemiBold);

            ParamSlider::new(cx, Data::params, |params| &params.gain)
            .color(RGBA::rgb(225,0,0))
            .border_color(RGBA::rgb(123,133,230))
            .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
            .font_weight(FontWeightKeyword::Light);

            VStack::new(cx, |cx| {
                Label::new(cx, "LFO")
                .color(RGBA::rgb(55,55,255))
                .child_top(Pixels(5.0))
                .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
                .font_weight(FontWeightKeyword::SemiBold);
    
                ParamSlider::new(cx, Data::params, |params| &params.lfo_rate)
                .color(RGBA::rgb(55,55,255))
                .border_color(RGBA::rgb(123,133,230))
                .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
                .font_weight(FontWeightKeyword::Light)
                .background_color(RGBA::rgb(253,253,250));
    
                ParamSlider::new(cx, Data::params, |params| &params.lfo_wave)
                .color(RGBA::rgb(55,55,255))
                .border_color(RGBA::rgb(123,133,230))
                .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
                .font_weight(FontWeightKeyword::Light)
                .child_bottom(Pixels(10.0))
                .background_color(RGBA::rgb(253,253,250));
    
                HStack::new(cx, move |cx| {
                    Label::new(cx, "Amount")
                    .color(RGBA::rgb(55,55,255))
                    .width(Pixels(100.0))
                    .child_left(Stretch(0.0))
                    .child_right(Stretch(1.0))
                    .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
                    .font_weight(FontWeightKeyword::Regular);
    
                    ParamSlider::new(cx, Data::params, |params| &params.lfo_gain)
                    .color(RGBA::rgb(55,55,255))
                    .width(Pixels(100.0))
                    .border_color(RGBA::rgb(123,133,230))
                    .font_family(vec![FamilyOwned::Name(String::from(my_assets::RED_ROSE))])
                    .font_weight(FontWeightKeyword::Light)
                    .background_color(RGBA::rgb(253,253,250));
                    //.child_left(Stretch(1.0))
                    //.child_right(Stretch(0.0));
                })
                .left(Units::Percentage(10.0));
            })
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0))
            .left(Units::Percentage(10.0))
            .width(Units::Percentage(80.0))
            .child_bottom(Pixels(10.0))
            .child_space(Stretch(1.0))
            .row_between(Pixels(5.0))
            .border_radius(Units::Percentage(20.0))
            .background_color(RGBA::rgb(33,33,30));

        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0))
        .height(Pixels(300.0))
        .background_color(RGBA::rgb(0,0,0));

        ResizeHandle::new(cx);
    })
    
}