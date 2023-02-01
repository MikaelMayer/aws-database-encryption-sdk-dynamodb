
# Structured Encryption Header

## Header Format

The serialized form of the header MUST be

| Length (bytes) | Meaning |
|---|---|
| 1 | [Format Version](#format-version) |
| 1 | [Format Flavor](#format-flavor) |
| 32 | [Message ID](#message-id) |
| Variable | [Encrypt Legend](#encrypt-legend) |
| Variable | [Encryption Context](#encryption-context) |
| Variable | [Encrypted Data Keys](#encrypted-data-keys) |
| 32 | [Header Commitment](#header-commitment) |

### Format Version

The message format version dictates the cryptographic algorithms supported
as well as the specific serialized header and footer format.

The Version MUST be `0x01`.

### Format Flavor

The flavor currently only encodes whether signatures are enabled for this record.
The flavor MUST be one of these two values.

| Value | Meaning |
|---|---|
| 0x01 | Signatures enabled (default) |
| 0x00 | Signatures disabled |

When signatures are enabled in the Format Flavor, a signature will be included in the
[footer](./footer.md#signature).

### Message ID

The Message ID is a random 256-bit value.

Implementations MUST generate a fresh 256-bit random MessageID, from a cryptographically secure source, for each record encrypted.

### Encrypt Legend

The Encrypt Legend is a serialized description of which authenticated fields in the record were encrypted.

The Encrypt Legend MUST be serialized as

| Field | Length (bytes) | Interpreted as |
| ----- | -------------- | -------------- |
| Encrypt Legend Length | 2 | big endian UInt16 |
| [Encrypt Legend Bytes](#encrypt-legend-bytes) | Variable. Equal to the value specified in the previous 2 bytes (Encrypt Legend Length). | Bytes |

#### Encrypt Legend Bytes

Each Crypto Action MUST be encoded as follows

- `0x65` (`e` in UTF-8, for "Encrypt and Sign") means that a particular field was encrypted
  and included in the signature calculation.
  This indicates that this field MUST be attempted to be decrypted during decryption.
- `0x73` (`s` in UTF-8, for "Sign Only") means that a particular field was not encrypted,
  but still included in the signature calculation.
  This indicates that this field MUST NOT be attempted to be decrypted during decryption.
- no entry if the attribute is not signed

The Encrypt Legend Bytes MUST be serialized as follows:

1. Order every authenticated attribute in the item lexicographically by the attribute name.
TODO - we must sort by the Canoncial Path, which is slightly different, even for just plain
attribute names in a single table.
2. For each authenticated terminal, in order,
append one of the byte values specified above to indicate whether
that field should be encrypted.

The length of this serialized value (in bytes) MUST equal the number of authenticated fields indicated
by the caller's [Authenticate Schema](./structures.md#authenticate-schema).

### Encryption Context

TODO: Link directly to the MPL definition once it is more generalized.

The Encryption Context MUST be serialized as follows

| Field | Length (bytes) | Interpreted as |
| ----- | -------------- | -------------- |
| [Key Value Pair Count](#key-value-pair-count) | 2 | big endian UInt16 |
| [Key Value Pair Entries](#key-value-pair-entries) | Variable. Determined by the count and length of each key-value pair. | [Key Value Pair Entries](#key-value-pair-entries) |

#### Key Value Pair Count

The number of key-value pairs within the [Key Value Pair Entries](#key-value-pair-entries) field.

#### Key Value Pair Entries

A sequence of one or more key-value pair entries.

This sequence MUST NOT contain duplicate entries.

These entries MUST have entries sorted, by key,
in ascending order according to the UTF-8 encoded binary value.

Each Key Value Pair MUST be serialized as follows

| Field | Length (bytes) | Interpreted as |
| ----- | -------------- | -------------- |
| Key Length | 2 | big endian UInt16 |
| Key | Variable. Equal to the value specified in the previous 2 bytes (Key Length). | UTF-8 encoded bytes |
| Value Length | 2 | big endian UInt16 |
| Value | Variable. Equal to the value specified in the previous 2 bytes (Value Length). | UTF-8 encoded bytes |

### Encrypted Data Keys

TODO : This is the same as in ESDK and MPL. Should they be unified somehow?

The Encrypted Data Keys MUST be serialized as follows

| Field | Length (bytes) | Interpreted as |
| ----- | -------------- | -------------- |
| [Encrypted Data Key Count](#encrypted-data-key-count) | 1 | unsigned 8 bit number |
| [Encrypted Data Key Entries](#encrypted-data-key-entries) | Variable. Determined by the count and length of each key-value pair. | [Encrypted Data Key Entries](#encrypted-data-key-entries) |

#### Encrypted Data Key Count

The number of encrypted data keys.
This value MUST be greater than 0.

#### Encrypted Data Key Entries

Each Encrypted Data Key MUST be serialized as follows

| Field | Length (bytes) | Interpreted as |
| ----- | -------------- | -------------- |
| [Key Provider ID Length](#key-provider-id-length)  | 2 | big endian UInt16 |
| [Key Provider ID](#key-provider-id) | Variable. Equal to the value specified in the previous 2 bytes (Key Provider ID Length). | UTF-8 encoded bytes |
| [Key Provider Information Length](#key-provider-information-length) | 2 | big endian UInt16 |
| [Key Provider Information](#key-provider-information) | Variable. Equal to the value specified in the previous 2 bytes (Key Provider Information Length). | Bytes |
| [Encrypted Data Key Length](#encrypted-data-key-length) | 2 | big endian UInt16 |
| [Encrypted Data Key](#encrypted-data-key) | Variable. Equal to the value specified in the previous 2 bytes (Encrypted Data Key Length). | Bytes |

##### Key Provider ID Length

The length of the key provider ID.

##### Key Provider ID

The key provider ID.

##### Key Provider Information Length

The length of the key provider information.

##### Key Provider Information

The key provider information.

##### Encrypted Data Key Length

The length of the encrypted data key.

##### Encrypted Data Key

The encrypted data key.
It is the data key encrypted by the key provider.

### Header Commitment

The Header Commitment MUST be calculated as a 256-bit HmacSha384,
with all preceding header bytes as the message
and a commitment key of "TRUSS_COMMIT_KEY"
TODO - use real commitment key

```
def GetHeaderCommitment(Header, CommitKey):
    # Truncate to 256 bits (32 bytes; returning only the leftmost bits)
    return HmacSha384(Header, CommitKey)[0:32]

def CheckHeaderCommitment(Header, CommitKey):
    Commitment = Header[-32:32]  # Last 32 bytes of the header
    Preceding = Header[:-32]     # Everything before the Header Commitment
    return ConstantTimeEquals(Commitment, GetHeaderCommitment(Preceding, CommitKey)    
```

It's important to note that, while the Header Commitment does
produce a distinct 256-bit hash output per header and commitment key,
it does not provide any integrity guarantees over the encrypted attributes.
Integrity over the encrypted attributes is ensured by the signatures in the footer.

When reading or deserializing a Header, the implementation MUST recalculate the commitment,
and fail if the calculated commitment does not match the stored commitment.