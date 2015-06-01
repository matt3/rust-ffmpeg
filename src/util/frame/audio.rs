use libc::c_int;
use std::mem;
use std::ops::{Deref, DerefMut};

use ffi::*;
use ::Samples;
use ::util::format;
use super::Frame;

#[derive(PartialEq, Eq)]
pub struct Audio(Frame);

impl Audio {
	pub fn empty() -> Self {
		unsafe {
			Audio(Frame { ptr: av_frame_alloc() })
		}
	}

	pub fn new(format: format::Sample, samples: usize, layout: i64) -> Self {
		unsafe {
			let mut frame = Audio::empty();

			frame.set_format(format);
			frame.set_sample_number(samples);
			frame.set_channel_layout(layout);

			av_frame_get_buffer(frame.ptr, 1);

			frame
		}
	}

	pub fn format(&self) -> format::Sample {
		unsafe {
			if (*self.ptr).format == -1 {
				format::Sample::None
			}
			else {
				format::Sample::from(mem::transmute::<_, AVSampleFormat>(((*self.ptr).format)))
			}
		}
	}

	pub fn set_format(&mut self, value: format::Sample) {
		unsafe {
			(*self.ptr).format = mem::transmute::<AVSampleFormat, c_int>(value.into());
		}
	}

	pub fn channel_layout(&self) -> i64 {
		unsafe {
			av_frame_get_channel_layout(self.ptr)
		}
	}

	pub fn set_channel_layout(&mut self, value: i64) {
		unsafe {
			av_frame_set_channel_layout(self.ptr, value);
		}
	}

	pub fn channels(&self) -> u16 {
		unsafe {
			av_frame_get_channels(self.ptr) as u16
		}
	}

	pub fn set_channels(&mut self, value: u16) {
		unsafe {
			av_frame_set_channels(self.ptr, value as c_int);
		}
	}

	pub fn rate(&self) -> u32 {
		unsafe {
			av_frame_get_sample_rate(self.ptr) as u32
		}
	}

	pub fn set_rate(&mut self, value: u32) {
		unsafe {
			av_frame_set_sample_rate(self.ptr, value as c_int);
		}
	}

	pub fn sample_number(&self) -> usize {
		unsafe {
			(*self.ptr).nb_samples as usize
		}
	}

	pub fn set_sample_number(&mut self, value: usize) {
		unsafe {
			(*self.ptr).nb_samples = value as c_int;
		}
	}

	pub fn samples(&self) -> Samples {
		Samples::wrap(self.ptr as *mut AVPicture, self.format(), self.rate(), self.sample_number(), self.channels())
	}

	pub fn samples_mut(&mut self) -> Samples {
		Samples::wrap(self.ptr as *mut AVPicture, self.format(), self.rate(), self.sample_number(), self.channels())
	}
}

unsafe impl Send for Audio { }

impl Deref for Audio {
	type Target = Frame;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

impl DerefMut for Audio {
	fn deref_mut(&mut self) -> &mut<Self as Deref>::Target {
		&mut self.0
	}
}

impl Clone for Audio {
	fn clone(&self) -> Self {
		let mut cloned = Audio::new(self.format(), self.sample_number(), self.channel_layout());
		cloned.clone_from(self);

		cloned
	}

	fn clone_from(&mut self, source: &Self) {
		unsafe {
			av_frame_copy(self.ptr, source.ptr);
			av_frame_copy_props(self.ptr, source.ptr);
		}
	}
}
