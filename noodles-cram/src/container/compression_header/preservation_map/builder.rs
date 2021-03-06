use super::{substitution_matrix, tag_ids_dictionary, PreservationMap};

use crate::Record;

#[derive(Debug)]
pub struct Builder {
    read_names_included: bool,
    ap_data_series_delta: bool,
    reference_required: bool,
    substitution_matrix_builder: substitution_matrix::Builder,
    tag_ids_dictionary_builder: tag_ids_dictionary::Builder,
}

impl Builder {
    pub fn set_read_names_included(mut self, read_names_included: bool) -> Self {
        self.read_names_included = read_names_included;
        self
    }

    pub fn set_ap_data_series_delta(mut self, ap_data_series_delta: bool) -> Self {
        self.ap_data_series_delta = ap_data_series_delta;
        self
    }

    pub fn set_reference_required(mut self, reference_required: bool) -> Self {
        self.reference_required = reference_required;
        self
    }

    pub fn update(&mut self, reference_sequence: &[u8], record: &Record) {
        self.substitution_matrix_builder
            .update(reference_sequence, record);
        self.tag_ids_dictionary_builder.update(record);
    }

    pub fn build(self) -> PreservationMap {
        let substitution_matrix = self.substitution_matrix_builder.build();
        let tag_ids_dictionary = self.tag_ids_dictionary_builder.build();

        PreservationMap::new(
            self.read_names_included,
            self.ap_data_series_delta,
            self.reference_required,
            substitution_matrix,
            tag_ids_dictionary,
        )
    }
}

impl Default for Builder {
    // § 8.4 Compression header block (2020-06-22): "The boolean values are optional, defaulting to
    // true when absent, although it is recommended to explicitly set them."
    fn default() -> Self {
        Self {
            read_names_included: true,
            ap_data_series_delta: true,
            reference_required: true,
            substitution_matrix_builder: substitution_matrix::Builder::default(),
            tag_ids_dictionary_builder: tag_ids_dictionary::Builder::default(),
        }
    }
}
