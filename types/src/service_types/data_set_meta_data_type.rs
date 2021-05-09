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
    localized_text::LocalizedText,
    guid::Guid,
    service_types::StructureDescription,
    service_types::EnumDescription,
    service_types::SimpleTypeDescription,
    service_types::FieldMetaData,
    service_types::ConfigurationVersionDataType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DataSetMetaDataType {
    pub namespaces: Option<Vec<UAString>>,
    pub structure_data_types: Option<Vec<StructureDescription>>,
    pub enum_data_types: Option<Vec<EnumDescription>>,
    pub simple_data_types: Option<Vec<SimpleTypeDescription>>,
    pub name: UAString,
    pub description: LocalizedText,
    pub fields: Option<Vec<FieldMetaData>>,
    pub data_set_class_id: Guid,
    pub configuration_version: ConfigurationVersionDataType,
}

impl BinaryEncoder<DataSetMetaDataType> for DataSetMetaDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.namespaces);
        size += byte_len_array(&self.structure_data_types);
        size += byte_len_array(&self.enum_data_types);
        size += byte_len_array(&self.simple_data_types);
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += byte_len_array(&self.fields);
        size += self.data_set_class_id.byte_len();
        size += self.configuration_version.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.namespaces)?;
        size += write_array(stream, &self.structure_data_types)?;
        size += write_array(stream, &self.enum_data_types)?;
        size += write_array(stream, &self.simple_data_types)?;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += write_array(stream, &self.fields)?;
        size += self.data_set_class_id.encode(stream)?;
        size += self.configuration_version.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let namespaces: Option<Vec<UAString>> = read_array(stream, decoding_options)?;
        let structure_data_types: Option<Vec<StructureDescription>> = read_array(stream, decoding_options)?;
        let enum_data_types: Option<Vec<EnumDescription>> = read_array(stream, decoding_options)?;
        let simple_data_types: Option<Vec<SimpleTypeDescription>> = read_array(stream, decoding_options)?;
        let name = UAString::decode(stream, decoding_options)?;
        let description = LocalizedText::decode(stream, decoding_options)?;
        let fields: Option<Vec<FieldMetaData>> = read_array(stream, decoding_options)?;
        let data_set_class_id = Guid::decode(stream, decoding_options)?;
        let configuration_version = ConfigurationVersionDataType::decode(stream, decoding_options)?;
        Ok(DataSetMetaDataType {
            namespaces,
            structure_data_types,
            enum_data_types,
            simple_data_types,
            name,
            description,
            fields,
            data_set_class_id,
            configuration_version,
        })
    }
}
