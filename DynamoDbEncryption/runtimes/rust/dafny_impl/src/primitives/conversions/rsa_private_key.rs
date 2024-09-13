// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::types::RsaPrivateKey,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::primitives::types::RsaPrivateKey,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey::RSAPrivateKey {
        lengthBits: value.length_bits.clone().unwrap(),
 pem: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.pem).Extract(),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::primitives::types::RsaPrivateKey>,
) -> ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
  crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey,
>>>{
    ::std::rc::Rc::new(match value {
        ::std::option::Option::None => crate::Wrappers::Option::None {},
        ::std::option::Option::Some(x) => crate::Wrappers::Option::Some {
            value: ::std::rc::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey,
    >,
) -> crate::primitives::types::RsaPrivateKey {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey,
) -> crate::primitives::types::RsaPrivateKey {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey::RSAPrivateKey {..} =>
            crate::primitives::types::RsaPrivateKey::builder()
                .set_length_bits(Some( dafny_value.lengthBits() .clone() ))
 .set_pem(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.pem().clone())))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::Wrappers::Option<::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::RSAPrivateKey,
    >>>,
) -> ::std::option::Option<crate::primitives::types::RsaPrivateKey> {
    match &*dafny_value {
        crate::Wrappers::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}