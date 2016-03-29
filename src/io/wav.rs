//! Audio IO from WAV files.

use byteorder::{self, ReadBytesExt, WriteBytesExt, LittleEndian};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

use error::{Error, Result};
use types::{SAMPLE_RATE, AudioDevice, Time, Sample};
use utils::helpers::{i16_to_sample, sample_to_16};


/// Reads audio from a wav file.
///
/// The reader will continue until it runs out of samples. When it does, the
/// reader will return silence until it is reset to the beginning of the file.
pub struct WavReader<R: Read> {
    num_channels: usize,
    num_samples: Time,
    samples_read: Time,
    reader: R
}

impl WavReader<File> {
    /// Returns a `WavReader` reading the provided file.
    pub fn open<P: AsRef<Path>>(filename: P) -> Result<Self> {
        let file = try!(File::open(filename));
        WavReader::new(file)
    }
}

impl<R: Read> WavReader<R> {
    /// Retuns a `WavReader` reading anything implementing `Read`.
    pub fn new(mut reader: R) -> Result<Self> {
        let header = try!(WavHeader::read_from_file(&mut reader));
        Ok(WavReader {
            num_channels: header.num_channels as usize,
            num_samples: (header.data_size / ((header.bit_depth/8) as u32) /
                (header.num_channels as u32)) as Time,
            samples_read: 0,
            reader: reader
        })
    }

    /// Returns the number of audio samples in the wav file.
    pub fn get_num_samples(&self) -> Time {
        self.num_samples
    }

    /// Returns true if we have read the entire wav file.
    pub fn is_done(&self) -> bool {
        self.samples_read >= self.num_samples
    }
}

impl<R: Read+Seek> WavReader<R> {
    /// Resets the reader to begin reading from the start of the file.
    pub fn restart(&mut self) -> io::Result<u64> {
        self.samples_read = 0;
        self.reader.seek(SeekFrom::Start(44))
    }
}

impl<R: Read> AudioDevice for WavReader<R> {
    fn num_inputs(&self) -> usize {
        0
    }

    fn num_outputs(&self) -> usize {
        self.num_channels
    }

    fn tick(&mut self, _: Time, _: &[Sample], outputs: &mut[Sample]) {
        for i in 0..self.num_channels {
            let s = if self.samples_read < self.num_samples {
                let n = self.reader.read_i16::<LittleEndian>()
                    .expect("Failed to read next sample from wav.");
                i16_to_sample(n)
            } else {
                0.0
            };
            outputs[i] = s;
        }
        self.samples_read += 1;
    }
}


/// Writes audio to a wav file.
///
/// The writer initializes the data_size to be 0. This will not be overwritten
/// with the proper size until `update_data_size` is called.
///
/// While `WavReader` only requires its type be `Seek` to use the `restart`
/// method, the `WavWriter` reqiures `Seek` for all types, because the final
/// data size must be written to the header when the writer leaves scope.
pub struct WavWriter<W: Write+Seek> {
    num_channels: usize,
    samples_written: usize,
    writer: W,
}

impl WavWriter<File> {
    /// Returns a `WavWriter` writing to the provided file.
    pub fn create<P: AsRef<Path>>(filename: P, num_channels: usize)
            -> Result<Self> {
        let file = try!(File::create(filename));
        WavWriter::new(file, num_channels)
    }
}

impl<W: Write+Seek> WavWriter<W> {
    /// Returns a `WavWriter` writing to anything implementing `Write`.
    pub fn new(mut writer: W, num_channels: usize) -> Result<Self> {
        let header = WavHeader::new(num_channels as u16, SAMPLE_RATE as u32,
                                    0u32);
        try!(header.write_to_file(&mut writer));
        Ok(WavWriter {
            num_channels: num_channels,
            samples_written: 0,
            writer: writer,
        })
    }
}

impl<W: Write+Seek> Drop for WavWriter<W> {
    fn drop(&mut self) {
        // Updates the wav header to have the correct amount of data written
        let data_size = self.samples_written * self.num_channels * 16/8;
        let file_size = 36+data_size;
        self.writer.seek(SeekFrom::Start(4))
            .expect("Failed to seek wav file size.");
        self.writer.write_u32::<LittleEndian>(file_size as u32)
            .expect("Failed to write wav file size.");
        self.writer.seek(SeekFrom::Start(40))
            .expect("Failed to seek wav data size.");
        self.writer.write_u32::<LittleEndian>(data_size as u32)
            .expect("Failed to write wav data size.");
    }
}

impl<W: Write+Seek> AudioDevice for WavWriter<W> {
    fn num_inputs(&self) -> usize {
        self.num_channels
    }

    fn num_outputs(&self) -> usize {
        0
    }

    fn tick(&mut self, _: Time, inputs: &[Sample], _: &mut[Sample]) {
        for s in inputs.iter() {
            self.writer.write_i16::<LittleEndian>(sample_to_16(*s))
                .expect("Failed to write next sample to wav file.");
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
#[derive(Clone, Debug)]
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
    fn new(num_channels: u16, sample_rate: u32, data_size: u32) -> Self {
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
    fn read_from_file<R: Read>(f: &mut R) -> Result<Self> {
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
        let header = WavHeader {
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
        };
        header.check()
    }

    /// Returns the header if the wav header has valid fields and uses the
    /// supported formats, otherwise return a descriptive error
    fn check(self) -> Result<Self> {
        // Check the headers are correct
        if self.riff_hdr != RIFF { return Err(Error::InvalidFile); }
        if self.wave_lbl != WAVE { return Err(Error::InvalidFile); }
        if self.fmt_hdr  != FMT_ { return Err(Error::InvalidFile); }
        if self.data_hdr != DATA { return Err(Error::InvalidFile); }

        // Check sizes are correct
        if self.file_size != self.data_size + 36 {
            return Err(Error::InvalidFile);
        }
        if self.section_size != 16 {
            return Err(Error::InvalidFile);
        }
        if self.byte_rate != self.sample_rate*(self.num_channels as u32)*
            (self.bit_depth as u32)/8 {
            return Err(Error::InvalidFile);
        }
        if self.block_align != self.num_channels*self.bit_depth/8 {
            return Err(Error::InvalidFile);
        }

        // Check for formats we can read
        if self.format != 1 {
            return Err(Error::Unsupported("Only PCM is supported"));
        }
        if self.sample_rate != (SAMPLE_RATE as u32) {
            return Err(Error::Unsupported(
                    "Sample rate conversion not supported"));
        }
        if self.bit_depth != 16 {
            return Err(Error::Unsupported("Only 16-bit supported"));
        }

        // If this header is valid, then return it instead
        Ok(self)
    }

    /// Attempts to write this wav header to the provided file
    fn write_to_file<W: Write>(&self, w: &mut W) -> byteorder::Result<()> {
        w.write_u32::<LittleEndian>(self.riff_hdr)
            .and_then(|()| w.write_u32::<LittleEndian>(self.file_size))
            .and_then(|()| w.write_u32::<LittleEndian>(self.wave_lbl))
            .and_then(|()| w.write_u32::<LittleEndian>(self.fmt_hdr))
            .and_then(|()| w.write_u32::<LittleEndian>(self.section_size))
            .and_then(|()| w.write_u16::<LittleEndian>(self.format))
            .and_then(|()| w.write_u16::<LittleEndian>(self.num_channels))
            .and_then(|()| w.write_u32::<LittleEndian>(self.sample_rate))
            .and_then(|()| w.write_u32::<LittleEndian>(self.byte_rate))
            .and_then(|()| w.write_u16::<LittleEndian>(self.block_align))
            .and_then(|()| w.write_u16::<LittleEndian>(self.bit_depth))
            .and_then(|()| w.write_u32::<LittleEndian>(self.data_hdr))
            .and_then(|()| w.write_u32::<LittleEndian>(self.data_size))
    }
}


#[cfg(test)]
mod test {
    use std::io::Cursor;

    use types::AudioDevice;
    use super::{WavHeader, WavReader, WavWriter};

    static WAV_HEADER: [u8; 48] =
        [0x52, 0x49, 0x46, 0x46, 0x28, 0x00, 0x00, 0x00, 0x57, 0x41,
         0x56, 0x45, 0x66, 0x6D, 0x74, 0x20, 0x10, 0x00, 0x00, 0x00,
         0x01, 0x00, 0x02, 0x00, 0x44, 0xAC, 0x00, 0x00, 0x10, 0xB1,
         0x02, 0x00, 0x04, 0x00, 0x10, 0x00, 0x64, 0x61, 0x74, 0x61,
         0x04, 0x00, 0x00, 0x00, 0x00, 0x80, 0xFF, 0x7F];

    #[test]
    fn test_read_wav_header() {
        let mut cursor = Cursor::new(&WAV_HEADER[..]);
        let header = WavHeader::read_from_file(&mut cursor).unwrap();
        assert_eq!(header.bit_depth, 16);
        assert_eq!(header.data_size, 4);
        assert_eq!(header.num_channels, 2);
    }

    #[test]
    fn test_wav_reader() {
        let cursor = Cursor::new(&WAV_HEADER[..]);
        let mut reader = WavReader::new(cursor).unwrap();
        assert_eq!(reader.num_inputs(), 0);
        assert_eq!(reader.num_outputs(), 2);
        assert_eq!(reader.get_num_samples(), 1);
        assert_eq!(reader.is_done(), false);

        let mut output = [0.0, 0.0];
        reader.tick(0, &[], &mut output);
        assert_eq!(reader.is_done(), true);
        assert_eq!(output, [-1.0, 0.9999695]);

        reader.tick(1, &[], &mut output);
        assert_eq!(reader.is_done(), true);
        assert_eq!(output, [0.0, 0.0]);

        reader.restart().unwrap();
        assert_eq!(reader.is_done(), false);
        reader.tick(0, &[], &mut output);
        assert_eq!(reader.is_done(), true);
        assert_eq!(output, [-1.0, 0.9999695]);
    }

    #[test]
    fn test_wav_writer() {
        let mut buffer = [0u8; 48];
        {
            // Scope the cursor so its borrow on the buffer ends.
            // Scope the writer so it gets dropped and the file size written.
            let cursor = Cursor::new(&mut buffer[..]);
            let mut writer = WavWriter::new(cursor, 2).unwrap();
            assert_eq!(writer.num_inputs(), 2);
            assert_eq!(writer.num_outputs(), 0);
            writer.tick(0, &[-1.0, 0.9999695], &mut[]);
        }
        assert_eq!(&buffer[..], &WAV_HEADER[..]);
    }
}
