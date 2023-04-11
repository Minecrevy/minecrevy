
# Region File Structure

The following table is generally how a region file is laid out.

<table>
<tr>
    <th colspan="3">Region File</th>
    <th>Field Name</th>
    <th>Field Type</th>
    <th>Notes</th>
<tr>
<tr>
    <td rowspan="5">Header</td>
    <td rowspan="3">Offset Table</td>
    <td rowspan="2">Offset</td>
    <td>Index</td>
    <td>uint24</td>
    <td>The sector-index (not array-index) of the first sector of a chunk.</td>
</tr>
<tr>
    <td>Count</td>
    <td>uint8</td>
    <td>The number of sectors a chunk uses.</td>
</tr>
<tr>
    <td colspan="4">1023 more offsets...</td>
</tr>
<tr>
    <td rowspan="2">Timestamp Table</td>
    <td>Timestamp</td>
    <td>Last Time Modified</td>
    <td>int32</td>
    <td>The last time (in seconds since Unix epoch) a chunk was modified.</td>
</tr>
<tr>
    <td colspan="4">1023 more timestamps...</td>
</tr>
<tr>
    <td rowspan="13">Sectors</td>
    <td rowspan="6">Chunk A</td>
    <td rowspan="3">Sector 1</td>
    <td>Length</td>
    <td>int32</td>
    <td>Number of bytes required to store the chunk.</td>
</tr>
<tr>
    <td>Compression Type</td>
    <td>uint8</td>
    <td>The type of compression used to store the chunk. 1 for GZip, 2 for ZLib, 3 for none.</td>
</tr>
<tr>
    <td>Data</td>
    <td>Array of 4091 bytes</td>
    <td rowspan="4">Chunk data as a (usually-)compressed NBT compound.</td>
</tr>
<tr>
    <td>Sector 2</td>
    <td>Data</td>
    <td>Array of 4096 bytes</td>
</tr>
<tr>
    <td>Sector 3</td>
    <td>Data</td>
    <td>Array of 4096 bytes</td>
</tr>
<tr>
    <td colspan="3">Rest of sectors...</td>
</tr>
<tr>
    <td colspan="5">Potentially empty sectors...</td>
</tr>
<tr>
    <td rowspan="5">Chunk B</td>
    <td rowspan="3">Sector 1</td>
    <td>Length</td>
    <td>int32</td>
    <td>Chunk data length.</td>
</tr>
<tr>
    <td>Compression Type</td>
    <td>uint8</td>
    <td>Type of compression.</td>
</tr>
<tr>
    <td>Data</td>
    <td>Array of 4091 bytes</td>
    <td rowspan="3">Chunk data.</td>
</tr>
<tr>
    <td>Sector 2</td>
    <td>Data</td>
    <td>Array of 4096 bytes</td>
</tr>
<tr>
    <td colspan="3">Rest of sectors...</td>
</tr>
<tr>
    <td colspan="5">Rest of chunks...</td>
</tr>
</table>

## How to load a chunk

### 1. Find the correct region file, given chunk coordinates.

Calculate the region coordinates:
```java
int chunkX = ...;
int chunkZ = ...;

int regionX = chunkX / 32;
int regionZ = chunkZ / 32;
```

### 2. Index into the offset table, given chunk coordinates.

Calculate the table index:
```java
int regionLocalX = chunkX % 32;
int regionLocalZ = chunkZ % 32;

int tableIndex = regionLocalX + regionLocalZ * 32;
int offset = offsetTable.getOffset(tableIndex);

int offsetIndex = offset >> 8 & 0xFF_FF_FF;
int offsetCount = offset & 0xFF;
```

### 3. Collect the bytes for all the given chunk's sectors combined.

```java
// if offset.index and offset.count are both zero, the chunk has not been saved yet.

byte[] data = sectors.getByteRange(offsetIndex, offsetCount);
```

### 4. Split the data into its components: length, compression, chunk data.

```java
int length = data.readInt(data);
byte compression = data.readByte(data);
byte[] chunkData = data.readN(length);
```

### 5. Decompress the chunk data.

```java
NbtCompound chunk = switch (compression) {
    case 1 -> nbtDecodeGzip(chunkData),
    case 2 -> nbtDecodeZlib(chunkData),
    case 3 -> nbtDecode(chunkData),
    case 4 -> throw new Exception("invalid compression type"),
};

// Now we use it!
```

## How to save a chunk

TODO
