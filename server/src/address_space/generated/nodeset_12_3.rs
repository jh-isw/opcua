// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock
// This file was autogenerated from Opc.Ua.NodeSet2.Part12.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::{convert::TryFrom, str::FromStr};

#[allow(unused_imports)]
use crate::{
    address_space::{EventNotifier, types::*},
    prelude::{DataTypeId, ExtensionObject, LocalizedText, NodeId, ReferenceTypeId, service_types::Argument, UAString, Variant, VariantTypeId}
};

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_method_1(address_space);
    add_method_2(address_space);
    add_method_3(address_space);
    add_method_4(address_space);
    add_method_5(address_space);
    add_method_6(address_space);
    add_method_7(address_space);
    add_method_8(address_space);
    add_method_9(address_space);
    add_method_10(address_space);
    add_method_11(address_space);
    add_method_12(address_space);
    add_method_13(address_space);
    add_method_14(address_space);
    add_method_15(address_space);
    add_method_16(address_space);
    add_method_17(address_space);
    add_method_18(address_space);
    add_method_19(address_space);
    add_method_20(address_space);
    add_method_21(address_space);
    add_method_22(address_space);
    add_method_23(address_space);
    add_method_24(address_space);
    add_method_25(address_space);
    add_method_26(address_space);
    add_method_27(address_space);
    add_method_28(address_space);
}

fn add_method_1(address_space: &mut AddressSpace) {
    // Method
    let name = "Close";
    let node_id = NodeId::new(0, 13892);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13893), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13883), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_2(address_space: &mut AddressSpace) {
    // Method
    let name = "Read";
    let node_id = NodeId::new(0, 13894);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13895), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13896), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13883), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_3(address_space: &mut AddressSpace) {
    // Method
    let name = "Write";
    let node_id = NodeId::new(0, 13897);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13898), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13883), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_4(address_space: &mut AddressSpace) {
    // Method
    let name = "GetPosition";
    let node_id = NodeId::new(0, 13899);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13900), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13901), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13883), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_5(address_space: &mut AddressSpace) {
    // Method
    let name = "SetPosition";
    let node_id = NodeId::new(0, 13902);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13903), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13883), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_6(address_space: &mut AddressSpace) {
    // Method
    let name = "OpenWithMasks";
    let node_id = NodeId::new(0, 13905);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13906), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13907), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13883), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_7(address_space: &mut AddressSpace) {
    // Method
    let name = "Open";
    let node_id = NodeId::new(0, 13923);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13924), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13925), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13917), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_8(address_space: &mut AddressSpace) {
    // Method
    let name = "Close";
    let node_id = NodeId::new(0, 13926);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13927), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13917), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_9(address_space: &mut AddressSpace) {
    // Method
    let name = "Read";
    let node_id = NodeId::new(0, 13928);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13929), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13930), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13917), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_10(address_space: &mut AddressSpace) {
    // Method
    let name = "Write";
    let node_id = NodeId::new(0, 13931);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13932), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13917), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_11(address_space: &mut AddressSpace) {
    // Method
    let name = "GetPosition";
    let node_id = NodeId::new(0, 13933);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13934), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13935), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13917), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_12(address_space: &mut AddressSpace) {
    // Method
    let name = "SetPosition";
    let node_id = NodeId::new(0, 13936);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13937), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13917), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_13(address_space: &mut AddressSpace) {
    // Method
    let name = "OpenWithMasks";
    let node_id = NodeId::new(0, 13939);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13940), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13941), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13917), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_14(address_space: &mut AddressSpace) {
    // Method
    let name = "Open";
    let node_id = NodeId::new(0, 13958);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13959), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13960), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13952), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_15(address_space: &mut AddressSpace) {
    // Method
    let name = "Close";
    let node_id = NodeId::new(0, 13961);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13962), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13952), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_16(address_space: &mut AddressSpace) {
    // Method
    let name = "Read";
    let node_id = NodeId::new(0, 13963);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13964), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13965), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13952), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_17(address_space: &mut AddressSpace) {
    // Method
    let name = "Write";
    let node_id = NodeId::new(0, 13966);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13967), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13952), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_18(address_space: &mut AddressSpace) {
    // Method
    let name = "GetPosition";
    let node_id = NodeId::new(0, 13968);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13969), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13970), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13952), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_19(address_space: &mut AddressSpace) {
    // Method
    let name = "SetPosition";
    let node_id = NodeId::new(0, 13971);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13972), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13952), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_20(address_space: &mut AddressSpace) {
    // Method
    let name = "OpenWithMasks";
    let node_id = NodeId::new(0, 13974);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 13975), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 13976), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 13952), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_21(address_space: &mut AddressSpace) {
    // Method
    let name = "UpdateCertificate";
    let node_id = NodeId::new(0, 12616);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 12617), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12618), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12581), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_22(address_space: &mut AddressSpace) {
    // Method
    let name = "ApplyChanges";
    let node_id = NodeId::new(0, 12734);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12581), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_23(address_space: &mut AddressSpace) {
    // Method
    let name = "CreateSigningRequest";
    let node_id = NodeId::new(0, 12731);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 12732), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12733), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12581), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_24(address_space: &mut AddressSpace) {
    // Method
    let name = "GetRejectedList";
    let node_id = NodeId::new(0, 12775);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 12776), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12581), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_25(address_space: &mut AddressSpace) {
    // Method
    let name = "CreateCredential";
    let node_id = NodeId::new(0, 17522);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 17523), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 17524), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 17496), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_26(address_space: &mut AddressSpace) {
    // Method
    let name = "GetEncryptingKey";
    let node_id = NodeId::new(0, 17534);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 17535), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 17536), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 18001), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_27(address_space: &mut AddressSpace) {
    // Method
    let name = "UpdateCredential";
    let node_id = NodeId::new(0, 18006);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 18007), &ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 18001), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

fn add_method_28(address_space: &mut AddressSpace) {
    // Method
    let name = "DeleteCredential";
    let node_id = NodeId::new(0, 18008);
    let node = Method::new(&node_id, name, name, true, true);
    let _ = address_space.insert(node, Some(&[
        (&NodeId::new(0, 80), &ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 18001), &ReferenceTypeId::HasComponent, ReferenceDirection::Inverse),
    ]));
}

