//use atomic_float::AtomicF32;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};

//use std::sync::atomic::Ordering;
use std::sync::Arc;
//use std::time::Duration;

use crate::PolyModSynthParams;

#[derive(Lens)]
struct Data {
    params: Arc<PolyModSynthParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 250))
}

pub(crate) fn create(
    params: Arc<PolyModSynthParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Sample Synth GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .height(Pixels(70.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Gain")
            .color(RGBA::rgb(225,0,0));

            ParamSlider::new(cx, Data::params, |params| &params.gain)
            .color(RGBA::rgb(225,0,0));

            VStack::new(cx, |cx| {
                Label::new(cx, "LFO")
                .color(RGBA::rgb(0,0,225))
                .child_top(Pixels(10.0));
    
                ParamSlider::new(cx, Data::params, |params| &params.lfo_rate)
                .color(RGBA::rgb(0,0,225));
    
                ParamSlider::new(cx, Data::params, |params| &params.lfo_wave)
                .color(RGBA::rgb(0,0,225));
    
                HStack::new(cx, move |cx| {
                    Label::new(cx, "Amount")
                    .color(RGBA::rgb(0,0,225))
                    .width(Pixels(100.0))
                    .child_left(Stretch(0.0))
                    .child_right(Stretch(1.0));
    
                    ParamSlider::new(cx, Data::params, |params| &params.lfo_gain)
                    .color(RGBA::rgb(0,0,225))
                    .width(Pixels(100.0));
                    //.child_left(Stretch(1.0))
                    //.child_right(Stretch(0.0));
                })
                .left(Units::Percentage(10.0));
            })
            .child_left(Stretch(1.0))
            .child_right(Stretch(1.0))
            .left(Units::Percentage(10.0))
            .width(Units::Percentage(80.0))
            .background_color(RGBA::rgb(225,225,180));

            

            

        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        ResizeHandle::new(cx);
    })
}