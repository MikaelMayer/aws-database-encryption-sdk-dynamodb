// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::material_providers::types::keyring::KeyringRef,
) -> ::dafny_runtime::Object<
  dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyring,
> {
  let wrap = KeyringWrapper {
      obj: value.clone(),
  };
  let inner = ::std::rc::Rc::new(::std::cell::UnsafeCell::new(wrap));
  ::dafny_runtime::Object (Some(inner) )
}

pub struct KeyringWrapper {
  obj: crate::material_providers::types::keyring::KeyringRef,
}

impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for KeyringWrapper {
  ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::ISimpleResource,
    >,
) -> crate::material_providers::types::keyring::KeyringRef {
    let wrap = IKeyringDafnyWrapper {
        obj: dafny_value.clone(),
    };
    crate::material_providers::types::keyring::KeyringRef {
      inner: ::std::rc::Rc::new(::std::cell::RefCell::new(wrap))
    }
}

#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IKeyringDafnyWrapper {
  pub(crate) obj: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyring,
  >,
}


impl crate::software::amazon::cryptography::keystore::internaldafny::types::IKeyring
  for KeyringWrapper
{
  fn r#_OnEncrypt_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::OnEncryptInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::OnEncryptOutput,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::on_encrypt::_on_encrypt_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().on_encrypt(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::on_encrypt::_on_encrypt_output::to_dafny(
                  x,
              ),
          },
          Err(x) => crate::Wrappers::Result::Failure {
              error: crate::material_providers::conversions::error::to_dafny(x),
          },
      };
      ::std::rc::Rc::new(result)
  }
}

impl crate::material_providers::types::keyring::Keyring for IKeyringDafnyWrapper {
  fn on_encrypt(
      &mut self,
      input: crate::material_providers::operation::on_encrypt::OnEncryptInput,
  ) -> Result<
      crate::material_providers::operation::on_encrypt::OnEncryptOutput,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::on_encrypt::_on_encrypt_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).OnEncrypt(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::on_encrypt::_on_encrypt_output::from_dafny(
                  inner_result.value().clone(),
              ),
          )
      } else {
          Err(crate::material_providers::conversions::error::from_dafny(
              inner_result.error().clone(),
          ))
      }
  }
}


impl crate::software::amazon::cryptography::keystore::internaldafny::types::IKeyring
  for KeyringWrapper
{
  fn r#_OnDecrypt_k(
      &mut self,
      input: &::std::rc::Rc<
      crate::r#software::amazon::cryptography::keystore::internaldafny::types::OnDecryptInput,
      >,
  ) -> ::std::rc::Rc<
      crate::Wrappers::Result<
          ::std::rc::Rc<
          crate::r#software::amazon::cryptography::keystore::internaldafny::types::OnDecryptOutput,
          >,
          ::std::rc::Rc<crate::r#software::amazon::cryptography::keystore::internaldafny::types::Error>,
      >,
  >
  {
      let inner_input =
          crate::material_providers::conversions::on_decrypt::_on_decrypt_input::from_dafny(
              input.clone(),
          );
      let inner_result = self.obj.inner.borrow_mut().on_decrypt(inner_input);
      let result = match inner_result {
          Ok(x) => crate::Wrappers::Result::Success {
              value: crate::material_providers::conversions::on_decrypt::_on_decrypt_output::to_dafny(
                  x,
              ),
          },
          Err(x) => crate::Wrappers::Result::Failure {
              error: crate::material_providers::conversions::error::to_dafny(x),
          },
      };
      ::std::rc::Rc::new(result)
  }
}

impl crate::material_providers::types::keyring::Keyring for IKeyringDafnyWrapper {
  fn on_decrypt(
      &mut self,
      input: crate::material_providers::operation::on_decrypt::OnDecryptInput,
  ) -> Result<
      crate::material_providers::operation::on_decrypt::OnDecryptOutput,
      crate::material_providers::types::error::Error,
  > {
      let inner_input =
          crate::material_providers::conversions::on_decrypt::_on_decrypt_input::to_dafny(input);
      let inner_result = ::dafny_runtime::md!(self.obj.clone()).OnDecrypt(&inner_input);
      if matches!(
          inner_result.as_ref(),
          crate::Wrappers::Result::Success { .. }
      ) {
          Ok(
              crate::material_providers::conversions::on_decrypt::_on_decrypt_output::from_dafny(
                  inner_result.value().clone(),
              ),
          )
      } else {
          Err(crate::material_providers::conversions::error::from_dafny(
              inner_result.error().clone(),
          ))
      }
  }
}
