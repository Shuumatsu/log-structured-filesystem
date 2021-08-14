> https://people.eecs.berkeley.edu/~brewer/cs262/LFS.pdf
> http://www.eecs.harvard.edu/~cs161/notes/lfs.pdf
> https://lwn.net/Articles/353411/
> https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.52.6474&rep=rep1&type=pdf

LFS 把整个磁盘看做一个 append only log, 永远都是顺序写入。每当我们写入新文件时，我们总是顺序地追加在 log 的最后。LFS 的读取依旧依赖 random seeks。

与 Unix FFS 相同的，我们有一个 `Superblock` 记录了一些静态配置，包括段的数量和段的大小。

在 LFS 中，空余空间是用固定大小的 `Segment` 来管理的：硬盘被分割成固定大小的 `Segment`；写操作首先会被写入到内存中；当内存中缓存的数据超过 `SEGMENT_SIZE` 后，LFS 将数据一次性写入到空闲的 `Segment` 中。在每个段可以被重写之前，所有的已存在的数据必须已经被复制出去。通常使用 512KB 或 1MB 作为 `Segment` 的大小。

LFS 的读操作类似 FFS，LFS 的在段内存储文件内容时，也存储了文件的索引。每个文件都有对应一个叫做 `Inode` 的数据结构。只要一个文件的 `inode` 被找到，便能通过 `inode` 中存储的直接和间接指针找到文件的内容。
在 Unix FFS 中，这些 `inode` 存储在一个固定的位置，我们能够通过 `inode number` 直接定位。而在 LFS `中，inode` 被放置在日志中（非固定位置）。我们用 `InodeMap` 结构来记录每个 `inode` 的位置。`inode map` 被分成一个个数据块，然后写入日志中。
然后我们用存储在固定位置的 `CheckpointRegion` 记录所有 inode map 的数据块在磁盘中的位置。
在 LFS 中，所有的 `InodeMap` 内容都会被缓存到内容中，从而加快读取速度。
LFS 周期性的将内存中的最新版本的 `InodeMap` 更新到 `CheckpointRegion` 中。

在 LFS 中读取一个 inode 号为 i 的文件流程如下:
1. 从内存中缓存的 inode map 中找到所有的段中 inode 为 i 的地址
2. Read the inode into memory if it isn’t already in the buffer cache, and find the appropriate data pointer
3. 读取 data block 中的数据


LFS 中 directory 的存储和 FFS 中相同，一个 directory 可以看作包含 (name, inode) pairs 的文件。


因为我们永远不会对已写入的数据做修改操作，我们的系统需要对进行垃圾回收。LFS 以 Segment 为粒度执行垃圾回收，这样保证了 GC 的读写都是顺序的：读取 X partially invalid segments, and writes the live data to Y < X segments (which will be totally valid)。

在每个 `Segment` 的开头，有一个 `SegmentSummary` 结构记录了 `Segment` 中每个数据块所属的 inode number 以及每个块在所属的 inode 中的 offset。
LFS 通过 `SegmentSummary` 判断是否一个数据块 is live：根据 inode number 从 `InodeMap` 中读取 inode，判断 inode 中对应 offset 的指针是否指向该数据块。



依赖 Compaction 操作，将有效数据拷贝出来后重新紧凑的存储。



为了支持效益最佳的整理策略， LFS 维护了一个叫做 `SegmentUsageTable` 的数据结构。对于每一个 `Segment`，这个表格记录了每一个 `Segment` 的活跃（live）数据的大小，以及任何数据块的最后修改时间。
`SegmentUsageTable` 的数据块被写入日志中。然后我们用存储在固定位置的 `CheckpointRegion` 记录所有 `SegmentUsageTable` 的数据块在磁盘中的位置。


---

Crash Recovery

为了保证 `CheckpointRegion` 是原子更新的（不会因为崩溃处于不一致的状态），LFS 在硬盘的不同地方保存了两个副本。LFS 交替的使用这两个副本来存储 latest snapshot。更新的时候，先写下一个 timestamp，然后更新内容，最后再写下一个 timestamp。只有当这两个 timestamp 相同的时候，这个 `CheckpointRegion` 才处于一致状态。

如果 `CheckpointRegion` B 是无效的，我们总能回到上一个有效的 `CheckpointRegion` A。但是这之间的更新就丢失了，我们让 `CheckpointRegion` 记录更多的信息以支持前滚操作。
