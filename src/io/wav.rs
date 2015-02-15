//! Provides audio IO from wav files.

#![unstable]

use std::old_io::{IoResult, File, Truncate, ReadWrite, SeekEnd, SeekSet};

use core::components::{InputArray, OutputArray};
use core::types::{SAMPLE_RATE, Device, Time, Sample};


/// Reads audio from a wav file.
///
/// The reader will continue until it runs out of samples. When it does, the
/// reader will return silence until it is reset to the beginning of the file.
#[stable]
pub struct WavReader {
    /// Output audio channels
    #[stable]
    pub outputs: OutputArray<Sample>,

    num_channels: usize,
    num_samples: usize,
    samples_read: usize,
    file: File
}

impl WavReader {
    /// Returns a `WavReader` reading the provided file.
    ///
    /// This function panics if the file can't be opened, or is not a valid wav
    /// file.
    #[stable]
    pub fn new(filename: &str) -> WavReader {
        let mut file = File::open(&Path::new(filename)).unwrap();
        let header = WavHeader::read_from_file(&mut file).unwrap();
        assert!(header.is_valid());
        WavReader {
            outputs: OutputArray::new(header.num_channels as usize),
            num_channels: header.num_channels as usize,
            num_samples: (header.data_size / ((header.bit_depth/8) as u32) / 
                (header.num_channels as u32)) as usize,
            samples_read: 0,
            file: file
        }
    }

    /// Returns the number of audio samples in the wav file.
    #[stable]
    pub fn get_num_samples(&self) -> usize {
        self.num_samples
    }

    /// Returns true if we have read the entire wav file.
    #[stable]
    pub fn is_done(&self) -> bool {
        self.samples_read >= self.num_samples
    }

    /// Resets the reader to begin reading from the start of the file.
    #[stable]
    pub fn restart(&mut self) -> IoResult<()> {
        self.samples_read = 0;
        self.file.seek(44, SeekSet)
    }
}

impl Device for WavReader {
    fn tick(&mut self, _t: Time) {
        for i in (0 .. self.num_channels) {
            let s = if self.samples_read < self.num_samples {
                (self.file.read_le_i16().unwrap() as Sample) / 32768.0
            } else {
                0.0
            };
            self.outputs.push(i, s);
        }
        self.samples_read += 1;
    }
}


/// Writes audio to a wav file.
///
/// The writer initializes the data_size to be 0. This will not be overwritten
/// with the proper size until `update_data_size` is called.
#[stable]
pub struct WavWriter {
    /// Input audio channels
    #[stable]
    pub inputs: InputArray<Sample>,

    num_channels: usize,
    file: File,
    samples_written: usize,
}

impl WavWriter {
    /// Returns a `WavWriting` writing to the provided file
    ///
    /// This function panics if the file can't be opened or written to
    #[stable]
    pub fn new(filename: &str, num_channels: usize) -> WavWriter {
        let mut file = File::open_mode(&Path::new(filename), Truncate, 
                                      ReadWrite).unwrap();
        let header = WavHeader::new(num_channels as u16, SAMPLE_RATE as u32,
                                    0u32);
        assert!(header.write_to_file(&mut file).is_ok());
        WavWriter { 
            inputs: InputArray::new(num_channels), 
            num_channels: num_channels,
            file: file,
            samples_written: 0
        }
    }

    /// Updates the wav header to have the correct amount of data written
    pub fn update_data_size(&mut self) {
        let data_size = self.samples_written * self.num_channels * 16/8;
        let file_size = 36+data_size;
        assert!(self.file.seek(4, SeekSet).is_ok());
        assert!(self.file.write_le_u32(file_size as u32).is_ok());
        assert!(self.file.seek(40, SeekSet).is_ok());
        assert!(self.file.write_le_u32(data_size as u32).is_ok());
        assert!(self.file.seek(0, SeekEnd).is_ok());
    }
}

impl Device for WavWriter {
    fn tick(&mut self, t: Time) {
        for i in (0 .. self.num_channels) {
            let mut s = self.inputs.get(i, t).unwrap_or(0.0);
            if s > 0.999f32 { s = 0.999f32; }
            if s < -0.999f32 { s = -0.999f32; }
            assert!(self.file.write_le_i16((s*32768.0) as i16).is_ok());
        }
        self.samples_written += 1;
    }
}


/// Constants for the strings used in a wav header
static RIFF: u32 = 0x46464952;
static WAVE: u32 = 0x45564157;
static FMT_: u32 = 0x20746d66;
static DATA: u32 = 0x61746164;

/// A struct container for the wav header
#[derive(Debug)]
struct WavHeader {
    riff_hdr: u32,
    file_size: u32,
    wave_lbl: u32,
    fmt_hdr: u32,
    section_size: u32,
    format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bit_depth: u16,
    data_hdr: u32,
    data_size: u32,
}

impl WavHeader {
    /// Returns a new wav header with all values initalized for our supported
    /// audio formats
    fn new(num_channels: u16, sample_rate: u32, data_size: u32) -> WavHeader {
        WavHeader {
            riff_hdr: RIFF,
            file_size: data_size+36,
            wave_lbl: WAVE,
            fmt_hdr: FMT_,
            section_size: 16,
            format: 1,
            num_channels: num_channels,
            sample_rate: sample_rate,
            byte_rate: sample_rate*(num_channels as u32)*16/8,
            block_align: num_channels*16/8,
            bit_depth: 16,
            data_hdr: DATA,
            data_size: data_size,
        }
    }

    /// Attempts to read a wav header from the provided file
    fn read_from_file(f: &mut File) -> IoResult<WavHeader> {
        let riff_hdr = f.read_le_u32();
        if  riff_hdr.is_err() { return Err(riff_hdr.unwrap_err()) }
        let file_size = f.read_le_u32();
        if  file_size.is_err() { return Err(file_size.unwrap_err()) }
        let wave_lbl = f.read_le_u32();
        if  wave_lbl.is_err() { return Err(wave_lbl.unwrap_err()) }
        let fmt_hdr = f.read_le_u32();
        if  fmt_hdr.is_err() { return Err(fmt_hdr.unwrap_err()) }
        let section_size = f.read_le_u32();
        if  section_size.is_err() { return Err(section_size.unwrap_err()) }
        let format = f.read_le_u16();
        if  format.is_err() { return Err(format.unwrap_err()) }
        let num_channels = f.read_le_u16();
        if  num_channels.is_err() { return Err(num_channels.unwrap_err()) }
        let sample_rate = f.read_le_u32();
        if  sample_rate.is_err() { return Err(sample_rate.unwrap_err()) }
        let byte_rate = f.read_le_u32();
        if  byte_rate.is_err() { return Err(byte_rate.unwrap_err()) }
        let block_align = f.read_le_u16();
        if  block_align.is_err() { return Err(block_align.unwrap_err()) }
        let bit_depth = f.read_le_u16();
        if  bit_depth.is_err() { return Err(bit_depth.unwrap_err()) }
        let data_hdr = f.read_le_u32();
        if  data_hdr.is_err() { return Err(data_hdr.unwrap_err()) }
        let data_size = f.read_le_u32();
        if  data_size.is_err() { return Err(data_size.unwrap_err()) }
        Ok(WavHeader {
            riff_hdr: riff_hdr.unwrap(),
            file_size: file_size.unwrap(),
            wave_lbl: wave_lbl.unwrap(),
            fmt_hdr: fmt_hdr.unwrap(),
            section_size: section_size.unwrap(),
            format: format.unwrap(),
            num_channels: num_channels.unwrap(),
            sample_rate: sample_rate.unwrap(),
            byte_rate: byte_rate.unwrap(),
            block_align: block_align.unwrap(),
            bit_depth: bit_depth.unwrap(),
            data_hdr: data_hdr.unwrap(),
            data_size: data_size.unwrap()
        })
    }

    /// Returns true if this wav header has valid fields and uses the supported
    /// formats
    fn is_valid(&self) -> bool {
        // Check the headers are correct
        if self.riff_hdr != RIFF { return false; }
        if self.wave_lbl != WAVE { return false; }
        if self.fmt_hdr  != FMT_ { return false; }
        if self.data_hdr != DATA { return false; }
        
        // Check sizes are correct
        if self.file_size != self.data_size + 36 { return false; }
        if self.section_size != 16 { return false; }
        if self.byte_rate != self.sample_rate*(self.num_channels as u32)*
            (self.bit_depth as u32)/8 {
            return false;
        }
        if self.block_align != self.num_channels*self.bit_depth/8 {
            return false;
        }

        // Check for formats we can read
        if self.format != 1 { return false; }
        if self.sample_rate != (SAMPLE_RATE as u32) { return false; }
        if self.bit_depth != 16 { return false; }

        true
    }

    /// Attempts to write this wav header to the provided file
    fn write_to_file(&self, f: &mut File) -> IoResult<()> {
        f.write_le_u32(self.riff_hdr)
            .and_then(|()| { f.write_le_u32(self.file_size) })
            .and_then(|()| { f.write_le_u32(self.wave_lbl) })
            .and_then(|()| { f.write_le_u32(self.fmt_hdr) })
            .and_then(|()| { f.write_le_u32(self.section_size) })
            .and_then(|()| { f.write_le_u16(self.format) })
            .and_then(|()| { f.write_le_u16(self.num_channels) })
            .and_then(|()| { f.write_le_u32(self.sample_rate) })
            .and_then(|()| { f.write_le_u32(self.byte_rate) })
            .and_then(|()| { f.write_le_u16(self.block_align) })
            .and_then(|()| { f.write_le_u16(self.bit_depth) })
            .and_then(|()| { f.write_le_u32(self.data_hdr) })
            .and_then(|()| { f.write_le_u32(self.data_size) })
    }
}
