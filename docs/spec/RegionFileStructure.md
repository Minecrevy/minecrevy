
# Region File Structure

The following table is generally how a region file is laid out.

<table>
<tr>
    <th colspan="2">Region File</th>
    <th>Field Name</th>
    <th>Field Type</th>
    <th>Notes</th>
<tr>
<tr>
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
    <td rowspan="6">Chunk 1</td>
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
    <td rowspan="3">Chunk data as a (usually-)compressed NBT compound.</td>
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
    <td colspan="4">Rest of sectors...</td>
</tr>
<tr>
    <td rowspan="5">Chunk 2</td>
    <td rowspan="3">Sector 1</td>
    <td>Length</td>
    <td>int32</td>
    <td>See above.</td>
</tr>
<tr>
    <td>Compression Type</td>
    <td>uint8</td>
    <td>See above.</td>
</tr>
<tr>
    <td>Data</td>
    <td>Array of 4091 bytes</td>
    <td rowspan="2">See above.</td>
</tr>
<tr>
    <td>Sector 2</td>
    <td>Data</td>
    <td>Array of 4096 bytes</td>
</tr>
<tr>
    <td colspan="4">Rest of sectors...</td>
</tr>
<tr>
    <td colspan="5">Rest of chunks...</td>
</tr>
</table>
