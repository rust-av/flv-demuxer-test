use av::format::demuxer::demux::{Demuxer,DemuxerBuilder,DemuxerDescription,Score};
use av::format::demuxer::context::DemuxerContext;
use av::data::packet::Packet;
use av::buffer::Buffered;
use std::io::{BufRead,Error,ErrorKind,SeekFrom};
use nom::{be_u8, be_u32, HexDisplay, IResult, Offset};
use flavors::parser::{Header,header};

/*
module! {
  (Flv) {
    open(self) => { () }
    read_headers(self, context) =>  { Ok(()) }
    read_packet(self, context)  => { unimplemented!() }

    describe(self) => {
      const D: &'static DemuxerDescription = &DemuxerDescription {
        name: "FLV",
        description: "flash video demuxer",
        extensions: &["flv"],
        mime: &["video/x-flv"],
      };

      D
    }

    probe(self, data) => {
      if &data[..3] == b"FLV" {
      //println!("got data:\n{}", &data[..4096].to_hex(16));
      //if let IResult::Done(_,header) = flv_header(data) {
      //  println!("got header: {:?}", header);
        Score::MAX as u8
      } else {
        0
      }
    }

    alloc(self) => {
      let demux = FlvDemuxer {};
      box demux
    }
  }
}
*/
struct FlvDemuxer {
  has_audio: bool,
  has_video: bool,
}
struct FlvDemuxerBuilder;

impl FlvDemuxer {
  pub fn new() -> FlvDemuxer {
    FlvDemuxer {
      has_audio: false,
      has_video: false,
    }
  }
}

impl Demuxer for FlvDemuxer {
  fn open(&mut self) { () }
  fn read_headers(&mut self, context: &Box<Buffered>) -> Result<SeekFrom, Error> {
    match header(context.data()) {
      IResult::Done(i, header) => {
        self.has_audio = header.audio;
        self.has_video = header.video;
        Ok(SeekFrom::Start(header.offset as u64))
      },
      e => Err(Error::new(ErrorKind::InvalidData, format!("err: {:?}", e))),
    }
  }
  fn read_packet(&mut self, context:  &Box<Buffered>) -> Result<(SeekFrom,Packet), Error> {
    unimplemented!()
  }
}

impl DemuxerBuilder for FlvDemuxerBuilder {
  fn describe(&self) -> &'static DemuxerDescription {
    const D: &'static DemuxerDescription = &DemuxerDescription {
      name: "FLV",
      description: "flash video demuxer",
      extensions: &["flv"],
      mime: &["video/x-flv"],
    };

    D
  }
  fn probe(&self, data: &[u8]) -> u8 {
    if &data[..3] == b"FLV" {
    //println!("got data:\n{}", &data[..4096].to_hex(16));
    //if let IResult::Done(_,header) = flv_header(data) {
    //  println!("got header: {:?}", header);
    Score::MAX as u8
    } else {
      0
    }
  }
  fn alloc(&self) -> Box<Demuxer> {
    Box::new(FlvDemuxer::new())
  }
}

// FIXME: not considered as const when imported?
//pub const PROBE_DATA: usize = 4 * 1024;

#[cfg(test)]
mod test {
  use super::{FlvDemuxer,FlvDemuxerBuilder};
  use av::format::demuxer::context::DemuxerContext;
  use av::format::demuxer::demux::{DemuxerBuilder,probe,PROBE_DATA,Score};
  use av::buffer::AccReader;
  use std::io::Cursor;

  const DEMUXER_BUILDERS: [&'static DemuxerBuilder; 1] = [&FlvDemuxerBuilder {}];
  const zelda : &'static [u8] = include_bytes!("../assets/zelda.flv");

  #[test]
  fn probe_demuxer() {
    let builder = probe(&DEMUXER_BUILDERS, zelda).expect("should have found a builder");
    let demuxer = builder.alloc();

    let mut context = DemuxerContext::new(demuxer, Box::new(AccReader::new(zelda)));

    let headers = context.read_headers();
    println!("headers result: {:?}", headers);
    let packet = context.read_packet();

  }
}
