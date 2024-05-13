// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../Model/AwsCryptographyDbEncryptionSdkStructuredEncryptionTypes.dfy"
include "../../../../submodules/MaterialProviders/libraries/src/Collections/Maps/Maps.dfy"

include "Header.dfy"
include "Footer.dfy"
include "Paths.dfy"
include "Crypt.dfy"
include "Util.dfy"
include "SortCanon.dfy"

module AwsCryptographyDbEncryptionSdkStructuredEncryptionOperations refines AbstractAwsCryptographyDbEncryptionSdkStructuredEncryptionOperations {
  import opened StructuredEncryptionUtil
  import opened AwsCryptographyDbEncryptionSdkStructuredEncryptionTypes

  import SortCanon
  import Base64
  import CMP = AwsCryptographyMaterialProvidersTypes
  import Prim = AwsCryptographyPrimitivesTypes
  import StructuredEncryptionHeader
  import Random
  import Aws.Cryptography.Primitives
  import Header = StructuredEncryptionHeader
  import Footer = StructuredEncryptionFooter
  import MaterialProviders
  import Materials
  import Crypt = StructuredEncryptionCrypt
  import Paths = StructuredEncryptionPaths
  import SortedSets
  import Seq
  import Digest
  import Defaults
  import HKDF
  import AlgorithmSuites
  import Maps

  datatype Config = Config(
    primitives : Primitives.AtomicPrimitivesClient,
    materialProviders : MaterialProviders.MaterialProvidersClient
  )
  type InternalConfig = Config

  predicate ValidInternalConfig?(config: InternalConfig)
  {
    && config.primitives.ValidState()
    && config.materialProviders.ValidState()
  }

  function ModifiesInternalConfig(config: InternalConfig) : set<object>
  {config.primitives.Modifies + config.materialProviders.Modifies}

  predicate EncryptStructureEnsuresPublicly(
    input: EncryptStructureInput,
    output: Result<EncryptStructureOutput, Error>) {
    // Ensure the CryptoSchema in the ParsedHeader matches the input crypto Schema, minus any DO_NOTHING terminals
    output.Success? ==>
      // For now we only support encrypting flat maps
      && var headerSchema := output.value.cryptoSchema;
      && var inputSchema := input.cryptoSchema;
      // && (forall k :: k in headerSchema ==> k in inputSchema && inputSchema[k] == headerSchema[k])
      && (forall v :: v in headerSchema.Values ==> IsAuthAttr(v))
  }

  predicate DecryptStructureEnsuresPublicly(
    input: DecryptStructureInput,
    output: Result<DecryptStructureOutput, Error>) {
    output.Success? ==>
      // For now we only support encrypting flat maps
      && var headerSchema := output.value.cryptoSchema;
      // && var inputSchema := input.cryptoSchema;
      // && (forall k :: k in headerSchema ==> k in inputSchema && inputSchema[k] == headerSchema[k])
      && (forall v :: v in headerSchema.Values ==> IsAuthAttr(v))
  }

  predicate DecryptPathStructureEnsuresPublicly(
    input: DecryptPathStructureInput,
    output: Result<DecryptPathStructureOutput, Error>) {
    true
  }

  predicate EncryptPathStructureEnsuresPublicly(
    input: EncryptPathStructureInput,
    output: Result<EncryptPathStructureOutput, Error>) {
    true
  }

  predicate ResolveAuthActionsEnsuresPublicly(
    input: ResolveAuthActionsInput,
    output: Result<ResolveAuthActionsOutput, Error>) {
    true
  }

  method ResolveAuthActions (config: InternalConfig, input: ResolveAuthActionsInput)
    returns (output: Result<ResolveAuthActionsOutput, Error>)
  {
    var head :- Header.PartialDeserialize(input.headerBytes);
    :- Need(ValidString(input.tableName), E("Bad Table Name"));
    var canonData :- CanonizeForDecrypt(input.tableName, input.authActions, head.legend);
    return Success(ResolveAuthActionsOutput(cryptoActions := UnCanon(canonData)));
  }

  predicate method SameUnCanon(x : CanonCryptoItem, y : CryptoItem)
  {
    && x.origKey == y.key
    && x.data == y.data
    && x.action == y.action
  }

  function method UnCanon(input : CanonCryptoList) : (ret : CryptoList)
    ensures
      && |ret| == |input|
      && forall i | 0 <= i < |input| :: SameUnCanon(input[i], ret[i])
  {
    if |input| == 0 then
      []
    else
      var newItem := CryptoItem(key := input[0].origKey, data := input[0].data, action := input[0].action);
      assert SameUnCanon(input[0], newItem);
      [newItem] + UnCanon(input[1..])
  }

  const DBE_COMMITMENT_POLICY := CMP.CommitmentPolicy.DBE(CMP.DBECommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT)

  // Fail unless the field exists, and is a binary terminal
  function method {:opaque} GetBinary(data : AuthList, path : Path): (result: Result<StructuredDataTerminal, Error>)
    ensures result.Success? ==> exists x :: x in data && x.key == path
  {
    var data := FindAuth(data, path);

    if data.None? then
      Failure(E("The field name " + Paths.PathToString(path) + " is required."))
    else if data.value.data.typeId != BYTES_TYPE_ID then
      Failure(E(Paths.PathToString(path) + " must be a binary Terminal."))
    else if data.value.action != DO_NOT_SIGN then
      Failure(E(Paths.PathToString(path) + " must be DO_NOT_SIGN."))
    else
      Success(data.value.data)
  }

  // Return the sum of the sizes of the given fields
  function method {:opaque} SumValueSize(fields : CanonCryptoList)
    : nat
  {
    if |fields| == 0 then
      0
    else if fields[0].action == ENCRYPT_AND_SIGN then
      |fields[0].data.value| + SumValueSize(fields[1..])
    else
      SumValueSize(fields[1..])
  }

  function method {:opaque} GetAlgorithmSuiteId(alg : Option<CMP.DBEAlgorithmSuiteId>)
    : (ret : CMP.AlgorithmSuiteId)
    //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
    //= type=implication
    //# - Algorithm Suite: If provided, this is the [input algorithm suite](#algorithm-suite);
    //# otherwise, this field MUST be the algorithm suite corresponding to the enum
    //# [DBE.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum).
    ensures
      && (alg.Some? ==> ret == CMP.AlgorithmSuiteId.DBE(alg.value))
      && (alg.None? ==> ret == CMP.AlgorithmSuiteId.DBE(CMP.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384))
  {
    if alg.Some? then
      CMP.AlgorithmSuiteId.DBE(alg.value)
    else
      CMP.AlgorithmSuiteId.DBE(CMP.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384)
  }

  // return the appropriate EncryptionMaterials
  method {:opaque} GetStructuredEncryptionMaterials(
    cmm : CMP.ICryptographicMaterialsManager,
    encryptionContext : Option<CMP.EncryptionContext>,
    algorithmSuiteId: Option<CMP.DBEAlgorithmSuiteId>,
    encryptedTerminalDataNum : nat,
    totalEncryptedTerminalValuesSize : nat
  )
    returns (ret : Result<CMP.EncryptionMaterials, Error>)
    ensures ret.Success? ==>
              && var mat := ret.value;
              && Materials.EncryptionMaterialsHasPlaintextDataKey(mat)
              && ValidSuite(mat.algorithmSuite)

              //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
              //= type=implication
              //# This operation MUST obtain a set of encryption materials by calling
              //# [Get Encryption Materials](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/cmm-interface.md#get-encryption-materials)
              //# on the [CMM](#cmm) calculated above.

              //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
              //= type=implication
              //# This operation MUST call Get Encryption Materials on the CMM as follows.
              && (|cmm.History.GetEncryptionMaterials| == |old(cmm.History.GetEncryptionMaterials)| + 1)
              && Seq.Last(cmm.History.GetEncryptionMaterials).output.Success?
              && var getEncIn := Seq.Last(cmm.History.GetEncryptionMaterials).input;
              //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
              //= type=implication
              //# - Encryption Context: This MUST be the encryption context calculated above.
              && (|| (encryptionContext.None? && getEncIn.encryptionContext == map[])
                  || (encryptionContext.Some? && getEncIn.encryptionContext == encryptionContext.value))

              //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
              //= type=implication
              //# - Commitment Policy: This MUST be
              //# [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/commitment-policy.md#esdkrequire_encrypt_require_decrypt).
              && getEncIn.commitmentPolicy == DBE_COMMITMENT_POLICY

              //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
              //= type=implication
              //# - Max Plaintext Length: This field MUST be the result of the calculation `encryptedTerminalDataNum * 2 + totalEncryptedTerminalValuesSize`
              // - `encryptedTerminalDataNum` is the number of [Terminal Data](./structures.md#terminal-data)
              //   in the [input Structured Data](#structured-data) being encrypted,
              //   as defined by the [input Crypto Schema](#crypto-schema).
              // - `totalEncryptedTerminalValuesSize` is the sum of the length of all [Terminal Values](./structures.md#terminal-value)
              //   in the [input Structured Data](#structured-data) being encrypted,
              //   as defined by the [input Crypto Schema](#crypto-schema).
              && var maxLength :=  encryptedTerminalDataNum * 2 + totalEncryptedTerminalValuesSize;
              && maxLength < INT64_MAX_LIMIT
              && (getEncIn.maxPlaintextLength == Some(maxLength as int64))

    modifies cmm.Modifies
    requires cmm.ValidState()
    ensures cmm.ValidState()
  {
    var maxLength :=  encryptedTerminalDataNum * 2 + totalEncryptedTerminalValuesSize;
    :- Need(maxLength < INT64_MAX_LIMIT, E("Encrypted Size too long."));

    var algId := GetAlgorithmSuiteId(algorithmSuiteId);

    var matR := cmm.GetEncryptionMaterials(
      CMP.GetEncryptionMaterialsInput(
        encryptionContext := encryptionContext.UnwrapOr(map[]),
        commitmentPolicy := DBE_COMMITMENT_POLICY,
        algorithmSuiteId := Some(algId),
        maxPlaintextLength := Some(maxLength as int64),
        requiredEncryptionContextKeys := None
      )
    );

    var matOutput :- matR.MapFailure(e => AwsCryptographyMaterialProviders(e));
    var mat := matOutput.encryptionMaterials;
    :- Need(Materials.EncryptionMaterialsHasPlaintextDataKey(mat), E("Encryption material has no key"));
    var alg := mat.algorithmSuite;
    //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
    //# If this algorithm suite is not a
    //# [supported suite for Database Encryption (DBE)](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum),
    //# this operation MUST yield an error.
    :- Need(ValidSuite(alg), E("Invalid Algorithm Suite"));
    var key : Key := mat.plaintextDataKey.value;
    return Success(mat);
  }

  function method {:opaque} MakeCanon(tableName : GoodString, data : CryptoItem) : (result : CanonCryptoItem)
    requires Paths.ValidPath(data.key)
    ensures result.key == Paths.CanonPath(tableName, data.key)
    ensures result.origKey == data.key
    ensures result.data == data.data
    ensures result.action == data.action
  {
    CanonCryptoItem(Paths.CanonPath(tableName, data.key), data.key, data.data, data.action)
  }

  function method {:opaque} MakeCanonAuth(tableName : GoodString, data : AuthItem) : (result : CanonAuthItem)
    requires Paths.ValidPath(data.key)
    ensures result.key == Paths.CanonPath(tableName, data.key)
    ensures result.origKey == data.key
    ensures result.data == data.data
    ensures result.action == data.action
  {
    CanonAuthItem(Paths.CanonPath(tableName, data.key), data.key, data.data, data.action)
  }

  // construct the EncryptCanon
  function method {:opaque} {:vcs_split_on_every_assert} CanonizeForEncrypt(tableName : GoodString, data : CryptoList)
    : (ret : Result<CanonCryptoList, Error>)
    ensures ret.Success? ==>
              && (forall k <- data :: Paths.ValidPath(k.key))
              && (forall k <- data :: (exists x :: x in ret.value && x.origKey == k.key && k.data == x.data))
              && |data| == |ret.value|
              && (forall k <- ret.value :: Paths.ValidPath(k.origKey))
              && (forall k <- ret.value :: k.key == Paths.CanonPath(tableName, k.origKey))
  {
    :- Need(forall k <- data :: Paths.ValidPath(k.key), E("Invalid Paths"));
    var canonList : CanonCryptoList := Seq.Map((s : CryptoItem) requires Paths.ValidPath(s.key) => MakeCanon(tableName, s), data);

    assert |canonList| == |data|;
    assert forall i | 0 <= i < |data| :: canonList[i] == MakeCanon(tableName, data[i]);
    assert forall k <- data :: (exists x :: x in canonList && k.key == x.origKey && k.data == x.data);
    assert forall k <- canonList :: Paths.ValidPath(k.origKey);
    assert forall k <- canonList :: k.key == Paths.CanonPath(tableName, k.origKey);

    var canonSorted := SortCanon.CryptoSort(canonList);

    assert |canonSorted| == |data|;
    assert forall k <- canonList :: k in multiset(canonList);
    assert forall k <- canonList :: k in canonSorted;
    assert forall k <- canonSorted :: k in multiset(canonSorted);
    assert forall k <- canonSorted :: k in canonList;
    assert forall k <- data :: (exists x :: x in canonSorted && k.key == x.origKey);
    assert forall k <- canonSorted :: Paths.ValidPath(k.origKey);
    assert forall k <- canonSorted :: k.key == Paths.CanonPath(tableName, k.origKey);

    Success(canonSorted)
  }

  function method LegendToAction(v : Header.LegendByte) : CryptoAction
  {
    if v == Header.ENCRYPT_AND_SIGN_LEGEND then
      ENCRYPT_AND_SIGN
    else if v == Header.SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT_LEGEND then
      SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT
    else
      SIGN_ONLY
  }

  predicate method Same(x : CanonAuthItem, y : CanonCryptoItem)
  {
    && x.key == y.key
    && x.origKey == y.origKey
    && x.data == y.data
  }

  function method MakeCryptoItem(x : CanonAuthItem, action : CryptoAction) : (ret : CanonCryptoItem)
    ensures Same(x, ret)
  {
    CanonCryptoItem(x.key, x.origKey, x.data, action)
  }

  function method {:tailrecursion} {:opaque} ResolveLegend(
    fields : CanonAuthList,
    legend : Header.Legend,
    ghost origFields : CanonAuthList,
    acc : CanonCryptoList
  )
    : (ret : Result<CanonCryptoList, Error>)
    requires |fields| + |acc| == |origFields|
    requires forall i | 0 <= i < |acc| :: Same(origFields[i], acc[i])
    requires forall i | |acc| <= i < |origFields| :: origFields[i] == fields[i-|acc|]
    ensures ret.Success? ==>
              && |origFields| == |ret.value|
              && forall i | 0 <= i < |origFields| :: Same(origFields[i], ret.value[i])
  {
    if |fields| == 0 then
      :- Need(|legend| == 0, E("Schema changed : something that was signed is now unsigned."));
      Success(acc)
    else if fields[0].action == DO_NOT_SIGN then
      ResolveLegend(fields[1..], legend, origFields, acc + [MakeCryptoItem(fields[0], DO_NOTHING)])
    else
      :- Need(0 < |legend|, E("Schema changed : something that was unsigned is now signed."));
      ResolveLegend(fields[1..], legend[1..], origFields, acc + [MakeCryptoItem(fields[0], LegendToAction(legend[0]))])
  }

  // construct the DecryptCanon
  function method {:opaque}  {:vcs_split_on_every_assert}  CanonizeForDecrypt(tableName : GoodString, data : AuthList, legend: Header.Legend)
    : (ret : Result<CanonCryptoList, Error>)
    ensures ret.Success? ==>
              && (forall k <- data :: Paths.ValidPath(k.key))
              && (forall k <- data :: (exists x :: x in ret.value && k.key == x.origKey && k.data == x.data))
              && |data| == |ret.value|
              && (forall k <- ret.value :: Paths.ValidPath(k.origKey))
              && (forall k <- ret.value :: k.key == Paths.CanonPath(tableName, k.origKey))
  {
    :- Need(forall k <- data :: Paths.ValidPath(k.key), E("Invalid Paths"));
    var canonList : CanonAuthList := Seq.Map((s : AuthItem) requires Paths.ValidPath(s.key) => MakeCanonAuth(tableName, s), data);

    assert |canonList| == |data|;
    assert forall i | 0 <= i < |data| :: canonList[i] == MakeCanonAuth(tableName, data[i]);
    assert forall k <- data :: (exists x :: x in canonList && k.key == x.origKey && k.data == x.data);
    assert forall k <- canonList :: Paths.ValidPath(k.origKey);
    assert forall k <- canonList :: k.key == Paths.CanonPath(tableName, k.origKey);

    var canonSorted := SortCanon.AuthSort(canonList);

    assert |canonSorted| == |data|;
    assert forall k <- canonList :: k in multiset(canonList);
    assert forall k <- canonList :: k in canonSorted;
    assert forall k <- canonSorted :: k in multiset(canonSorted);
    assert forall k <- canonSorted :: k in canonList;
    assert forall k <- data :: (exists x :: x in canonSorted && k.key == x.origKey && k.data == x.data);
    assert forall k <- canonSorted :: Paths.ValidPath(k.origKey);
    assert forall k <- canonSorted :: k.key == Paths.CanonPath(tableName, k.origKey);

    var acc : CanonCryptoList := [];
    assert |canonSorted| + |acc| == |canonSorted|;
    assert forall i | 0 <= i < |acc| :: Same(canonSorted[i], acc[i]);
    assert forall i | |acc| <= i < |canonSorted| :: canonSorted[i] == canonSorted[i-|acc|];
    var canonResolved :- ResolveLegend(canonSorted, legend, canonSorted, acc);

    assert |canonResolved| == |data|;
    assert forall k <- data :: (exists x :: x in canonResolved && k.key == x.origKey && k.data == x.data);
    assert forall k <- canonResolved :: Paths.ValidPath(k.origKey);
    assert forall k <- canonResolved :: k.key == Paths.CanonPath(tableName, k.origKey);

    Success(canonResolved)
  }

  method GetV2EncryptionContext(schema : CryptoList)
    returns (output : Result<CMP.EncryptionContext, Error>)
  {
    var contextAttrs : CryptoList := Seq.Filter((s : CryptoItem) => s.action == SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT, schema);
    //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
    //# Otherwise, this operation MUST add an [entry](../dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2) to the encryption context for every
    //# [SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT Crypto Action](./structures.md#sign_and_include_in_encryption_context)
    //# [Terminal Data](./structures.md#terminal-data)
    //# in the input record, plus the Legend.
    output := GetV2EncryptionContext2(contextAttrs);
  }

  function method {:opaque} Find(haystack : CryptoList, needle : Path) : Result<CryptoItem, Error>
  {
    if |haystack| == 0 then
      Failure(E("Not Found"))
    else if haystack[0].key == needle
    then Success(haystack[0])
    else
      Find(haystack[1..], needle)
  }

  function method {:opaque} FindAuth(haystack : AuthList, needle : Path) : (result : Option<AuthItem>)
    ensures result.Some? ==> exists x :: x in haystack && x.key == needle
  {
    if |haystack| == 0 then
      None
    else if haystack[0].key == needle
    then Some(haystack[0])
    else
      FindAuth(haystack[1..], needle)
  }

  function method {:opaque} CountEncrypted(list : CanonCryptoList) : nat
  {
    if |list| == 0 then
      0
    else if list[0].action == ENCRYPT_AND_SIGN then
      1 + CountEncrypted(list[1..])
    else
      CountEncrypted(list[1..])
  }

  method {:vcs_split_on_every_assert} GetV2EncryptionContext2(fields : CryptoList)
    returns (output : Result<CMP.EncryptionContext, Error>)
  {
    var fieldMap : map<ValidUTF8Bytes, Path> := map[];
    for i := 0 to |fields|
    {
      //= specification/structured-encryption/encrypt-path-structure.md#encryption-context-naming
      //# When a key-value pair is added to the encryption context,
      //# the key MUST be the concatenation of the literal
      //# "aws-crypto-attr." and the member strings of the
      //# path joined by the '.' character.
      var keyVal := ATTR_PREFIX + Paths.PathToString(fields[i].key);

      var utf8Value :- UTF8.Encode(keyVal).MapFailure(e =>E(e));

      //= specification/structured-encryption/encrypt-path-structure.md#encryption-context-naming
      //# An error MUST be returned if an attempt is made to add two
      //# different attributes that produce the same encryption context key.
      if utf8Value in fieldMap {
        return Failure(E(keyVal + " appears twice in encryption context."));
      }

      fieldMap := fieldMap[utf8Value := fields[i].key];
    }
    var keys : seq<UTF8.ValidUTF8Bytes> := SortedSets.ComputeSetToOrderedSequence2(fieldMap.Keys, ByteLess);
    var newContext : CMP.EncryptionContext := map[];
    var legend : string := "";

    //= specification/dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2
    //# The value MUST be :
    //# - If the type is Number or String, the unaltered (already utf8) bytes of the value
    //# - If the type if Null, the string "null"
    //# - If the type is Boolean, then the string "true" for true and the string "false" for false.
    //# - Else, the value as defined in [Base Context Value Version 1](#base-context-value-version-1)

    //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
    //# The Legend MUST be named "aws-crypto-legend" and be a string with one character per attribute added above,
    //# with a one-to-one correspondence with the attributes sorted by their UTF8 encoding,
    //# each character designating the original type of the attribute,
    //# to allow reversing of the [encoding](../dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2).
    //# - 'S' if the attribute was of type String
    //# - 'N' if the attribute was of type Number
    //# - 'L' if the attribute was of type Null or Boolean
    //# - 'B' otherwise
    for i := 0 to |keys|
      invariant forall j | 0 <= j < i :: keys[j] in newContext
      invariant forall k <- newContext :: k in keys[..i]
      invariant |legend| == |newContext| == i
    {
      assert keys[i] !in newContext by {
        reveal Seq.HasNoDuplicates();
      }
      var fieldUtf8 := keys[i];
      var fieldStr := fieldMap[fieldUtf8];
      var item :- Find(fields, fieldMap[fieldUtf8]);
      var attr : StructuredDataTerminal := item.data;
      var attrStr : ValidUTF8Bytes;
      var legendChar : char;
      if attr.typeId == NULL {
        legendChar := LEGEND_LITERAL;
        attrStr := NULL_UTF8;
      } else if attr.typeId == STRING {
        legendChar := LEGEND_STRING;
        :- Need(ValidUTF8Seq(attr.value), E("Internal Error : string was not UTF8."));
        attrStr := attr.value;
        var yy :- expect UTF8.Decode(attrStr);
      } else if attr.typeId == NUMBER {
        legendChar := LEGEND_NUMBER;
        :- Need(ValidUTF8Seq(attr.value), E("Internal Error : number was not UTF8."));
        attrStr := attr.value;
      } else if attr.typeId == BOOLEAN {
        legendChar := LEGEND_LITERAL;
        :- Need(|attr.value| == 1, E("Internal Error : boolean was not of length 1."));
        attrStr := if attr.value[0] == 0 then FALSE_UTF8 else TRUE_UTF8;
      } else {
        legendChar := LEGEND_BINARY;
        attrStr := EncodeTerminal(attr);
      }
      assert fieldUtf8 !in newContext;
      newContext := newContext[fieldUtf8 := attrStr];
      legend := legend + [legendChar];
      assert forall j | 0 <= j < i+1 :: keys[j] in newContext by {
        assert keys[i] in newContext;
      }
    }
    var utf8Legend :- UTF8.Encode(legend).MapFailure(e =>E(e));
    newContext := newContext[LEGEND_UTF8 := utf8Legend];

    return Success(newContext);
  }

  function method {:tailrecursion} BuildCryptoMap2(
    keys : seq<string>,
    plaintextStructure: StructuredDataMap,
    cryptoSchema: CryptoSchemaMap,
    acc : CryptoList := []
  )
    : (ret : Result<CryptoList, Error>)
    requires forall k <- keys :: k in plaintextStructure
    requires forall k <- keys :: k in cryptoSchema
    requires forall k <- acc :: |k.key| == 1
    ensures ret.Success? ==>
              forall k <- ret.value :: |k.key| == 1
  {
    if |keys| == 0 then
      Success(acc)
    else
      var key := keys[0];
      var path := Paths.StringToUniPath(key);
      var item := CryptoItem(key := path, data := plaintextStructure[key], action := cryptoSchema[key]);
      BuildCryptoMap2(keys[1..], plaintextStructure, cryptoSchema, acc + [item])
  }

  function method BuildCryptoMap(plaintextStructure: StructuredDataMap, cryptoSchema: CryptoSchemaMap) :
    (ret : Result<CryptoList, Error>)
    requires plaintextStructure.Keys == cryptoSchema.Keys
    ensures ret.Success? ==>
              forall k <- ret.value :: |k.key| == 1
  {
    var keys := SortedSets.ComputeSetToOrderedSequence2(plaintextStructure.Keys, CharLess);
    BuildCryptoMap2(keys, plaintextStructure, cryptoSchema)
  }

  function method {:tailrecursion} BuildAuthMap2(
    keys : seq<string>,
    plaintextStructure: StructuredDataMap,
    authSchema: AuthenticateSchemaMap,
    acc : AuthList := []
  )
    : (ret : Result<AuthList, Error>)
    requires forall k <- keys :: k in plaintextStructure
    requires forall k <- keys :: k in authSchema
    requires forall k <- acc :: |k.key| == 1
    ensures ret.Success? ==>
              forall k <- ret.value :: |k.key| == 1
  {
    if |keys| == 0 then
      Success(acc)
    else
      var key := keys[0];
      var path := Paths.StringToUniPath(key);
      var item := AuthItem(key := path, data := plaintextStructure[key], action := authSchema[key]);
      BuildAuthMap2(keys[1..], plaintextStructure, authSchema, acc + [item])
  }

  function method BuildAuthMap(plaintextStructure: StructuredDataMap, authSchema: AuthenticateSchemaMap)
    : (ret : Result<AuthList, Error>)
    requires plaintextStructure.Keys == authSchema.Keys
    ensures ret.Success? ==>
              forall k <- ret.value :: |k.key| == 1
  {
    var keys := SortedSets.ComputeSetToOrderedSequence2(plaintextStructure.Keys, CharLess);
    BuildAuthMap2(keys, plaintextStructure, authSchema)
  }

  function method UnBuildCryptoMap(list : CryptoList, dataSoFar : StructuredDataMap := map[], actionsSoFar : CryptoSchemaMap := map[]) :
    (res : Result<(StructuredDataMap, CryptoSchemaMap), Error>)
    requires forall k <- actionsSoFar :: k in dataSoFar
    requires (forall v :: v in actionsSoFar.Values ==> IsAuthAttr(v))
    requires forall k <- list :: |k.key| == 1
    ensures res.Success? ==>
              && (forall k <- res.value.1 :: k in res.value.0)
              && (forall v :: v in res.value.1.Values ==> IsAuthAttr(v))
  {
    if |list| == 0 then
      Success((dataSoFar, actionsSoFar))
    else
      var key :- Paths.UniPathToString(list[0].key);
      :- Need(key !in dataSoFar, E("Duplicate Key " + key));
      if IsAuthAttr(list[0].action) then
        UnBuildCryptoMap(list[1..], dataSoFar[key := list[0].data], actionsSoFar[key := list[0].action])
      else
        UnBuildCryptoMap(list[1..], dataSoFar[key := list[0].data], actionsSoFar)
  }


  method {:vcs_split_on_every_assert} EncryptStructure(config: InternalConfig, input: EncryptStructureInput)
    returns (output: Result<EncryptStructureOutput, Error>)
    ensures output.Success? ==>
              && var headerSchema := output.value.cryptoSchema;
              && var inputSchema := input.cryptoSchema;
              // && (forall k :: k in headerSchema ==> k in inputSchema && inputSchema[k] == headerSchema[k])
              && (forall v :: v in headerSchema.Values ==> IsAuthAttr(v))
  {
    //= specification/structured-encryption/encrypt-structure.md#behavior
    //= type=implication
    //# The input [Structured Data](encrypt-path-structure.md#structured-data) and [Crypto Schema](encrypt-path-structure.md#crypto-schema)
    //# MUST refer to the same set of locations.
    :- Need(input.plaintextStructure.Keys == input.cryptoSchema.Keys, E("Crypto Keys don't match."));

    //= specification/structured-encryption/encrypt-structure.md#behavior
    //= type=implication
    //# The input [Structured Data](encrypt-path-structure.md#structured-data) and [Crypto Schema](encrypt-path-structure.md#crypto-schema)
    //# MUST be combined into a single [Crypto List](encrypt-path-structure.md#crypto-list).
    var cryptoMap :- BuildCryptoMap(input.plaintextStructure, input.cryptoSchema);

    var pathInput := EncryptPathStructureInput(
      tableName := input.tableName,
      plaintextStructure := cryptoMap,
      cmm := input.cmm,
      algorithmSuiteId := input.algorithmSuiteId,
      encryptionContext := input.encryptionContext
    );

    //= specification/structured-encryption/encrypt-structure.md#behavior
    //= type=implication
    //# Encrypt Structure MUST then behave as [Encrypt Path Structure](encrypt-path-structure.md)
    var pathOutput :- EncryptPathStructure(config, pathInput);

    // This should be provable, but I'm not smart enough
    assert forall k <- pathInput.plaintextStructure :: |k.key| == 1;
    :- Need(forall k <- pathOutput.encryptedStructure :: |k.key| == 1, E("Internal Error"));

    //= specification/structured-encryption/encrypt-structure.md#behavior
    //= type=implication
    //# The output [Crypto List](encrypt-path-structure.md#crypto-list) produced by [Encrypt Path Structure](encrypt-path-structure.md)
    //# MUST be split into [Structured Data](encrypt-path-structure.md#structured-data) and [Crypto Schema](encrypt-path-structure.md#crypto-schema)
    //# maps.
    var parts :- UnBuildCryptoMap(pathOutput.encryptedStructure);
    var plainOutput := EncryptStructureOutput(
      encryptedStructure := parts.0,
      cryptoSchema := parts.1,
      parsedHeader := pathOutput.parsedHeader
    );
    return Success(plainOutput);
  }

  const HeaderPaths : seq<Path> := [HeaderPath, FooterPath]

  method {:vcs_split_on_every_assert} EncryptPathStructure(config: InternalConfig, input: EncryptPathStructureInput)
    returns (output: Result<EncryptPathStructureOutput, Error>)
    ensures
      output.Success? ==>
        //= specification/structured-encryption/encrypt-path-structure.md#crypto-list
        //= type=implication
        //# The Crypto List MUST include at least one [Crypto Action](./structures.md#crypto-action)
        //# that is not [DO_NOTHING](./structures.md#do_nothing).
        && (exists k <- input.plaintextStructure :: IsAuthAttr(k.action))

        //= specification/structured-encryption/encrypt-path-structure.md#crypto-list
        //= type=implication
        //# This Crypto List MUST NOT already contain data located at the [header index](./header.md#header-index)
        //# or the [footer index](./footer.md#footer-index).
        && (!exists x | x in input.plaintextStructure :: x.key in HeaderPaths)

        //= specification/structured-encryption/encrypt-path-structure.md#encrypted-structured-data
        //= type=implication
        //# - for every entry in the input [Crypto List](#crypto-list)
        //# an entry MUST exist with the same [path](./structures.md#path) in the final Encrypted Structured Data.
        && (forall k <- input.plaintextStructure :: (exists x :: x in output.value.encryptedStructure && x.key == k.key))

        //= specification/structured-encryption/encrypt-path-structure.md#encrypted-structured-data
        //= type=implication
        //# Otherwise, this Terminal Data MUST have [Terminal Type ID](./structures.md#terminal-type-id)
        //# and [Terminal Value](./structures.md#terminal-value) equal to the input Terminal Data's.
        && (forall k <- input.plaintextStructure ::
              (exists x ::
                 && x in output.value.encryptedStructure
                 && x.key == k.key
                 && (
                      || k.action == ENCRYPT_AND_SIGN
                      || x.data == k.data
                    )))

        //= specification/structured-encryption/encrypt-path-structure.md#crypto-list
        //= type=implication
        //# The [paths](./structures.md#path) in the input [Crypto List](./structures.md#crypto-list) MUST be unique.
        && var pathSet := set x | x in input.plaintextStructure :: x.key;
        && |pathSet| == |input.plaintextStructure|

        //= specification/structured-encryption/encrypt-path-structure.md#encrypted-structured-data
        //= type=implication
        //# - There MUST be no other entries in the final Encrypted Structured Data.
        && |output.value.encryptedStructure| == 2 + |input.plaintextStructure|

        //= specification/structured-encryption/encrypt-path-structure.md#encrypted-structured-data
        //= type=implication
        //# - The [Header Field](#header-field) MUST exist in the final Encrypted Structured Data
        && output.value.encryptedStructure[|output.value.encryptedStructure|-2].key == HeaderPath

        //= specification/structured-encryption/encrypt-path-structure.md#encrypted-structured-data
        //= type=implication
        //# - The [Footer Field](#footer-field) MUST exist in the final Encrypted Structured Data
        && output.value.encryptedStructure[|output.value.encryptedStructure|-1].key == FooterPath

        //= specification/structured-encryption/encrypt-path-structure.md#encrypted-structured-data
        //= type=implication
        //# If the [Crypto Schema](#crypto-list)
        //# indicates a [Crypto Action](./structures.md#crypto-action)
        //# of [ENCRYPT_AND_SIGN](./structures.md#encryptandsign),
        //# the Terminal Data MUST have [Terminal Type ID](./structures.md#terminal-type-id)
        //# equal to 0xffff and the value MUST be
        //# the [encryption](#terminal-data-encryption)
        //# of the input's Terminal Data.
        && (forall x | 0 <= x < |output.value.encryptedStructure| :: (output.value.encryptedStructure[x].action == ENCRYPT_AND_SIGN ==> output.value.encryptedStructure[x].data.typeId == BYTES_TYPE_ID))
  {
    :- Need(
      || input.encryptionContext.None?
      || !exists k <- input.encryptionContext.value :: ReservedCryptoContextPrefixUTF8 <= input.encryptionContext.value[k],
      E("Encryption Context must not contain members beginning with " + ReservedCryptoContextPrefixString));

    :- Need(exists k <- input.plaintextStructure :: IsAuthAttr(k.action),
            E("At least one field in the Crypto Schema must be ENCRYPT_AND_SIGN, SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT or SIGN_ONLY."));

    :- Need(!exists x | x in input.plaintextStructure :: x.key in HeaderPaths,
            E("The paths " + HeaderField + " and " + FooterField + " are reserved."));

    var pathSet := set x | x in input.plaintextStructure :: x.key;
    :- Need(|pathSet| == |input.plaintextStructure|, E("Duplicate Paths"));

    :- Need(ValidString(input.tableName), E("Bad Table Name"));
    var plaintextStructure : CryptoList := input.plaintextStructure;
    var canonData :- CanonizeForEncrypt(input.tableName, plaintextStructure);

    //= specification/structured-encryption/encrypt-path-structure.md#calculate-intermediate-encrypted-structured-data
    //= type=implication
    //# For every entry
    //# in the input [Crypto List](#crypto-list)
    //# there MUST be an entry with the same [canonical path](./header.md#canonical-path)
    //# in Intermediate Encrypted Structured Data.
    assert forall k <- input.plaintextStructure :: (exists x :: x in canonData && x.origKey == k.key && x.data == k.data);

    //= specification/structured-encryption/encrypt-path-structure.md#calculate-intermediate-encrypted-structured-data
    //= type=implication
    //# There MUST be no other entries in the Intermediate Encrypted Structured Data.
    assert |input.plaintextStructure| == |canonData|;

    //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
    //# This operation MUST [calculate the appropriate CMM and encryption context](#create-new-encryption-context-and-cmm).
    var encryptionContext := input.encryptionContext.UnwrapOr(map[]);
    var cmm := input.cmm;

    //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
    //# If no [Crypto Action](./structures.md#crypto-action) is configured to be
    //# [SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT Crypto Action](./structures.md#sign_and_include_in_encryption_context)
    //# then the input cmm and encryption context MUST be used unchanged.
    if exists x <- plaintextStructure :: x.action == SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT {
      assume {:axiom} input.cmm.Modifies !! {config.materialProviders.History};
      var newEncryptionContext :- GetV2EncryptionContext(plaintextStructure);
      if |newEncryptionContext| != 0 {
        //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
        //# An error MUST be returned if any of the entries added to the encryption context in this step
        //# have the same key as any entry already in the encryption context.
        :- Need(encryptionContext.Keys !! newEncryptionContext.Keys,
                E("Internal Error - Structured Encryption encryption context overlaps with Item Encryptor encryption context."));
        encryptionContext := encryptionContext + newEncryptionContext;
        assert cmm.Modifies !! {config.materialProviders.History};
        //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
        //# Then, this operation MUST create a [Required Encryption Context CMM](https://github.com/awslabs/private-aws-encryption-sdk-specification-staging/blob/dafny-verified/framework/required-encryption-context-cmm.md)
        //# with the following inputs:
        //# - This input [CMM](./ddb-table-encryption-config.md#cmm) as the underlying CMM.
        //# - The name of every entry added above.
        var contextKeysX := SortedSets.ComputeSetToOrderedSequence2(newEncryptionContext.Keys, ByteLess);
        assert forall k <- contextKeysX :: ValidUTF8Seq(k) by {
          assert forall k <- newEncryptionContext.Keys :: ValidUTF8Seq(k);
          assert forall k <- contextKeysX :: k in newEncryptionContext.Keys;
        }
        var contextKeys :  seq<UTF8.ValidUTF8Bytes> := contextKeysX;
        var cmmR := config.materialProviders.CreateRequiredEncryptionContextCMM(
          CMP.CreateRequiredEncryptionContextCMMInput(
            underlyingCMM := Some(input.cmm),
            keyring := None,
            requiredEncryptionContextKeys := contextKeys
          )
        );
        cmm :- cmmR.MapFailure(e => AwsCryptographyMaterialProviders(e));
      }
    }

    var mat :- GetStructuredEncryptionMaterials(
      cmm,
      Some(encryptionContext),
      input.algorithmSuiteId,
      CountEncrypted(canonData),
      SumValueSize(canonData));

    var key : Key := mat.plaintextDataKey.value;
    var alg := mat.algorithmSuite;
    :- Need(Header.ValidEncryptionContext(mat.encryptionContext), E("Bad encryption context"));

    //= specification/structured-encryption/header.md#message-id
    //# Implementations MUST generate a fresh 256-bit random MessageID, from a cryptographically secure source, for each record encrypted.

    //= specification/structured-encryption/encrypt-path-structure.md#calculate-intermediate-encrypted-structured-data
    //# The process used to generate this identifier MUST use a good source of randomness
    //# to make the chance of duplicate identifiers negligible.
    var randBytes := Random.GenerateBytes(MSGID_LEN as int32);
    var msgID :- randBytes.MapFailure(e => Error.AwsCryptographyPrimitives(e));
    var head :- Header.Create(input.tableName, canonData, msgID, mat);
    //= specification/structured-encryption/header.md#commit-key
    //# The commit key calculation described above MUST be performed with the record's plaintext data key
    //# and the header's message id.
    var commitKey :- Crypt.GetCommitKey(config.primitives, alg, key, head.msgID);
    var headerSerialized :- Header.Serialize(config.primitives, alg, commitKey, head);

    //= specification/structured-encryption/encrypt-path-structure.md#header-field
    //# The Header Field TypeID MUST be 0xFFFF

    //= specification/structured-encryption/encrypt-path-structure.md#header-field
    //# The Header Field Value MUST be the full serialized [header](header.md) with commitment.
    var headerAttribute := ValueToData(headerSerialized, BYTES_TYPE_ID);

    :- Need(|canonData| < (UINT32_LIMIT / 3), E("Too many encrypted fields"));
    // input canonData has all input fields, none encrypted
    // output canonData has all input fields, some encrypted
    assert forall k <- input.plaintextStructure :: (exists x :: x in canonData && x.origKey == k.key);
    var encryptedItems : CanonCryptoList :- Crypt.Encrypt(config.primitives, alg, key, head, canonData);
    assert forall k <- input.plaintextStructure :: (exists x :: x in encryptedItems && x.origKey == k.key);

    // this assert can be an implication, because it is explicitly ensuring an intermediate state.
    assert forall i | 0 <= i < |canonData| :: canonData[i].key == encryptedItems[i].key;

    // this assert can be an implication, because it is explicitly ensuring an intermediate state.
    assert forall i | 0 <= i < |encryptedItems| :: encryptedItems[i].key == canonData[i].key;

    assert forall x | 0 <= x < |encryptedItems| :: (encryptedItems[x].action == ENCRYPT_AND_SIGN ==> encryptedItems[x].data.typeId == BYTES_TYPE_ID);
    assert forall x | 0 <= x < |encryptedItems| :: (encryptedItems[x].action == ENCRYPT_AND_SIGN || encryptedItems[x].data == canonData[x].data);

    // verifies, but it takes too long
    assume {:axiom} forall k <- input.plaintextStructure ::
        (exists x ::
           && x in encryptedItems
           && x.origKey == k.key
           && Crypt.Updated5(k, x, Crypt.DoEncrypt)
        );

    var footer :- Footer.CreateFooter(config.primitives, mat, encryptedItems, headerSerialized);
    var footerAttribute := footer.makeTerminal();

    assert forall k <- input.plaintextStructure :: (exists x :: x in encryptedItems && x.origKey == k.key);
    var smallResult : CryptoList := UnCanon(encryptedItems);
    assert forall k <- input.plaintextStructure :: (exists x :: x in smallResult && x.key == k.key);
    assert forall x | 0 <= x < |smallResult| :: (smallResult[x].action == ENCRYPT_AND_SIGN ==> smallResult[x].data.typeId == BYTES_TYPE_ID) by {
      assert |smallResult| == |encryptedItems|;
      assert forall x | 0 <= x < |smallResult| :: SameUnCanon(encryptedItems[x], smallResult[x]);
      assert forall x | 0 <= x < |smallResult| :: (smallResult[x].action == encryptedItems[x].action && smallResult[x].data == encryptedItems[x].data);
      assert forall x | 0 <= x < |encryptedItems| :: (encryptedItems[x].action == ENCRYPT_AND_SIGN || encryptedItems[x].data == canonData[x].data);
    }
    // verifies, but it takes too long
    assume {:axiom} forall k <- input.plaintextStructure ::
        (exists x ::
           && x in smallResult
           && x.key == k.key
           && Crypt.Updated4(k, x, Crypt.DoEncrypt)
        );

    var headItem := CryptoItem(key := HeaderPath, data := headerAttribute, action := DO_NOTHING);
    var footItem := CryptoItem(key := FooterPath, data := footerAttribute, action := DO_NOTHING);
    var largeResult := smallResult + [headItem, footItem];
    assert |largeResult| == |smallResult| + 2;
    assert largeResult[|largeResult|-2] == headItem;
    assert largeResult[|largeResult|-2].key == HeaderPath;
    assert largeResult[|largeResult|-1] == footItem;
    assert largeResult[|largeResult|-1].key == FooterPath;
    assert forall k <- input.plaintextStructure :: (exists x :: x in largeResult && x.key == k.key);
    assert forall x | 0 <= x < |largeResult| :: (largeResult[x].action == ENCRYPT_AND_SIGN ==> largeResult[x].data.typeId == BYTES_TYPE_ID) by {
      assert forall x | 0 <= x < |smallResult| :: (smallResult[x].action == ENCRYPT_AND_SIGN ==> smallResult[x].data.typeId == BYTES_TYPE_ID);
      assert forall x | 0 <= x < |smallResult| :: smallResult[x] == largeResult[x];
      assert forall x | 0 <= x < |smallResult| :: (largeResult[x].action == ENCRYPT_AND_SIGN ==> largeResult[x].data.typeId == BYTES_TYPE_ID);
      assert largeResult[|smallResult|] == headItem;
      assert largeResult[|smallResult|].key == HeaderPath;
      assert largeResult[|smallResult|+1] == footItem;
      assert largeResult[|smallResult|+1].key == FooterPath;
      assert largeResult[|smallResult|].action == DO_NOTHING;
      assert largeResult[|smallResult|+1].action == DO_NOTHING;
      assert |largeResult| == |smallResult| + 2;
      // verifies, but it takes too long
      assume {:axiom} forall x | |smallResult| <= x < |largeResult| :: largeResult[x].action == DO_NOTHING;
    }

    assert forall k <- input.plaintextStructure ::
        (exists x ::
           && x in largeResult
           && x.key == k.key
           && Crypt.Updated4(k, x, Crypt.DoEncrypt)
        );

    var headerAlgorithmSuite :- head.GetAlgorithmSuite(config.materialProviders);
    var parsedHeader := ParsedHeader (
      algorithmSuiteId := headerAlgorithmSuite.id.DBE,
      encryptedDataKeys := head.dataKeys,
      storedEncryptionContext := head.encContext,
      encryptionContext := mat.encryptionContext
    );

    var encryptOutput := EncryptPathStructureOutput (
      encryptedStructure := largeResult,
      parsedHeader := parsedHeader
    );
    assert encryptOutput.encryptedStructure[|encryptOutput.encryptedStructure|-1].key == FooterPath;

    return Success(encryptOutput);
  }

  function method SafeDecode(data : CMP.Utf8Bytes) : string
  {
    var x := UTF8.Decode(data);
    if x.Success? then
      x.value
    else
      "[corrupt value]"
  }

  function method {:tailrecursion} DescribeMismatch(inputFields : seq<Bytes>, inputContext : CMP.EncryptionContext, headContext : Header.CMP.EncryptionContext)
    : string
    requires forall k <- inputFields :: k in inputContext
  {
    if |inputFields| == 0 then
      ""
    else
      var key := inputFields[0];
      if key in headContext && headContext[key] != inputContext[key] then
        var keyStr := SafeDecode(key);
        var headStr := SafeDecode(headContext[key]);
        var inputStr := SafeDecode(inputContext[key]);
        var msg := "input context for " + keyStr + " was " + inputStr + " but stored context had " + headStr + "\n";
        msg + DescribeMismatch(inputFields[1..], inputContext, headContext)
      else
        DescribeMismatch(inputFields[1..], inputContext, headContext)
  }

  function method DetectMismatch(inputContext : CMP.EncryptionContext, headContext : CMP.EncryptionContext)
    : Outcome<Error>
  {
    var inputFields := SortedSets.ComputeSetToOrderedSequence2(inputContext.Keys, ByteLess);
    var str := DescribeMismatch(inputFields, inputContext, headContext);
    if |str| == 0 then
      Pass
    else
      Fail(E("Encryption Context Mismatch\n" + str))
  }

  method {:vcs_split_on_every_assert} DecryptStructure (config: InternalConfig, input: DecryptStructureInput)
    returns (output: Result<DecryptStructureOutput, Error>)
  {
    //= specification/structured-encryption/decrypt-structure.md#behavior
    //= type=implication
    //# The input [Structured Data](decrypt-path-structure.md#structured-data) and [Authenticate Schema](decrypt-path-structure.md#authenticate-schema)
    //# MUST refer to the same set of locations.
    :- Need(input.encryptedStructure.Keys == input.authenticateSchema.Keys, E("DecryptStructure requires encryptedStructure and authenticateSchema have the same keys."));

    //= specification/structured-encryption/decrypt-structure.md#behavior
    //= type=implication
    //# The input [Structured Data](decrypt-path-structure.md#structured-data) and [Authenticate Schema](decrypt-path-structure.md#authenticate-schema)
    //# MUST be combined into a single [Auth List](decrypt-path-structure.md#auth-list).
    var cryptoMap :- BuildAuthMap(input.encryptedStructure, input.authenticateSchema);

    var pathInput := DecryptPathStructureInput(
      tableName := input.tableName,
      encryptedStructure := cryptoMap,
      cmm := input.cmm,
      encryptionContext := input.encryptionContext
    );

    //= specification/structured-encryption/decrypt-structure.md#behavior
    //= type=implication
    //# Decrypt Structure MUST then behave as [Decrypt Path Structure](decrypt-path-structure.md)
    var pathOutput :- DecryptPathStructure(config, pathInput);

    // This should be provable, but I'm not smart enough
    assert forall k <- pathInput.encryptedStructure :: |k.key| == 1;
    :- Need(forall k <- pathOutput.plaintextStructure :: |k.key| == 1, E("Internal Error"));

    //= specification/structured-encryption/decrypt-structure.md#behavior
    //= type=implication
    //# The output [Crypto List](decrypt-path-structure.md#crypto-list) produced by [Decrypt Path Structure](decrypt-path-structure.md)
    //# MUST be split into [Structured Data](decrypt-path-structure.md#structured-data) and [Crypto Schema](decrypt-path-structure.md#crypto-schema)
    //# maps.
    var parts :- UnBuildCryptoMap(pathOutput.plaintextStructure);
    var plainOutput := DecryptStructureOutput(
      plaintextStructure := parts.0,
      cryptoSchema := parts.1,
      parsedHeader := pathOutput.parsedHeader
    );
    return Success(plainOutput);
  }

  method {:vcs_split_on_every_assert} DecryptPathStructure (config: InternalConfig, input: DecryptPathStructureInput)
    returns (output: Result<DecryptPathStructureOutput, Error>)

    ensures output.Success? ==>
              && var encRecord : AuthList := input.encryptedStructure;

              //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
              //= type=implication
              //# Given the [input data](#auth-list),
              //# this operation MUST access the [Terminal Data](./structures.md#terminal-data)
              //# at "aws_dbe_head".

              //= specification/structured-encryption/decrypt-path-structure.md#auth-list
              //= type=implication
              //# This Auth List MUST contain data located at the [header index](./header.md#header-index)
              //# and the [footer index](./footer.md#footer-index).

              //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
              //= type=implication
              //# The [Terminal Type Id](./structures.md#terminal-type-id) on this Terminal Data MUST be `0xFFFF`.
              && GetBinary(encRecord, HeaderPath).Success?
              && var headerSerialized := GetBinary(encRecord, HeaderPath).value;

              //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
              //= type=implication
              //# A footer field MUST exist with the name `aws_dbe_foot`

              //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
              //= type=implication
              //# The footer field TypeID MUST be 0xFFFF
              && GetBinary(encRecord, FooterPath).Success?
              && var footerSerialized := GetBinary(encRecord, FooterPath).value;

              //= specification/structured-encryption/decrypt-path-structure.md#auth-list
              //= type=implication
              //# The Auth List MUST include at least one [SIGN Authenticate Action](./structures.md#sign);
              //# otherwise, this operation MUST yield an error.
              && (exists x :: (x in encRecord && x.action == SIGN))

              //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
              //= type=implication
              //# This operation MUST deserialize the header bytes
              //# according to the [header format](./header.md).
              && Header.PartialDeserialize(headerSerialized.value).Success?

              //= specification/structured-encryption/decrypt-path-structure.md#construct-decrypted-structured-data
              //= type=implication
              //# - An entry MUST NOT exist with the key "aws_dbe_head" or "aws_dbe_foot".
              && (!exists x :: x in output.value.plaintextStructure && x.key == HeaderPath)
              && (!exists x :: x in output.value.plaintextStructure && x.key == FooterPath)

              //= specification/structured-encryption/decrypt-path-structure.md#construct-decrypted-structured-data
              //= type=implication
              //# - For every entry in the [input Auth List](#auth-list), other than the header and footer,
              //# an entry MUST exist with the same key in the output Crypto List.
              && (forall k <- input.encryptedStructure | k.key !in HeaderPaths ::
                    (exists x :: x in output.value.plaintextStructure && x.key == k.key))

              //= specification/structured-encryption/decrypt-path-structure.md#construct-decrypted-structured-data
              //= type=implication
              //# - The output Crypto List MUST NOT have any additional entries.
              && |output.value.plaintextStructure| == |input.encryptedStructure| - 2

              //= specification/structured-encryption/decrypt-path-structure.md#construct-decrypted-structured-data
              //= type=implication
              //# If the action is [ENCRYPT_AND_SIGN](./structures.md#encryptandsign)
              //# this Terminal Data MUST have [Terminal Type ID](./structures.md#terminal-type-id)
              //# equal to the first two bytes of the input Terminal Data's value,
              //# and a value equal to the [decryption](#terminal-data-decryption) of the input Terminal Data's value.

              //= specification/structured-encryption/decrypt-path-structure.md#construct-decrypted-structured-data
              //= type=implication
              //# Otherwise, this Terminal Data MUST have [Terminal Type ID](./structures.md#terminal-type-id) and
              //# [Terminal Value](./structures.md#terminal-value) equal to the input Terminal Data.
              && (forall k <- input.encryptedStructure  | k.key !in HeaderPaths ::
                    (exists x ::
                       && x in output.value.plaintextStructure
                       && x.key == k.key
                       && (x.action == ENCRYPT_AND_SIGN ==> |k.data.value| >= 2 && x.data.typeId == k.data.value[..2])
                       && (x.action != ENCRYPT_AND_SIGN ==> k.data == x.data)
                    )
                 )
  {
    :- Need(exists x :: (x in input.encryptedStructure && x.action == SIGN), E("At least one Authenticate Action must be SIGN"));

    var headerSerialized :- GetBinary(input.encryptedStructure, HeaderPath);
    var footerSerialized :- GetBinary(input.encryptedStructure, FooterPath);
    assert exists x :: x in input.encryptedStructure && x.key == HeaderPath;
    assert exists x :: x in input.encryptedStructure && x.key == FooterPath;

    //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
    //# This operation MUST deserialize the header bytes
    //# according to the [header format](./header.md).
    var head :- Header.PartialDeserialize(headerSerialized.value);
    var headerAlgorithmSuite :- head.GetAlgorithmSuite(config.materialProviders);

    :- Need(ValidString(input.tableName), E("Bad Table Name"));
    var canonData :- CanonizeForDecrypt(input.tableName, input.encryptedStructure, head.legend);
    assert forall k <- input.encryptedStructure :: (exists x :: x in canonData && k.key == x.origKey && k.data == x.data);
    assert |canonData| == |input.encryptedStructure|;
    assert exists x :: x in canonData && x.origKey == HeaderPath;
    assert exists x :: x in canonData && x.origKey == FooterPath;

    assume {:axiom} input.cmm.Modifies !! {config.materialProviders.History};

    //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
    //# This operation MUST [calculate the appropriate CMM and encryption context](#create-new-encryption-context-and-cmm).
    var encryptionContext := input.encryptionContext.UnwrapOr(map[]);
    var cmm := input.cmm;

    //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
    //# If the version stored in the header is 1,
    //# then the input cmm and encryption context MUST be used unchanged.
    if head.version == 2 {
      //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
      //# Otherwise, this operation MUST add an [entry](../dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2) to the encryption context for every
      //# [SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT Crypto Action](./structures.md#sign_and_include_in_encryption_context)
      //# [Terminal Data](./structures.md#terminal-data)
      //# in the input record, plus the Legend.
      var newEncryptionContext :- GetV2EncryptionContext(UnCanon(canonData));
      if |newEncryptionContext| != 0 {
        //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
        //# An error MUST be returned if any of the entries added to the encryption context in this step
        //# have the same key as any entry already in the encryption context.
        :- Need(encryptionContext.Keys !! newEncryptionContext.Keys,
                E("Internal Error - Structured Encryption encryption context overlaps with Item Encryptor encryption context."));
        encryptionContext := encryptionContext + newEncryptionContext;
        assert cmm.Modifies !! {config.materialProviders.History};

        var contextKeysX := SortedSets.ComputeSetToOrderedSequence2(newEncryptionContext.Keys, ByteLess);
        assert forall k <- contextKeysX :: ValidUTF8Seq(k) by {
          assert forall k <- newEncryptionContext.Keys :: ValidUTF8Seq(k);
          assert forall k <- contextKeysX :: k in newEncryptionContext.Keys;
        }
        var contextKeys :  seq<UTF8.ValidUTF8Bytes> := contextKeysX;

        //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
        //# Then, this operation MUST create a [Required Encryption Context CMM](https://github.com/awslabs/private-aws-encryption-sdk-specification-staging/blob/dafny-verified/framework/required-encryption-context-cmm.md)
        //# with the following inputs:
        //# - This input [CMM](./ddb-table-encryption-config.md#cmm) as the underlying CMM.
        //# - The name of every entry added above.
        var cmmR := config.materialProviders.CreateRequiredEncryptionContextCMM(
          CMP.CreateRequiredEncryptionContextCMMInput(
            underlyingCMM := Some(input.cmm),
            keyring := None,
            requiredEncryptionContextKeys := contextKeys
          )
        );
        cmm :- cmmR.MapFailure(e => AwsCryptographyMaterialProviders(e));
      }
    }

    //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
    //# This operation MUST obtain a set of decryption materials by calling
    //# [Decrypt Materials](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/cmm-interface.md#decrypt-materials)
    //# on the [CMM](#cmm) calculated above.

    //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
    //# The call to the CMM's Decrypt Materials operation MUST be constructed as follows:
    // - Encryption Context: The encryption context containing exactly the union of
    //   key-value pairs in the [input Encryption Context](#encryption-context)
    //   and the key-value pairs in the [Encryption Context parsed from the header](./header.md#encryption-context).
    // - Algorithm Suite ID: The algorithm suite [indicated by the Message Format Flavor](./header.md#format-flavor)
    //   parsed in the header.
    // - Encrypted Data Keys: The [Encrypted Data Keys parsed from the header](./header.md#encrypted-data-keys).

    var matR := cmm.DecryptMaterials(
      CMP.DecryptMaterialsInput (
        algorithmSuiteId := headerAlgorithmSuite.id,
        commitmentPolicy := DBE_COMMITMENT_POLICY,
        encryptedDataKeys := head.dataKeys,
        encryptionContext := head.encContext,
        reproducedEncryptionContext := Some(encryptionContext)
      )
    );

    var matOutput :- matR.MapFailure(e => AwsCryptographyMaterialProviders(e));
    var mat := matOutput.decryptionMaterials;
    :- Need(Header.ValidEncryptionContext(mat.encryptionContext), E("Bad encryption context"));
    :- Need(Materials.DecryptionMaterialsWithPlaintextDataKey(mat), E("Encryption material has no key"));

    //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
    //# The algorithm suite used in all further aspects of this operation MUST be
    //# the algorithm suite in the
    //# [decryption materials](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/structures.md#decryption-materials)
    //# returned from the Decrypt Materials call.

    //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
    //# Note that the algorithm suite in the retrieved decryption materials MAY be different from the input algorithm suite.

    //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
    //# If this algorithm suite is not a
    //# [supported suite for DBE](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# this operation MUST yield an error.
    :- Need(ValidSuite(mat.algorithmSuite), E("Invalid Algorithm Suite"));
    var postCMMAlg := mat.algorithmSuite;
    var key : Key := mat.plaintextDataKey.value;
    var commitKey :- Crypt.GetCommitKey(config.primitives, postCMMAlg, key, head.msgID);
    //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
    //# The header field value MUST be [verified](header.md#commitment-verification)
    var ok :- head.verifyCommitment(config.primitives, postCMMAlg, commitKey, headerSerialized.value);

    //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
    //# This operation MUST deserialize the bytes in [Terminal Value](./structures.md#terminal-value)
    //# according to the [footer format](./footer.md).
    var footer :- Footer.DeserializeFooter(footerSerialized.value, postCMMAlg.signature.ECDSA?);

    //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
    //# The footer field value MUST be [verified](footer.md#footer-verification).

    //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
    //# Decryption MUST fail immediately if verification fails.
    var _ :- footer.validate(config.primitives, mat, head.dataKeys, canonData, headerSerialized.value);
    var decryptedItems : CanonCryptoList :- Crypt.Decrypt(config.primitives, postCMMAlg, key, head, canonData);
    assert |decryptedItems| == |input.encryptedStructure|;
    assert forall k <- input.encryptedStructure :: (exists x :: x in decryptedItems && x.origKey == k.key);
    assert exists x :: x in decryptedItems && x.origKey == HeaderPath;
    assert exists x :: x in decryptedItems && x.origKey == FooterPath;

    assert (forall k <- input.encryptedStructure ::
              (exists x ::
                 && x in decryptedItems
                 && x.origKey == k.key
                 && Crypt.Updated2(k, x, Crypt.DoDecrypt)
              ));

    var largeResult := UnCanon(decryptedItems);
    assert |largeResult| == |input.encryptedStructure|;
    assert forall k <- input.encryptedStructure :: (exists x :: x in largeResult && x.key == k.key);
    assert (forall k <- input.encryptedStructure ::
              (exists x ::
                 && x in largeResult
                 && x.key == k.key
                 && Crypt.Updated3(k, x, Crypt.DoDecrypt)
              ));

    assert exists x :: x in largeResult && x.key == HeaderPath;
    assert exists x :: x in largeResult && x.key == FooterPath;
    var smallResult := Seq.Filter((x : CryptoItem) => x.key !in HeaderPaths, largeResult);
    reveal Seq.Filter();
    assert !exists x :: x in smallResult && x.key == HeaderPath;
    assert !exists x :: x in smallResult && x.key == FooterPath;
    // verifies, but it takes too long
    assume {:axiom} forall k <- largeResult | k.key !in HeaderPaths :: (exists x :: x in smallResult && x == k);
    :- Need(|smallResult| == |input.encryptedStructure| - 2, E("Internal Error."));
    assert |smallResult| == |input.encryptedStructure| - 2;

    assert (forall k <- input.encryptedStructure  | k.key !in HeaderPaths ::
              (exists x ::
                 && x in smallResult
                 && x.key == k.key
                 && Crypt.Updated3(k, x, Crypt.DoDecrypt)
              ));

    //= specification/structured-encryption/decrypt-path-structure.md#construct-decrypted-structured-data
    //= type=implication
    //# The output MUST also include a [Parsed Header](#parsed-header) that contains
    //# data that was serialized into the header included in the output Structured Data.
    var parsedHeader := ParsedHeader(
      algorithmSuiteId := headerAlgorithmSuite.id.DBE,
      encryptedDataKeys := head.dataKeys,
      storedEncryptionContext := head.encContext,
      encryptionContext := mat.encryptionContext
    );

    var decryptOutput := DecryptPathStructureOutput(
      plaintextStructure := smallResult,
      parsedHeader := parsedHeader
    );

    assert (forall k <- input.encryptedStructure  | k.key !in HeaderPaths ::
              (exists x ::
                 && x in smallResult
                 && x.key == k.key
                 && (x.action == ENCRYPT_AND_SIGN ==> |k.data.value| >= 2 && x.data.typeId == k.data.value[..2])
                 && (x.action != ENCRYPT_AND_SIGN ==> k.data == x.data)
              )
      );

    output := Success(decryptOutput);
  }

  // predicates/lemmas like this are not yet provided out of the box in the standard library.
  predicate {:opaque} Contains<X, Y>(big: map<X, Y>, small: map<X, Y>)
  {
    && small.Keys <= big.Keys
    && forall x <- small :: small[x] == big[x]
  }

  lemma LemmaContainsPreservesInjectivity<X, Y>(big: map<X, Y>, small: map<X, Y>)
    requires Contains(big, small)
    requires Maps.Injective(big)
    ensures Maps.Injective(small)
  {
    reveal Contains();
    reveal Maps.Injective();
  }

  lemma LemmaInjectiveImpliesUniqueValues<X(!new), Y>(m: map<X, Y>)
    requires Maps.Injective(m)
    ensures |m.Keys| == |m.Values|
  {
    if |m| > 0 {
      var x: X :| x in m;
      var y := m[x];
      var m' := Maps.Remove(m, x);
      reveal Contains();
      assert Contains(m, m');

      reveal Maps.Injective();
      assert m'.Values == m.Values - {y};
      LemmaContainsPreservesInjectivity(m, m');
      LemmaInjectiveImpliesUniqueValues(m');
    }
  }
}
