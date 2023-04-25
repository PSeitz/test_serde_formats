
# Serde Format Tests

Compare different formats if they succeed in de/serialization roundtrip for a reasonable complex datatype.

The tested datastructure contains:
- Nested enums
- Circular structs

# Results

| Format   | Result | Serialized Size | Serialize Time | Deserialize Time |
|----------|--------|-----------------|----------------|------------------|
| Json     | Ok     | 690             | 6242           | 10239            |
| RON      | Ok     | 1196            | 12253          | 19146            |
| Bincode  | Ok     | 309             | 1213           | 2825             |
| Bitcode  | Ok     | 150             | 1834           | 2515             |
| Ciborium | Ok     | 486             | 3356           | 9918             |


