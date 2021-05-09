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
    string::UAString,
    service_types::enums::BrokerTransportQualityOfService,
};

#[derive(Debug, Clone, PartialEq)]
pub struct BrokerWriterGroupTransportDataType {
    pub queue_name: UAString,
    pub resource_uri: UAString,
    pub authentication_profile_uri: UAString,
    pub requested_delivery_guarantee: BrokerTransportQualityOfService,
}

impl BinaryEncoder<BrokerWriterGroupTransportDataType> for BrokerWriterGroupTransportDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.queue_name.byte_len();
        size += self.resource_uri.byte_len();
        size += self.authentication_profile_uri.byte_len();
        size += self.requested_delivery_guarantee.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.queue_name.encode(stream)?;
        size += self.resource_uri.encode(stream)?;
        size += self.authentication_profile_uri.encode(stream)?;
        size += self.requested_delivery_guarantee.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let queue_name = UAString::decode(stream, decoding_options)?;
        let resource_uri = UAString::decode(stream, decoding_options)?;
        let authentication_profile_uri = UAString::decode(stream, decoding_options)?;
        let requested_delivery_guarantee = BrokerTransportQualityOfService::decode(stream, decoding_options)?;
        Ok(BrokerWriterGroupTransportDataType {
            queue_name,
            resource_uri,
            authentication_profile_uri,
            requested_delivery_guarantee,
        })
    }
}
