use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor, nih_log};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::GainParams;

use self::sample::SampleView;

#[path = "./widgets/sample.rs"]
mod sample;

#[derive(Lens)]
struct Data {
    params: Arc<GainParams>,
    peak_meter: Arc<AtomicF32>,
    buffer_view: Arc<AtomicF32>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (300, 250))
}

pub(crate) fn create(
    params: Arc<GainParams>,
    peak_meter: Arc<AtomicF32>,
    buffer_view: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        assets::register_noto_sans_light(cx);
        assets::register_noto_sans_thin(cx);

        Data {
            params: params.clone(),
            peak_meter: peak_meter.clone(),
            buffer_view: buffer_view.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Gain GUI")
                .font_family(vec![FamilyOwned::Name(String::from(assets::NOTO_SANS))])
                .font_weight(FontWeightKeyword::Thin)
                .font_size(30.0)
                .height(Pixels(70.0))
                .child_top(Stretch(1.0))
                .child_bottom(Pixels(0.0));

            Label::new(cx, "Gain");
            ParamSlider::new(cx, Data::params, |params| &params.gain);

            /*GenericUi::new(cx, Data::buffer_view);
            */

            Label::new(cx, "Toggle");
            ParamButton::new(cx, Data::params, |params| &params.square_toggle);

            

            PeakMeter::new(
                cx,
                Data::peak_meter
                    .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
                Some(Duration::from_millis(600)),
            )
            // This is how adding padding works in vizia
            .top(Pixels(10.0));

            let x = buffer_view.load(Ordering::Relaxed);


            let buffer_string = format!("beffer:- {:>+10.10}", 
            x);

            nih_log!("TEST_Editor");

            Label::new(
                cx, 
                &buffer_string
            );

            SampleView::new(
                cx,
                Data::buffer_view.map(|buffer_view|(buffer_view.load(Ordering::Relaxed))),
                Some(Duration::from_millis(10)),
            );
            

            PeakMeter::new(
                cx,
                Data::buffer_view.map(|buffer_view|(buffer_view.load(Ordering::Relaxed))),
                Some(Duration::from_millis(10)),
            )
            .top(Pixels(10.0));
        })
        .row_between(Pixels(0.0))
        .child_left(Stretch(1.0))
        .child_right(Stretch(1.0));

        ResizeHandle::new(cx);
    })
}