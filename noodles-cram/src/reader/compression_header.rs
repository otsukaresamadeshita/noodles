mod data_series_encodings;
mod preservation_map;
mod tag_encodings;

use std::io::{self, Read};

use self::{
    data_series_encodings::read_data_series_encodings, preservation_map::read_preservation_map,
    tag_encodings::read_tag_encodings,
};

use crate::{Block, CompressionHeader};

use super::block::read_block;

pub fn read_compression_header<R>(reader: &mut R) -> io::Result<CompressionHeader>
where
    R: Read,
{
    let mut block = Block::default();
    read_block(reader, &mut block)?;

    let data = block.decompressed_data();
    let mut data_reader = &data[..];

    let preservation_map = read_preservation_map(&mut data_reader)?;
    let data_series_encodings = read_data_series_encodings(&mut data_reader)?;
    let tag_encodings = read_tag_encodings(&mut data_reader)?;

    Ok(CompressionHeader::new(
        preservation_map,
        data_series_encodings,
        tag_encodings,
    ))
}