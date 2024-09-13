#!/bin/bash

cd $( dirname -- "${BASH_SOURCE[0]}" )

rm -rf src/material_providers
cp -r ../../../../submodules/MaterialProviders/AwsCryptographicMaterialProviders/runtimes/rust/src/ src/material_providers

rm -f src/material_providers/implementation_from_dafny.rs
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::_Wrappers_Compile/crate::Wrappers/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::r#_Wrappers_Compile/crate::Wrappers/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::client/crate::material_providers::client/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::conversions/crate::material_providers::conversions/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::error/crate::material_providers::error/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::operation/crate::material_providers::operation/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::types/crate::material_providers::types/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::wrapped/crate::material_providers::wrapped/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::standard_library_conversions/crate::ddb::standard_library_conversions/'
find src/material_providers -name '*.rs' | xargs perl -i -pe 's/crate::r#software::amazon::cryptography::material_providers::internaldafny::wrapped::/crate::software::amazon::cryptography::materialproviders::internaldafny::wrapped::/'
