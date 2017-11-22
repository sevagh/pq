use std::collections::BTreeMap;
use serde_value::Value;
use inflector::cases::camelcase::to_camel_case;

/*
Traverse the deserialized JSON and enforce rules from the official protobuf JSON spec:
https://developers.google.com/protocol-buffers/docs/proto3#json

Rules enforced:

[✓] Keys should be lowerCamelCase
*/
pub fn spec_compliance(json: Value) -> Value {
    match json {
        Value::Map(ref map) => Value::Map(traverse_and_canonicalize_map_keys(map)),
        Value::Seq(x) => {
            let (kvmapseq, nonkvmapseq) = partition_vec_to_find_possible_map(x);
            if !kvmapseq.is_empty() {
                assert!(nonkvmapseq.is_empty());
                Value::Map(convert_seq_map_to_flattened_map(kvmapseq))
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

/*
Traverse a map recursively and convert the keys to lowerCamelCase.

The output of a nested protobuf will be a map, e.g.

Parent { "child" = Child { "foo_bar" : "baz" } } => Map("child" = Map("foo_bar": "baz"))

We want to iteratively search for all keys and convert them to lowerCamelCase

In the above example, `child` and `foo_bar` are converted to `child` and `fooBar`
*/
fn traverse_and_canonicalize_map_keys(map: &BTreeMap<Value, Value>) -> BTreeMap<Value, Value> {
    map.iter()
        .map(|(key, value)| match *key {
            Value::String(ref contents) => (
                Value::String(to_camel_case(contents)),
                spec_compliance(value.clone()),
            ),
            _ => (spec_compliance(key.clone()), spec_compliance(value.clone())),
        })
        .collect::<BTreeMap<Value, Value>>()
}

/*
Partition a Vec[Value, Value] into 2 Vecs:

* First result vec is if every element of the vec is a BTreeMap with 2 entries, `"key" = key, "value" = value

This is the result of deserializing a protobuf map (i.e. Parent { "child" = map{"foo_bar": "baz"}}) which is not a nested Protobuf message type.

We want to flatten this sequence of `Vec[Map(key=k1, value=v1), Map(key=k2, value=v2)` into a `Map(k1=v1, k2=v2)`. This is done in the function `convert_seq_map_to_flattened_map`

* Second result vec is every element of the vec which is not like the above
*/
fn partition_vec_to_find_possible_map(x: Vec<Value>) -> (Vec<Value>, Vec<Value>) {
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
    })
}

/*
Convert the aforementioned sequence of `Vec[Map(key=k1, value=v1), Map(key=k2, value=v2)` into a `Map(k1=v1, k2=v2)`
*/
fn convert_seq_map_to_flattened_map(x: Vec<Value>) -> BTreeMap<Value, Value> {
    x.into_iter()
        .map(|elem| {
            if let Value::Map(ref map) = elem {
                let key = map.get(&Value::String(String::from("key")));
                let value = map.get(&Value::String(String::from("value")));
                if let Some(&Value::Option(Some(box ref k))) = key {
                    if let Some(v) = value {
                        return (k.clone(), spec_compliance(v.clone()));
                    }
                }
            }
            panic!("What the fuck am I deserializing right now?")
        })
        .collect::<BTreeMap<_, _>>()
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
