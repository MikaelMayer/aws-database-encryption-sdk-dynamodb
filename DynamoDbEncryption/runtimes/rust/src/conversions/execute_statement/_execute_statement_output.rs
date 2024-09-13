// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::operation::execute_statement::ExecuteStatementOutput,
) -> ::std::rc::Rc<
    crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteStatementOutput,
>{
    ::std::rc::Rc::new(crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteStatementOutput::ExecuteStatementOutput {
        Items: ::std::rc::Rc::new(match &value.items {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&e.clone(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| dynamodb::conversions::attribute_value::to_dafny(v.clone())
,
)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 NextToken: crate::standard_library_conversions::ostring_to_dafny(&value.next_token),
 ConsumedCapacity: ::std::rc::Rc::new(match &value.consumed_capacity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: dynamodb::conversions::consumed_capacity::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 LastEvaluatedKey:
::std::rc::Rc::new(match &value.last_evaluated_key {
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
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#aws::cryptography::dbencryptionsdk::dynamodb::transforms::internaldafny::types::ExecuteStatementOutput,
    >,
) -> crate::operation::execute_statement::ExecuteStatementOutput {
    crate::operation::execute_statement::ExecuteStatementOutput::builder()
        .set_items(match (*dafny_value.Items()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&e,
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v| dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
)
,
            )
        ),
    _ => None
}
)
 .set_next_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.NextToken().clone()))
 .set_consumed_capacity(match (*dafny_value.ConsumedCapacity()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(dynamodb::conversions::consumed_capacity::from_dafny(value.clone())),
    _ => None,
}
)
 .set_last_evaluated_key(match (*dafny_value.LastEvaluatedKey()).as_ref() {
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
        .build()
        .unwrap()
}