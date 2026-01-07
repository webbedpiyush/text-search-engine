### Text Search Engine 

#### Handled massive Wikipedia dumps efficiently with the following key components:

- **Streaming Multi-Bz2 Decompression** Uses MultiBzDecoder to read concatenated compressed streams sequentially, enabling the processing of massive Wikipedia dumps without loading the full file into RAM.
  
- **Event-Driven SAX XML** Parsing Implements a custom state machine to process XML tags event-by-event, ensuring memory usage remains low and constant (O(1)) regardless of input size.
  
- **High-Performance Inverted Index** Maps unique tokens to Document IDs using a HashMap, allowing for instant, constant-time (O(1)) lookups during search queries.
  
- **Stemming and stop-word removal** (using rust-stemmers, stop-words)
  
- **Binary Serialization** Encodes the final in-memory index into a compact binary format using bincode, enabling the engine to load instantly on startup without re-indexing raw data.
  
