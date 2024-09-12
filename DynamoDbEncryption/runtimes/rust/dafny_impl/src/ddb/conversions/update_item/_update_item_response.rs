// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::update_item::UpdateItemOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateItemOutput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::UpdateItemOutput::UpdateItemOutput {
        Attributes:
::std::rc::Rc::new(match &value.attributes {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| crate::ddb::conversions::attribute_value::to_dafny(v)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
 ConsumedCapacity: ::std::rc::Rc::new(match &value.consumed_capacity {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::consumed_capacity::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 ItemCollectionMetrics: ::std::rc::Rc::new(match &value.item_collection_metrics {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::item_collection_metrics::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
    })
}
 
