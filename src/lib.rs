#[macro_use] extern crate vst;

extern crate lerp;

use vst::buffer::AudioBuffer;
use vst::plugin::{Plugin, Info};
use lerp::Lerp;

struct DigiDist {
    threshold: f32,
    dry_wet_mix: f32
}

impl Default for DigiDist{
    fn default() -> Self {
        DigiDist {
            threshold: 1.0,
            dry_wet_mix: 0.0
        }
    }
}


impl Plugin for DigiDist {
    fn get_info(&self) -> Info {
        Info {
            name: "DigiDist".to_string(),
            vendor: "alexyer".to_string(),
            unique_id: 25032017,

            inputs: 2,
            outputs: 2,
            parameters: 2,

            ..Info::default()
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.threshold * 100.0),
            1 => format!("{}", self.dry_wet_mix * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold".to_string(),
            1 => "Dry/Wet Mix".to_string(),
            _ => "".to_string(),
        }

    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold,
            1 => self.dry_wet_mix,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.threshold = value.max(0.01),
            1 => self.dry_wet_mix = value,
            _ => (),
        };
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        for (input, output) in buffer.zip() {
            // For each input sample and output sample in buffer
            for (in_frame, out_frame) in input.into_iter().zip(output.into_iter()) {
                let distorted;

                if *in_frame >= 0.0 {
                    distorted = in_frame.min(self.threshold) / self.threshold;
                } else {
                    distorted = in_frame.max(-self.threshold) / self.threshold;
                }

                *out_frame = in_frame.lerp(distorted, self.dry_wet_mix);
            }
        }
    }
}

plugin_main!(DigiDist);