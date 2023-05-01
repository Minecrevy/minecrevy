use bitvec::{bitvec, order::Lsb0, vec::BitVec};

use crate::region::header::Offset;

pub struct Sectors;

impl Sectors {
    /// The size of a single sector (as a `u64`).
    pub const SIZE: u64 = 4096;

    /// The size of a single sector (as a `usize`).
    pub const USIZE: usize = Self::SIZE as usize;

    /// The size of a chunk's header stored at the beginning of the chunk's
    /// first sector. This includes the `i32` length and `u8` compression type.
    pub const CHUNK_HEADER_SIZE: usize = 5;
}

#[derive(Clone, Debug)]
pub struct UsedSectors(BitVec);

impl Default for UsedSectors {
    fn default() -> Self {
        // header always counts as 2 used sectors
        Self(bitvec![1, 1])
    }
}

impl UsedSectors {
    /// Marks the specified range of `sectors` as used.
    pub fn force(&mut self, offset: Offset) {
        let sectors = offset.sectors();

        if sectors.end >= self.0.len() {
            let additional = sectors.end - self.0.len() + 1;
            self.0.extend(bitvec![0; additional]);
        }

        for sector in sectors {
            self.0.set(sector, true);
        }
    }

    /// Marks the specified range of `sectors` as free.
    pub fn free(&mut self, offset: Offset) {
        for sector in offset.sectors() {
            self.0.set(sector, false);
        }
    }

    /// Finds enough free space for `num_sectors` and returns the index of the first sector.
    pub fn allocate(&mut self, num_sectors: usize) -> Offset {
        let mut start: usize = 0;

        loop {
            if let Some(free) = self.0[start..].first_zero().map(|idx| start + idx) {
                if let Some(used) = self.0[free..].first_one().map(|idx| free + idx) {
                    if used - free >= num_sectors {
                        // enough space between free and used bits
                        let sectors = Offset::try_from(free..(free + num_sectors))
                            .unwrap_or_else(|_| unreachable!());
                        self.force(sectors);
                        return sectors;
                    } else {
                        // not enough space between free and used bits
                        start = used;
                        continue;
                    }
                } else {
                    // No used bits after our free bit
                    let sectors = Offset::try_from(free..(free + num_sectors))
                        .unwrap_or_else(|_| unreachable!());
                    self.force(sectors);
                    return sectors;
                }
            } else {
                // No free bits in the current set.
                // Extend at the end.
                let sectors = Offset::try_from(self.0.len()..(self.0.len() + num_sectors))
                    .unwrap_or_else(|_| unreachable!());
                self.force(sectors);
                return sectors;
            }
        }
    }
}
