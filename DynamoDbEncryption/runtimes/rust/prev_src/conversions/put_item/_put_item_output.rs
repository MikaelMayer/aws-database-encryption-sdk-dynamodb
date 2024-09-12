// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::put_item::PutItemOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutput::PutItemOutput {
        Attributes:
::std::rc::Rc::new(match &value.attributes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| dynamodb::conversions::attribute_value::to_dafny(v.clone())
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ConsumedCapacity: ::std::rc::Rc::new(match &value.consumed_capacity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::consumed_capacity::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ItemCollectionMetrics: ::std::rc::Rc::new(match &value.item_collection_metrics {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::item_collection_metrics::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::PutItemOutput,
    >,
) -> crate::operation::put_item::PutItemOutput {
    crate::operation::put_item::PutItemOutput::builder()
        .set_attributes(match (*dafny_value.Attributes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v| dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
 .set_consumed_capacity(match (*dafny_value.ConsumedCapacity()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::consumed_capacity::from_dafny(value.clone())),
    _ => None,
}
)
 .set_item_collection_metrics(match (*dafny_value.ItemCollectionMetrics()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::item_collection_metrics::from_dafny(value.clone())),
    _ => None,
}
)
        .build()
        .unwrap()
}
