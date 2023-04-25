
# Serde Format Tests

Compare different formats if they succeed in de/serialization roundtrip for a reasonable complex datatype.

The tested datastructure contains:
- Nested enums
- Circular structs

# Results

| Format   | Result | Serialized Size | Serialize Time [ns] | Deserialize Time [ns] |
|----------|--------|-----------------|---------------------|-----------------------|
| Json     | Ok     | 2469160         | 5123010             | 8810734               |
| RON      | Ok     | 2179713         | 8993176             | 24152544              |
| Bincode  | Ok     | 960144          | 1759824             | 1659657               |
| Bitcode  | Ok     | 580069          | 1165111             | 2183037               |
| Ciborium | Ok     | 1749907         | 5266750             | 10346990              |


