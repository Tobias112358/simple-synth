use nih_plug::prelude::*;

use simple_synth::PolyModSynth;

fn main() {
    nih_export_standalone::<PolyModSynth>();
}