// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ExpectedAttributeValue,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue::ExpectedAttributeValue {
        Value: ::std::rc::Rc::new(match &value.value {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::attribute_value::to_dafny(x) },
    None => crate::Wrappers::Option::None { }
})
,
 Exists: crate::ddb::standard_library_conversions::obool_to_dafny(&value.exists),
 ComparisonOperator: ::std::rc::Rc::new(match &value.comparison_operator {
    Some(x) => crate::Wrappers::Option::Some { value: crate::ddb::conversions::comparison_operator::to_dafny(x.clone()) },
    None => crate::Wrappers::Option::None { }
})
,
 AttributeValueList: ::std::rc::Rc::new(match &value.attribute_value_list {
    Some(x) => crate::Wrappers::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::ddb::conversions::attribute_value::to_dafny(e)
,
        )
    },
    None => crate::Wrappers::Option::None {}
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ExpectedAttributeValue,
    >,
) -> aws_sdk_dynamodb::types::ExpectedAttributeValue {
    aws_sdk_dynamodb::types::ExpectedAttributeValue::builder()
          .set_value(match (*dafny_value.Value()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(crate::ddb::conversions::attribute_value::from_dafny(value.clone())),
    _ => None,
}
)
 .set_exists(crate::ddb::standard_library_conversions::obool_from_dafny(dafny_value.Exists().clone()))
 .set_comparison_operator(match &**dafny_value.ComparisonOperator() {
    crate::Wrappers::Option::Some { value } => Some(
        crate::ddb::conversions::comparison_operator::from_dafny(value)
    ),
    _ => None,
}
)
 .set_attribute_value_list(match (*dafny_value.AttributeValueList()).as_ref() {
    crate::Wrappers::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e| crate::ddb::conversions::attribute_value::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()

}
