VFS 层给出了两个关键的抽象，`trait INode` 以及 `trait FileSystem`，更高层级的文件操作进入到 VFS 子系统后根据文件系统类型被分发到具体的实现上去。

```rust
pub trait INode: Any + Sync + Send {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize>;
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize>;
    fn poll(&self) -> Result<PollStatus>;
    fn metadata(&self) -> Result<Metadata> { Err(FsError::NotSupported) }
    fn set_metadata(&self, _metadata: &Metadata) -> Result<()> { Err(FsError::NotSupported) }
    fn sync_all(&self) -> Result<()> { Err(FsError::NotSupported) }
    fn sync_data(&self) -> Result<()> { Err(FsError::NotSupported) }
    fn resize(&self, _len: usize) -> Result<()> { Err(FsError::NotSupported) }
    /// Create a new INode in the directory
    fn create(&self, name: &str, type_: FileType, mode: u32) -> Result<Arc<dyn INode>> {
        self.create2(name, type_, mode, 0)
    }

    /// Create a new INode in the directory, with a data field for usages like device file.
    fn create2(
        &self,
        name: &str,
        type_: FileType,
        mode: u32,
        _data: usize,
    ) -> Result<Arc<dyn INode>> {
        self.create(name, type_, mode)
    }

    /// Create a hard link `name` to `other`
    fn link(&self, _name: &str, _other: &Arc<dyn INode>) -> Result<()> {
        Err(FsError::NotSupported)
    }

    /// Delete a hard link `name`
    fn unlink(&self, _name: &str) -> Result<()> {
        Err(FsError::NotSupported)
    }

    /// Move INode `self/old_name` to `target/new_name`.
    /// If `target` equals `self`, do rename.
    fn move_(&self, _old_name: &str, _target: &Arc<dyn INode>, _new_name: &str) -> Result<()> {
        Err(FsError::NotSupported)
    }

    /// Find the INode `name` in the directory
    fn find(&self, _name: &str) -> Result<Arc<dyn INode>> {
        Err(FsError::NotSupported)
    }

    /// Get the name of directory entry
    fn get_entry(&self, _id: usize) -> Result<String> {
        Err(FsError::NotSupported)
    }

    /// Control device
    fn io_control(&self, _cmd: u32, _data: usize) -> Result<()> {
        Err(FsError::NotSupported)
    }

    /// Map files or devices into memory
    fn mmap(&self, _area: MMapArea) -> Result<()> {
        Err(FsError::NotSupported)
    }

    /// Get the file system of the INode
    fn fs(&self) -> Arc<dyn FileSystem> {
        unimplemented!();
    }

    /// This is used to implement dynamics cast.
    /// Simply return self in the implement of the function.
    fn as_any_ref(&self) -> &dyn Any;
}

pub trait FileSystem: Sync + Send {
    fn sync(&self) -> Result<()>;
    fn root_inode(&self) -> Arc<dyn INode>;
    fn info(&self) -> FsInfo;
}
```

我们通过实现了 `trait FileSystem` 的对象得到 root 的 `trait INode` 实现，此后的文件操作便从 root 开始执行。

我们围绕这两个 trait 实现我们的文件系统。