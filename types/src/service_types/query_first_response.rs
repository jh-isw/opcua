// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    response_header::ResponseHeader,
    byte_string::ByteString,
    diagnostic_info::DiagnosticInfo,
    service_types::QueryDataSet,
    service_types::ParsingResult,
    service_types::ContentFilterResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct QueryFirstResponse {
    pub response_header: ResponseHeader,
    pub query_data_sets: Option<Vec<QueryDataSet>>,
    pub continuation_point: ByteString,
    pub parsing_results: Option<Vec<ParsingResult>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
    pub filter_result: ContentFilterResult,
}

impl MessageInfo for QueryFirstResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::QueryFirstResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<QueryFirstResponse> for QueryFirstResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.query_data_sets);
        size += self.continuation_point.byte_len();
        size += byte_len_array(&self.parsing_results);
        size += byte_len_array(&self.diagnostic_infos);
        size += self.filter_result.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.query_data_sets)?;
        size += self.continuation_point.encode(stream)?;
        size += write_array(stream, &self.parsing_results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        size += self.filter_result.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_options)?;
        let query_data_sets: Option<Vec<QueryDataSet>> = read_array(stream, decoding_options)?;
        let continuation_point = ByteString::decode(stream, decoding_options)?;
        let parsing_results: Option<Vec<ParsingResult>> = read_array(stream, decoding_options)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream, decoding_options)?;
        let filter_result = ContentFilterResult::decode(stream, decoding_options)?;
        Ok(QueryFirstResponse {
            response_header,
            query_data_sets,
            continuation_point,
            parsing_results,
            diagnostic_infos,
            filter_result,
        })
    }
}
