extern crate minimp3;

use minimp3::safe::Context;
use minimp3::safe::ErrorMessage;

fn main() {
    let mp3_buf = include_bytes!("../minimp3/vectors/M2L3_bitrate_24_all.bit");

    let mut context = minimp3::safe::Context::new();

    let mut pcm = [0i16; Context::MAX_SAMPLES_PER_FRAME];

    let mut total_samples = 0;
    let mut offset = 0usize;

    loop {
        let frame = match context.decode_frame(&mp3_buf[offset..], &mut pcm) {
            Ok(frame) => frame,
            Err(ErrorMessage::NoValidData) => break,
            Err(e) => panic!("unexpected decode error: {:?}", e),
        };

        println!("frame {:?}", frame);
        offset += frame.frame_bytes;
        total_samples += frame.samples;
    }

    println!("---");
    println!("total_samples = {}", total_samples);
}
