// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::batch_write_item::BatchWriteItemInput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchWriteItemInput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchWriteItemInput::BatchWriteItemInput {
        RequestItems: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&value.request_items.clone().unwrap(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&v,
    |e| dynamodb::conversions::write_request::to_dafny(e.clone())
,
)
,
)
,
 ReturnConsumedCapacity: ::std::rc::Rc::new(match &value.return_consumed_capacity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::return_consumed_capacity::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReturnItemCollectionMetrics: ::std::rc::Rc::new(match &value.return_item_collection_metrics {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::return_item_collection_metrics::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::BatchWriteItemInput,
    >,
) -> crate::operation::batch_write_item::BatchWriteItemInput {
    crate::operation::batch_write_item::BatchWriteItemInput::builder()
        .set_request_items(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&dafny_value.RequestItems(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v| ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(v,
    |e| dynamodb::conversions::write_request::from_dafny(e.clone())
,
)
,
)
 ))
 .set_return_consumed_capacity(match &**dafny_value.ReturnConsumedCapacity() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        dynamodb::conversions::return_consumed_capacity::from_dafny(value)
    ),
    _ => None,
}
)
 .set_return_item_collection_metrics(match &**dafny_value.ReturnItemCollectionMetrics() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        dynamodb::conversions::return_item_collection_metrics::from_dafny(value)
    ),
    _ => None,
}
)
        .build()
        .unwrap()
}
