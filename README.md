
# Serde Format Tests

Compare different formats if they succeed in de/serialization roundtrip for a reasonable complex datatype.

The tested datastructure contains:
- Nested enums
- Circular structs
- f64::INFINITY
- Integer key in Hashmap

# Results

| Format       | Result                                                                                |
|--------------|---------------------------------------------------------------------------------------|
| Json         | Err(invalid type: null, expected f64 at line 1 column 81)                             |
| Postcard     | Err(Found an Option discriminant that wasn't 0 or 1)                                  |
| Ron          | Ok(())                                                                                |
| MessagePack  | Err(invalid length 0, expected struct IntermediateAggregationResults with 2 elements) |
| Bincode      | Err(invalid value: integer `10`, expected variant index 0 <= i < 2)                   |
| Ciborium     | Ok(())                                                                                |
| Bson         | Err(Invalid map key type: 10)                                                         |

