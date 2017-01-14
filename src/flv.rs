use av::format::demuxer::demux::{Demuxer,DemuxerBuilder,DemuxerDescription,PROBE_DATA,Score};
use av::format::demuxer::context::DemuxerContext;
use av::data::packet::Packet;
use std::io::{BufRead,Error};
use nom::{be_u8, be_u32, HexDisplay, IResult};

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
struct FlvDemuxer;
struct FlvDemuxerBuilder;

impl Demuxer for FlvDemuxer {
  fn open(&mut self) { () }
  fn read_headers(&mut self, context: &Box<BufRead>) -> Result<(), Error> {
    Ok(())
  }
  fn read_packet(&mut self, context:  &Box<BufRead>) -> Result<Packet, Error> {
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
    Box::new(FlvDemuxer { })
  }
}

#[derive(Debug,PartialEq,Eq)]
pub struct FlvHeader {
  pub version: u8,
  pub audio: bool,
  pub video: bool,
  pub offset: u32,
}

named!(pub flv_header<FlvHeader>,
  chain!(
             tag!("FLV") ~
    version: be_u8       ~
    flags:   be_u8       ~
    offset:  be_u32      ,
    || {
      FlvHeader {
        version: version,
        audio:   flags & 4 == 4,
        video:   flags & 1 == 1,
        offset:  offset
      }
    }
  )
);

// FIXME: not considered as const when imported?
//pub const PROBE_DATA: usize = 4 * 1024;

#[cfg(test)]
mod test {
  use super::{FlvDemuxer,FlvDemuxerBuilder};
  use av::format::demuxer::demux::{DemuxerBuilder,probe,PROBE_DATA,Score};

  const DEMUXER_BUILDERS: [&'static DemuxerBuilder; 1] = [&FlvDemuxerBuilder {}];
  const zelda : &'static [u8] = include_bytes!("../assets/zelda.flv");

  #[test]
  fn probe_demuxer() {
    match probe(&DEMUXER_BUILDERS, zelda) {
      Some(_) => panic!("some"),
      None => panic!("none"),
    }
  }
}
