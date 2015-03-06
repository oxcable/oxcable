//! Provides audio IO from wav files.

#![unstable]

extern crate byteorder;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::fs::File;
use std::io::{self, Seek, SeekFrom};

use components::{InputArray, OutputArray};
use types::{SAMPLE_RATE, Device, Time, Sample};

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
        let mut file = File::open(filename).unwrap();
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
    pub fn restart(&mut self) -> io::Result<u64> {
        self.samples_read = 0;
        self.file.seek(SeekFrom::Start(44))
    }
}

impl Device for WavReader {
    fn tick(&mut self, _t: Time) {
        for i in (0 .. self.num_channels) {
            let s = if self.samples_read < self.num_samples {
                (self.file.read_i16::<LittleEndian>().unwrap() as Sample) / 32768.0
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
        let mut file = File::create(filename).unwrap();
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
        assert!(self.file.seek(SeekFrom::Start(4)).is_ok());
        assert!(self.file.write_u32::<LittleEndian>(file_size as u32).is_ok());
        assert!(self.file.seek(SeekFrom::Start(40)).is_ok());
        assert!(self.file.write_u32::<LittleEndian>(data_size as u32).is_ok());
    }
}

impl Device for WavWriter {
    fn tick(&mut self, t: Time) {
        for i in (0 .. self.num_channels) {
            let mut s = self.inputs.get(i, t).unwrap_or(0.0);
            if s > 0.999f32 { s = 0.999f32; }
            if s < -0.999f32 { s = -0.999f32; }
            assert!(self.file.write_i16::<LittleEndian>((s*32768.0) as i16).is_ok());
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
    fn read_from_file(f: &mut File) -> byteorder::Result<WavHeader> {
        let riff_hdr = try!(f.read_u32::<LittleEndian>());
        let file_size = try!(f.read_u32::<LittleEndian>());
        let wave_lbl = try!(f.read_u32::<LittleEndian>());
        let fmt_hdr = try!(f.read_u32::<LittleEndian>());
        let section_size = try!(f.read_u32::<LittleEndian>());
        let format = try!(f.read_u16::<LittleEndian>());
        let num_channels = try!(f.read_u16::<LittleEndian>());
        let sample_rate = try!(f.read_u32::<LittleEndian>());
        let byte_rate = try!(f.read_u32::<LittleEndian>());
        let block_align = try!(f.read_u16::<LittleEndian>());
        let bit_depth = try!(f.read_u16::<LittleEndian>());
        let data_hdr = try!(f.read_u32::<LittleEndian>());
        let data_size = try!(f.read_u32::<LittleEndian>());
        Ok(WavHeader {
            riff_hdr: riff_hdr,
            file_size: file_size,
            wave_lbl: wave_lbl,
            fmt_hdr: fmt_hdr,
            section_size: section_size,
            format: format,
            num_channels: num_channels,
            sample_rate: sample_rate,
            byte_rate: byte_rate,
            block_align: block_align,
            bit_depth: bit_depth,
            data_hdr: data_hdr,
            data_size: data_size
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
    fn write_to_file(&self, f: &mut File) -> byteorder::Result<()> {
        f.write_u32::<LittleEndian>(self.riff_hdr)
            .and_then(|()| { f.write_u32::<LittleEndian>(self.file_size) })
            .and_then(|()| { f.write_u32::<LittleEndian>(self.wave_lbl) })
            .and_then(|()| { f.write_u32::<LittleEndian>(self.fmt_hdr) })
            .and_then(|()| { f.write_u32::<LittleEndian>(self.section_size) })
            .and_then(|()| { f.write_u16::<LittleEndian>(self.format) })
            .and_then(|()| { f.write_u16::<LittleEndian>(self.num_channels) })
            .and_then(|()| { f.write_u32::<LittleEndian>(self.sample_rate) })
            .and_then(|()| { f.write_u32::<LittleEndian>(self.byte_rate) })
            .and_then(|()| { f.write_u16::<LittleEndian>(self.block_align) })
            .and_then(|()| { f.write_u16::<LittleEndian>(self.bit_depth) })
            .and_then(|()| { f.write_u32::<LittleEndian>(self.data_hdr) })
            .and_then(|()| { f.write_u32::<LittleEndian>(self.data_size) })
    }
}
