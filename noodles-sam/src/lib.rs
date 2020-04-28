pub mod cigar;
pub mod data;
mod flags;
pub mod header;
mod mapping_quality;
mod reader;
pub mod record;
mod writer;

pub use self::{
    cigar::Cigar, data::Data, flags::Flags, header::Header, mapping_quality::MappingQuality,
    reader::Reader, record::Record, writer::Writer,
};
