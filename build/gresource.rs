// Copyright 2023, (Feohr) Mohammed Rehaan and the ToadWriter contributors.
// SPDX-License-Identifier: BSD-3-Clause
//
// Redistribution and use in source and binary forms, with or without modification, are permitted
// provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
// conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list of
// conditions and the following disclaimer in the documentation and/or other materials provided
// with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be used to
// endorse or promote products derived from this software without specific prior written
// permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

use crate::config::get_config_table;
use std::{borrow::Cow, ffi::OsStr, fs::File, io::BufReader, path::PathBuf};
use xml::{
    attribute::OwnedAttribute, name::OwnedName, namespace::Namespace, reader::XmlEvent, writer,
    writer::EmitterConfig, EventReader,
};

/*▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇*/

pub fn compile_gresource() {
    let xml_resources = get_xml_resources("data").unwrap();
    for resource in xml_resources {
        parse_resources(resource);
    }
    // Compile resources
    glib_build_tools::compile_resources(
        &["data"],
        "data/resources.gresources.xml",
        "compiled.gresource",
    );
}

fn parse_resources(resource: PathBuf) {
    let file = File::open(resource.clone()).expect("Error while opening a resource file");
    let reader = BufReader::new(file);
    let parser = EventReader::new(reader);

    let name = resource.file_stem().expect("Error while getting file name");
    let parent = resource
        .parent()
        .expect("Error while getting the parent directories in \'data\'");
    let destination = parent.join(name);
    let destination = std::fs::File::create(destination).unwrap();

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(destination);

    for event in parser {
        let mut event = event.expect("Error while parsing xml");
        let Some(write_event) = as_writer_event(&mut event) else { break };

        writer
            .write(write_event)
            .expect("Error while writing Xml to file");
    }
}

fn as_writer_event<'a>(event: &'a mut XmlEvent) -> Option<writer::XmlEvent<'a>> {
    match event {
        XmlEvent::StartElement {
            ref name,
            ref mut attributes,
            ref namespace,
        } => Some(new_writer_start_element(name, attributes, namespace)),
        XmlEvent::Characters(ref mut value) => Some(new_writer_char_element(value)),
        XmlEvent::EndDocument => None,
        _ => Some(
            event
                .as_writer_event()
                .expect("Couldn't convert XmlEvent to writer Event"),
        ),
    }
}

#[inline]
fn new_writer_char_element<'a>(value: &'a mut String) -> writer::XmlEvent<'a> {
    let parsed_value = parse_attribute_value(value);
    writer::XmlEvent::Characters(parsed_value.as_str())
}

fn new_writer_start_element<'a>(
    name: &'a OwnedName,
    attributes: &'a mut Vec<OwnedAttribute>,
    namespace: &'a Namespace,
) -> writer::XmlEvent<'a> {
    let parsed_attributes = parse_attributes(attributes);

    writer::XmlEvent::StartElement {
        name: name.borrow(),
        attributes: parsed_attributes.iter().map(|a| a.borrow()).collect(),
        namespace: Cow::Borrowed(&namespace),
    }
}

fn parse_attributes(attributes: &mut Vec<OwnedAttribute>) -> &mut Vec<OwnedAttribute> {
    attributes.iter_mut().for_each(|item| {
        *item = OwnedAttribute {
            name: item.name.clone(),
            value: parse_attribute_value(&mut item.value).to_string(),
        };
    });

    return attributes;
}

fn parse_attribute_value(value: &mut String) -> &mut String {
    let config = get_config_table();

    *value = value.replace("@APP_NAME@", &config.vars.app_name);
    *value = value.replace("@APP_ID@", &config.vars.app_id);
    *value = value.replace("@PATH_ID@", &config.vars.path_id);
    *value = value.replace("@BASE_ID@", &config.vars.base_id);

    return value;
}

fn get_xml_resources(path: &str) -> std::io::Result<Vec<PathBuf>> {
    Ok(std::fs::read_dir(path)?
        .map(|file| file.unwrap().path())
        .filter(|path| {
            let Some(extension) = path.extension() else { return false };
            extension.eq(OsStr::new("in"))
        })
        .collect::<Vec<PathBuf>>())
}
