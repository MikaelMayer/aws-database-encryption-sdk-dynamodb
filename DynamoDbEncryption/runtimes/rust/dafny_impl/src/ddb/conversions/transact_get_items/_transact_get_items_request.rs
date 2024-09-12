// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::transact_get_items::TransactGetItemsInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsInput::TransactGetItemsInput {
        TransactItems: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&value.transact_items.clone().unwrap(),
    |e| crate::ddb::conversions::transact_get_item::to_dafny(e)
,
)
,
 ReturnConsumedCapacity: ::std::rc::Rc::new(match &value.return_consumed_capacity {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::return_consumed_capacity::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::TransactGetItemsInput,
    >,
    client: aws_sdk_dynamodb::Client,
) -> aws_sdk_dynamodb::operation::transact_get_items::builders::TransactGetItemsFluentBuilder {
    client.transact_get_items()
          .set_transact_items(Some( ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(dafny_value.TransactItems(),
    |e| crate::ddb::conversions::transact_get_item::from_dafny(e.clone())
,
)
 ))
 .set_return_consumed_capacity(match &**dafny_value.ReturnConsumedCapacity() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::return_consumed_capacity::from_dafny(value)
    ),
    _ => None,
}
)
}
