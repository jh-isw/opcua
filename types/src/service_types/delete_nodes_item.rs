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
    node_id::NodeId,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteNodesItem {
    pub node_id: NodeId,
    pub delete_target_references: bool,
}

impl MessageInfo for DeleteNodesItem {
    fn object_id(&self) -> ObjectId {
        ObjectId::DeleteNodesItem_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DeleteNodesItem> for DeleteNodesItem {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.delete_target_references.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.delete_target_references.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream, decoding_options)?;
        let delete_target_references = bool::decode(stream, decoding_options)?;
        Ok(DeleteNodesItem {
            node_id,
            delete_target_references,
        })
    }
}
