// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::batch_get_item::BatchGetItemOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchGetItemOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchGetItemOutput::BatchGetItemOutput {
        Responses:
::std::rc::Rc::new(match &value.responses {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v,
    |e| ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&e.clone(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| dynamodb::conversions::attribute_value::to_dafny(v.clone())
,
)
,
)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 UnprocessedKeys:
::std::rc::Rc::new(match &value.unprocessed_keys {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| dynamodb::conversions::keys_and_attributes::to_dafny(v.clone())
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ConsumedCapacity: ::std::rc::Rc::new(match &value.consumed_capacity {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| dynamodb::conversions::consumed_capacity::to_dafny(e.clone())
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchGetItemOutput,
    >,
) -> crate::operation::batch_get_item::BatchGetItemOutput {
    crate::operation::batch_get_item::BatchGetItemOutput::builder()
        .set_responses(match (*dafny_value.Responses()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v| ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(v,
    |e| ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&e,
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v| dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
)
,
)
,
            )
        ),
    _ => None
}
)
 .set_unprocessed_keys(match (*dafny_value.UnprocessedKeys()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v| dynamodb::conversions::keys_and_attributes::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
 .set_consumed_capacity(match (*dafny_value.ConsumedCapacity()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| dynamodb::conversions::consumed_capacity::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
        .build()
        .unwrap()
}
