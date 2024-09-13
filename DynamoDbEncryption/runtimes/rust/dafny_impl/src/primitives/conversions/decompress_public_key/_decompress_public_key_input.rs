// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::primitives::operation::decompress_public_key::DecompressPublicKeyInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput::DecompressPublicKeyInput {
        compressedPublicKey: crate::ddb::standard_library_conversions::oblob_to_dafny(&value.compressed_public_key).Extract(),
 eccCurve: crate::primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
    >,
) -> crate::primitives::operation::decompress_public_key::DecompressPublicKeyInput {
    crate::primitives::operation::decompress_public_key::DecompressPublicKeyInput::builder()
        .set_compressed_public_key(Some(crate::ddb::standard_library_conversions::blob_from_dafny(dafny_value.compressedPublicKey().clone())))
 .set_ecc_curve(Some( crate::primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
        .build()
        .unwrap()
}
