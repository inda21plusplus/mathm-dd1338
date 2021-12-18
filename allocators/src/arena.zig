const std = @import("std");
const Allocator = std.mem.Allocator;

const ArenaAllocator = @This();

underlying: Allocator,

pub fn init(underlying: Allocator) ArenaAllocator {
    return ArenaAllocator{ .underlying = underlying };
}

pub fn allocator(this: *ArenaAllocator) Allocator {
    return Allocator.init(this, alloc, resize, free);
}

fn alloc(this: *ArenaAllocator, len: usize, ptr_align: u29, len_align: u29, ret_addr: usize) error{OutOfMemory}![]u8 {
    _ = this;
    _ = len;
    _ = ptr_align;
    _ = len_align;
    _ = ret_addr;
    unreachable;
}

fn resize(this: *ArenaAllocator, buf: []u8, buf_align: u29, new_len: usize, len_align: u29, ret_addr: usize) ?usize {
    _ = this;
    _ = buf;
    _ = buf_align;
    _ = new_len;
    _ = len_align;
    _ = ret_addr;
    unreachable;
}

fn free(this: *ArenaAllocator, buf: []u8, buf_align: u29, ret_addr: usize) void {
    _ = this;
    _ = buf;
    _ = buf_align;
    _ = ret_addr;
    unreachable;
}

pub fn deinit(this: *ArenaAllocator) void {
    _ = this;
    unreachable;
}
