`size-rs` will give you the size in memory that some rust and libc
structures takes on your system. May only work on Linux systems:

```
$ size-rs

Integer sizes:
        u8: 1 byte - 8 bits
        u16: 2 bytes - 16 bits
        u32: 4 bytes - 32 bits
        u64: 8 bytes - 64 bits
        u128: 16 bytes - 128 bits

Ids types:
        pid_t: 4 bytes - 32 bits
        uid_t: 4 bytes - 32 bits
        gid_t: 4 bytes - 32 bits
        id_t: 4 bytes - 32 bits

Files related sizes:
        ino_t: 8 bytes - 64 bits
        off_t: 8 bytes - 64 bits
        loff_t: 8 bytes - 64 bits
        dev_t: 8 bytes - 64 bits
        dirent: 280 bytes - 2240 bits
        ino64_t: 8 bytes - 64 bits
        off64_t: 8 bytes - 64 bits
        dirent64: 280 bytes - 2240 bits

Length sizes:
        size_t: 8 bytes - 64 bits
        ssize_t: 8 bytes - 64 bits
```
