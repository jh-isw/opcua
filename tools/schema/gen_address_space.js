var _ = require("lodash");
var fs = require("fs");
var xml2js = require("xml2js");

var settings = require("./settings");

// THIS file will generate the address space

var node_set =
    [
        {
            name: "Opc.Ua.NodeSet2.Part3.xml", module: "nodeset_part_3"
        },
        {
            name: "Opc.Ua.NodeSet2.Part4.xml", module: "nodeset_part_4"
        },
        {
            name: "Opc.Ua.NodeSet2.Part5.xml", module: "nodeset_part_5"
        },
        {
            name: "Opc.Ua.NodeSet2.Part8.xml", module: "nodeset_part_8"
        },
        {
            name: "Opc.Ua.NodeSet2.Part9.xml", module: "nodeset_part_9"
        },
        {
            name: "Opc.Ua.NodeSet2.Part10.xml", module: "nodeset_part_10"
        },
        {
            name: "Opc.Ua.NodeSet2.Part11.xml", module: "nodeset_part_11"
        },
        {
            name: "Opc.Ua.NodeSet2.Part12.xml", module: "nodeset_part_12"
        },
        {
            name: "Opc.Ua.NodeSet2.Part13.xml", module: "nodeset_part_13"
        },
        {
            name: "Opc.Ua.NodeSet2.Part14.xml", module: "nodeset_part_14"
        },
        {
            name: "Opc.Ua.NodeSet2.Part999.xml", module: "nodeset_part_999"
        }
    ];

///////////////////////////////////////////////////////////////////////////////
// Parse all XML inputs into data and place it on the node sets above

var parser = new xml2js.Parser();
_.each(node_set, function (ns) {
    fs.readFile(`${settings.schema_dir}/${ns.name}`, function (err, data) {
        parser.parseString(data, function (err, result) {
            ns.data = result;
            console.log(`Generating code for module ${ns.module}`);
            generate_node_set(ns);
        });
    });
});

///////////////////////////////////////////////////////////////////////////////
// All files to be created under server/src/address_space/generated/
function generate_node_set(ns) {
    var contents = `// This file was autogenerated from ${ns.name}
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::*;
#[allow(unused_imports)]
use address_space::types::*;

`;

// Parse the xml
// Create a file with rs
//   in that file, create a populate_address_space method

    contents += "#[allow(unused_variables)]\n";
    contents += `pub fn populate_address_space(address_space: &mut AddressSpace) {\n`;

    var indent = "    ";
    var nodes = ns.data["UANodeSet"];
    if (_.has(nodes, "UAObject")) {
        _.each(nodes["UAObject"], function (value) {
            contents += insert_node(indent, "Object", value, "Object::new_node(&node_id, browse_name, display_name, description)");
        });
    }
    if (_.has(nodes, "UAObjectType")) {
        _.each(nodes["UAObjectType"], function (value) {
            var is_abstract = _.has(value["$"], "IsAbstract") && value["$"]["IsAbstract"] === "true";
            contents += insert_node(indent, "ObjectType", value, `ObjectType::new_node(&node_id, browse_name, display_name, description, ${is_abstract})`);
        });
    }
    if (_.has(nodes, "UADataType")) {
        _.each(nodes["UADataType"], function (value) {
            var is_abstract = _.has(value["$"], "IsAbstract") && value["$"]["IsAbstract"] === "true";
            contents += insert_node(indent, "DataType", value, `DataType::new_node(&node_id, browse_name, display_name, description, ${is_abstract})`);
        });
    }
    if (_.has(nodes, "UAReferenceType")) {
        _.each(nodes["UAReferenceType"], function (value) {
            var is_abstract = _.has(value["$"], "IsAbstract") && value["$"]["IsAbstract"] === "true";
            var inverse_name = _.has(value, "InverseName") ? `Some(LocalizedText::new("", "${value["InverseName"][0]}"))` : "None";
            var symmetric = _.has(value["$"], "Symmetric") && value["$"]["Symmetric"] === "true";
            contents += insert_node(indent, "DataType", value, `ReferenceType::new_node(&node_id, browse_name, display_name, description, ${inverse_name}, ${symmetric}, ${is_abstract})`);
        });
    }
    if (_.has(nodes, "UAVariable")) {
        _.each(nodes["UAVariable"], function (value) {
            var data_type = "DataTypeId::Boolean";
            if (_.has(value["$"], "DataType")) {
                data_type = value["$"]["DataType"];
                if (data_type.startsWith("i=")) {
                    data_type = `DataTypeId::from_u64(${data_type.substr(2)}u64).unwrap()`;
                }
                else {
                    data_type = `DataTypeId::${data_type}`;
                }
            }
            else {
                console.log("UAVariable has no data type???");
            }
            var data_value = "DataValue::null()";
            contents += insert_node(indent, "Variable", value, `Variable::new_node(&node_id, browse_name, display_name, description, ${data_type}, ${data_value})`);
        });
    }
    if (_.has(nodes, "UAVariableType")) {
        _.each(nodes["UAVariableType"], function (value) {
            var is_abstract = _.has(value["$"], "IsAbstract") && value["$"]["IsAbstract"] === "true";
            var value_rank = _.has(value["$"], "ValueRank") ? value["$"]["ValueRank"] : -1;
            contents += insert_node(indent, "VariableType", value, `VariableType::new_node(&node_id, browse_name, display_name, description, ${is_abstract}, ${value_rank})`);
        });
    }
    if (_.has(nodes, "UAMethod")) {
        _.each(nodes["UAMethod"], function (value) {
            var is_abstract = _.has(value["$"], "IsAbstract") && value["$"]["IsAbstract"] === "true";
            var executable = false; // TODO
            var user_executable = false; // TODO
            contents += insert_node(indent, "Method", value, `Method::new_node(&node_id, browse_name, display_name, description, ${is_abstract}, ${executable}, ${user_executable})`);
        });
    }

    contents += `}\n`;

    settings.write_to_file(`${settings.rs_address_space_dir}/${ns.module}.rs`, contents);
}

///////////////////////////////////////////////////////////////////////////////
// Create the mod.rs

var mod_contents = `// This file was autogenerated
// DO NOT EDIT THIS FILE

use address_space::{AddressSpace};

`;

// use each part
_.each(node_set, function (ns) {
    mod_contents += `mod ${ns.module};\n`
});
mod_contents += `\n`;

// in a populate_address_space method
mod_contents += `/// Populates the address space with all defined node sets
pub fn populate_address_space(address_space: &mut AddressSpace) {\n`;

_.each(node_set, function (ns) {
    mod_contents += `    ${ns.module}::populate_address_space(address_space);\n`
});

mod_contents += `}\n`;

settings.write_to_file(`${settings.rs_address_space_dir}/mod.rs`, mod_contents);

function node_id_ctor(snippet) {
    // This turns a snippet like "i=2015" into a node id
    return `NodeId::new_numeric(0, ${snippet.substr(2)})`;
}

function insert_node(indent, node_type, value, node_ctor) {
    var contents = `${indent}{\n`;
    indent += "    ";

    contents += `${indent}// ${node_type}\n`;

    var browse_name = _.has(value["$"], "BrowseName") ? value["$"]["BrowseName"] : "";
    contents += `${indent}let browse_name = "${browse_name}";\n`;
    var display_name = _.has(value, "DisplayName") ? value["DisplayName"][0] : "";
    contents += `${indent}let display_name = "${display_name}";\n`;
    var description = _.has(value, "Description") ? value["Description"][0] : "";
    contents += `${indent}let description = "${description}";\n`;

    contents += `${indent}let node_id = ${node_id_ctor(value["$"]["NodeId"])};\n`;
    contents += `${indent}let node = ${node_ctor};\n`;
    contents += `${indent}address_space.insert(node);\n`;

    // Process references
    if (_.has(value, "References")) {
        contents += insert_references(indent, value["References"][0])
    }

    // Organizes
    if (_.has(value["$"], "ParentNodeId")) {
        var parent_node_id = node_id_ctor(value["$"]["ParentNodeId"]);
        contents += `${indent}address_space.add_organizes(&${parent_node_id}, &node_id);\n`;
    }

    // Process definitions
    if (_.has(value, "Definition")) {
        // TODO process Fields
    }

    // Process values
    if (_.has(value, "Value")) {
        // TODO process ListOfLocalizedText
        // TODO process ListOfExtensionObject
    }

    // Process InverseName
    indent = indent.substr(0, indent.length - 4);
    contents += `${indent}}\n`;

    return contents;
}

function insert_references(indent, references) {
    var contents = "";
    if (_.has(references, "Reference")) {
        _.each(references["Reference"], function (reference) {
            // Test if the reference is forward or reverse
            var reference_type = reference["$"]["ReferenceType"];
            var reference_id = node_id_ctor(reference["_"]);
            var is_forward = !_.has(reference["$"], "IsForward") || reference["$"]["IsForward"] === "true";

            var node_id1 = is_forward ? "node_id" : reference_id;
            var node_id2 = is_forward ? reference_id : "node_id";

            if (reference_type.startsWith("i=")) {
                // TODO
                contents += `${indent}// address_space.insert_reference(&${node_id1}, &${node_id2}, ReferenceTypeId::${reference_type});\n`
            }
            else {
                contents += `${indent}address_space.insert_reference(&${node_id1}, &${node_id2}, ReferenceTypeId::${reference_type});\n`
            }
        });
    }
    return contents;
}