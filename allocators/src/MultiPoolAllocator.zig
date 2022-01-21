const std = @import("std");
const mem = std.mem;
const Allocator = std.mem.Allocator;

const Buffer = @import("buffer.zig").Buffer;
const PoolAllocator = @import("pool_allocator.zig").PoolAllocator;

const This = @This();

const LastFreed = ?*LastFreed;

underlying: Allocator,
pool8: PoolAllocator(u64),
pool16: PoolAllocator(u128),
pool32: PoolAllocator(u256),
pool64: PoolAllocator(u512),
pool128: PoolAllocator(u1024),
pool256: PoolAllocator(u2048),
pool512: PoolAllocator(u4096),
pool1024: PoolAllocator(u8192),

pub fn init(underlying: Allocator) This {
    return .{
        .underlying = underlying,
        .pool8 = PoolAllocator(u64).init(underlying),
        .pool16 = PoolAllocator(u128).init(underlying),
        .pool32 = PoolAllocator(u256).init(underlying),
        .pool64 = PoolAllocator(u512).init(underlying),
        .pool128 = PoolAllocator(u1024).init(underlying),
        .pool256 = PoolAllocator(u2048).init(underlying),
        .pool512 = PoolAllocator(u4096).init(underlying),
        .pool1024 = PoolAllocator(u8192).init(underlying),
    };
}

pub fn deinit(this: *This) void {
    this.pool8.deinit();
    this.pool16.deinit();
    this.pool32.deinit();
    this.pool64.deinit();
    this.pool128.deinit();
    this.pool256.deinit();
    this.pool512.deinit();
    this.pool1024.deinit();
}

pub fn allocator(this: *This) Allocator {
    return Allocator.init(this, alloc, Allocator.NoResize(This).noResize, free);
}

fn alloc(this: *This, len: usize, ptr_align: u29, len_align: u29, ret_addr: usize) error{OutOfMemory}![]u8 {
    _ = ptr_align;
    _ = ret_addr;

    // if (len <= 8) return @ptrCast([*]u8, try this.pool8.create())[0..8];

    inline for ([_][]const u8{
        "8", "16", "32", "64", "128", "256", "512", "1024",
    }) |s| {
        const si = std.fmt.parseInt(u64, s, 10) catch unreachable;
        if (len <= si) {
            const ptr = try @field(this, "pool" ++ s).create();
            const u8_ptr = @ptrCast([*]u8, ptr);
            const size = if (len_align == 0) len else mem.alignBackward(si, len_align);
            return u8_ptr[0..size];
        }
    }

    return error.OutOfMemory;
}

fn free(this: *This, buf: []u8, buf_align: u29, ret_addr: usize) void {
    _ = buf_align;
    _ = ret_addr;

    inline for ([_][]const u8{
        "8", "16", "32", "64", "128", "256", "512", "1024",
    }) |s| {
        const si = std.fmt.parseInt(u64, s, 10) catch unreachable;
        if (buf.len <= si) {
            const pool = &@field(this, "pool" ++ s);
            pool._free(@ptrCast([*]u8, buf)[0..si], buf_align, @returnAddress());
            return;
        }
    }
}
