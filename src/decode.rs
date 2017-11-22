use std::io::Write;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use serde_json::ser::Serializer;
use serde_protobuf::de::Deserializer;
use serde_protobuf::descriptor::Descriptors;
use serde_value::Value;
use protobuf::CodedInputStream;
use protobuf::descriptor::FileDescriptorSet;
use inflector::cases::camelcase::to_camel_case;
use formatter::CustomFormatter;
use errors::*;

pub struct PqrsDecoder<'a> {
    pub descriptors: Descriptors,
    pub message_type: &'a str,
    spec_compliant: bool,
}

impl<'a> PqrsDecoder<'a> {
    pub fn new(
        loaded_descs: Vec<FileDescriptorSet>,
        msgtype: &str,
        compliant: bool,
    ) -> Result<PqrsDecoder> {
        let mut descriptors = Descriptors::new();
        for fdset in loaded_descs {
            descriptors.add_file_set_proto(&fdset);
        }
        descriptors.resolve_refs();
        Ok(PqrsDecoder {
            descriptors: descriptors,
            message_type: msgtype,
            spec_compliant: compliant,
        })
    }

    pub fn decode_message(&self, data: &[u8], out: &mut Write, is_tty: bool) -> Result<()> {
        let stream = CodedInputStream::from_bytes(data);
        let mut deserializer = Deserializer::for_named_message(
            &self.descriptors,
            &(format!(".{}", self.message_type)),
            stream,
        ).expect("Couldn't initialize deserializer");
        let decoded_json = Value::deserialize(&mut deserializer).chain_err(
            || "Deser error",
        )?;
        let compliant_json = if self.spec_compliant {
            spec_compliance(decoded_json)
        } else {
            decoded_json
        };
        Ok(compliant_json
            .serialize(&mut Serializer::with_formatter(
                out,
                CustomFormatter::new(is_tty),
            ))
            .chain_err(|| "Ser error")?)
    }
}

/*
Traverse the deserialized JSON and enforce rules from the official protobuf JSON spec:
https://developers.google.com/protocol-buffers/docs/proto3#json

Rules enforced:

[✓] Keys should be lowerCamelCase
*/
fn spec_compliance(json: Value) -> Value {
    match json {
        Value::Map(ref map) => {
            Value::Map(
                map.iter()
                    .map(|(key, value)| match *key {
                        Value::String(ref contents) => (
                            Value::String(to_camel_case(contents)),
                            spec_compliance(value.clone()),
                        ),
                        _ => (spec_compliance(key.clone()), spec_compliance(value.clone())),
                    })
                    .collect::<BTreeMap<Value, Value>>(),
            )
        }
        Value::Seq(x) => {
            let (kvmapseq, nonkvmapseq): (Vec<Value>, Vec<Value>) =
                x.into_iter().partition(|elem| {
                    if let Value::Map(ref map) = *elem {
                        if map.len() == 2 {
                            // possibly key/value map case
                            let key = map.get(&Value::String(String::from("key")));
                            let value = map.get(&Value::String(String::from("value")));
                            if let Some(&Value::Option(Some(box ref _k))) = key {
                                if let Some(_v) = value {
                                    return true;
                                }
                            }
                        }
                    }
                    false
                });
            if !kvmapseq.is_empty() {
                assert!(nonkvmapseq.is_empty());
                Value::Map(
                    kvmapseq
                        .into_iter()
                        .map(|elem| {
                            if let Value::Map(ref map) = elem {
                                let key = map.get(&Value::String(String::from("key")));
                                let value = map.get(&Value::String(String::from("value")));
                                if let Some(&Value::Option(Some(box ref k))) = key {
                                    if let Some(v) = value {
                                        return (k.clone(), v.clone());
                                    }
                                }
                            }
                            panic!("What the fuck am I deserializing right now?")
                        })
                        .collect::<BTreeMap<_, _>>(),
                )
            } else {
                Value::Seq(
                    nonkvmapseq
                        .into_iter()
                        .map(spec_compliance)
                        .collect::<Vec<_>>(),
                )
            }
        }
        Value::Option(Some(box x)) |
        Value::Newtype(box x) => spec_compliance(x),
        x => x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_value::Value;

    #[test]
    fn test_json_spec_compliance_lower_camel_case_keys() {
        let snake_case_key = Value::String(String::from("snake_case_key"));
        let lower_camel_case_key = Value::String(String::from("snakeCaseKey"));

        let mut map = BTreeMap::new();
        map.insert(snake_case_key, Value::String(String::from("val")));

        let noncompliant_json = Value::Map(map);

        let compliant_json = spec_compliance(noncompliant_json);

        match compliant_json {
            Value::Map(compliant_map) => {
                assert_eq!(1, compliant_map.len());
                assert_eq!(
                    &Value::String(String::from("val")),
                    compliant_map.get(&lower_camel_case_key).ok_or(0).expect(
                        "Test failure",
                    )
                );
            }
            _ => assert!(false),
        }
    }
}
