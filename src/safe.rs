use cast::i32;
use cast::usize;

use bindgen;

pub struct Context {
    inner: bindgen::mp3dec_t,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FrameInfo {
    /// Valid samples recovered from the input buffer
    pub samples: usize,
    /// Bytes consumed from the input buffer
    pub frame_bytes: usize,

    pub channels: usize,
    pub hz: usize,
    pub layer: usize,
    pub bitrate_kbps: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorMessage {
    PcmTooShort,
    FileTooLong,
    NoValidData,
    UnrepresentableReturnValue,
}

impl Context {
    pub const MAX_SAMPLES_PER_FRAME: usize = bindgen::MINIMP3_MAX_SAMPLES_PER_FRAME as usize;

    #[inline]
    pub fn new() -> Context {
        let mut inner = Default::default();
        assert_eq!((), unsafe { bindgen::mp3dec_init(&mut inner) });
        Context { inner }
    }

    pub fn decode_frame(
        &mut self,
        file: &[u8],
        pcm: &mut [i16],
    ) -> Result<FrameInfo, ErrorMessage> {
        if pcm.len() < Context::MAX_SAMPLES_PER_FRAME {
            return Err(ErrorMessage::PcmTooShort);
        }

        let mut raw_frame_info = bindgen::mp3dec_frame_info_t::default();

        let samples = unsafe {
            bindgen::mp3dec_decode_frame(
                &mut self.inner,
                file.as_ptr(),
                i32(file.len()).map_err(|_| ErrorMessage::FileTooLong)?,
                pcm.as_mut_ptr(),
                &mut raw_frame_info,
            )
        };

        if 0 == samples {
            return Err(ErrorMessage::NoValidData);
        }

        let samples = to_usize(samples)?;

        assert_le!(samples, Context::MAX_SAMPLES_PER_FRAME);

        let bindgen::mp3dec_frame_info_t {
            frame_bytes,
            channels,
            hz,
            layer,
            bitrate_kbps,
        } = raw_frame_info;

        let frame_bytes = to_usize(frame_bytes)?;

        assert_le!(frame_bytes, file.len());

        Ok(FrameInfo {
            samples,
            frame_bytes,
            channels: to_usize(channels)?,
            hz: to_usize(hz)?,
            layer: to_usize(layer)?,
            bitrate_kbps: to_usize(bitrate_kbps)?,
        })
    }
}

#[inline]
fn to_usize(val: ::std::os::raw::c_int) -> Result<usize, ErrorMessage> {
    usize(val).map_err(|_| ErrorMessage::UnrepresentableReturnValue)
}
