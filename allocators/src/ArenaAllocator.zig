//! A fast memory allocator that doesn't free underlying memory until the
//! allocator itself is deinitialized. Allocating is most often O(1) but
//! sometimes calls to the underlying allocator. Freeing is always O(1) but is
//! most often a noop.
//!
//! ## Example:
//! ```
//! var arena = ArenaAllocator.init(std.heap.page_allocator);
//! defer arena.deinit();
//! var allocator = arena.allocator();
//! var x = try allocator.create(u64);
//! allocator.destroy(x);
//! ```

const std = @import("std");
const mem = std.mem;
const Allocator = mem.Allocator;

const Buffer = @import("buffer.zig").Buffer;

const ArenaAllocator = @This();

underlying: Allocator,
buffer: ?*Buffer(u8),
current_index: usize,

pub fn init(underlying: Allocator) ArenaAllocator {
    return ArenaAllocator{ .underlying = underlying, .buffer = null, .current_index = 0 };
}

pub fn allocator(this: *ArenaAllocator) Allocator {
    return Allocator.init(this, alloc, resize, free);
}

fn create_buffer(this: *ArenaAllocator, required_len: usize) error{OutOfMemory}!*Buffer(u8) {
    var len = @maximum(4096 - Buffer(u8).header_size, required_len);

    this.buffer = try Buffer(u8).init(len, this.buffer, this.underlying);

    this.current_index = 0;
    return this.buffer.?;
}

fn alloc(this: *ArenaAllocator, len: usize, ptr_align: u29, len_align: u29, ret_addr: usize) error{OutOfMemory}![]u8 {
    _ = len_align;
    _ = ret_addr;

    var buffer = if (this.buffer == null or this.current_index + len > this.buffer.?.data().len)
        try this.create_buffer(len)
    else
        this.buffer.?;

    var unaligned_ptr = &buffer.data()[this.current_index];
    var ptr = mem.alignForward(@ptrToInt(unaligned_ptr), ptr_align);
    var index = ptr - @ptrToInt(buffer.data().ptr);
    var end_index = index + len;
    var buf = buffer.data()[index..end_index];
    this.current_index = end_index;

    return buf;
}

fn resize(this: *ArenaAllocator, buf: []u8, buf_align: u29, new_len: usize, len_align: u29, ret_addr: usize) ?usize {
    _ = buf_align;
    _ = len_align;
    _ = ret_addr;

    var buffer = this.buffer orelse return if (new_len <= buf.len) new_len else null;

    if (@ptrToInt(buffer.data().ptr) + this.current_index != @ptrToInt(buf.ptr) + buf.len)
        return if (new_len <= buf.len) new_len else null;

    if (this.current_index - buf.len + new_len > buffer.data().len)
        return null;

    this.current_index = this.current_index - buf.len + new_len;
    return new_len;
}

fn free(this: *ArenaAllocator, buf: []u8, buf_align: u29, ret_addr: usize) void {
    _ = buf_align;
    _ = ret_addr;

    var buffer = this.buffer orelse return;

    if (@ptrToInt(buffer.data().ptr) + this.current_index == @ptrToInt(buf.ptr) + buf.len)
        this.current_index -= buf.len;
}

pub fn deinit(this: *ArenaAllocator) void {
    while (this.buffer) |buffer| {
        var prev = buffer.prev;
        buffer.deinit(this.underlying);
        this.buffer = prev;
    }
}
