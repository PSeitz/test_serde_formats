
# Serde Format Tests

Compare different formats if they succeed in de/serialization roundtrip for a reasonable complex datatype.

The tested datastructure contains:
- Nested enums
- Circular structs

# Results
| Format   | Result | Serialized Size | Serialize Time [ns] | Deserialize Time [ns] | Roundtrip Time [ns] |
|----------|--------|-----------------|---------------------|-----------------------|---------------------|
| Json     | Ok     | 2469160         | 4425223             | 8187162               | 12612385            |
| RON      | Ok     | 2179713         | 8544252             | 22065377              | 30609629            |
| Bincode  | Ok     | 960144          | 1752901             | 1970859               | 3723760             |
| Bitcode  | Ok     | 580069          | 1088287             | 2149733               | 3238020             |
| Ciborium | Ok     | 1749907         | 5114753             | 10164283              | 15279036            |




