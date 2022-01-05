const std = @import("std");
const mem = std.mem;
const Allocator = mem.Allocator;

const ArenaAllocator = @This();

const Buffer = struct {
    prev: ?*Buffer,
    data: [LEN]u8, // TODO: variable size

    pub const LEN = 4096 - @sizeOf(*Buffer);
};

underlying: Allocator,
buffer: ?*Buffer,
current_index: usize,

pub fn init(underlying: Allocator) ArenaAllocator {
    return ArenaAllocator{ .underlying = underlying, .buffer = null, .current_index = 0 };
}

pub fn allocator(this: *ArenaAllocator) Allocator {
    return Allocator.init(this, alloc, resize, free);
}

fn create_buffer(this: *ArenaAllocator) error{OutOfMemory}!*Buffer {
    var new_buffer = @ptrCast(*Buffer, @alignCast(@alignOf(Buffer), try this.underlying.rawAlloc(@sizeOf(Buffer), @alignOf(Buffer), 1, @returnAddress())));
    new_buffer.prev = this.buffer;
    this.buffer = new_buffer;

    return new_buffer;
}

fn alloc(this: *ArenaAllocator, len: usize, ptr_align: u29, len_align: u29, ret_addr: usize) error{OutOfMemory}![]u8 {
    _ = len_align;
    _ = ret_addr;

    var buffer = if (this.buffer == null or this.current_index + len > this.buffer.?.data.len)
        try this.create_buffer()
    else
        this.buffer.?;

    var unaligned_ptr = &buffer.data[this.current_index];
    var ptr = mem.alignForward(@ptrToInt(unaligned_ptr), ptr_align);
    var index = ptr - @ptrToInt(&buffer.data);
    var end_index = index + len;
    var buf = buffer.data[index..end_index];
    this.current_index = end_index;

    return buf;
}

fn resize(this: *ArenaAllocator, buf: []u8, buf_align: u29, new_len: usize, len_align: u29, ret_addr: usize) ?usize {
    _ = buf_align;
    _ = len_align;
    _ = ret_addr;

    var buffer = this.buffer orelse return if (new_len <= buf.len) new_len else null;

    if (@ptrToInt(&buffer.data) + this.current_index != @ptrToInt(buf.ptr) + buf.len)
        return if (new_len <= buf.len) new_len else null;

    if (this.current_index - buf.len + new_len > buffer.data.len)
        return null;

    this.current_index = this.current_index - buf.len + new_len;
    return new_len;
}

fn free(this: *ArenaAllocator, buf: []u8, buf_align: u29, ret_addr: usize) void {
    _ = buf_align;
    _ = ret_addr;

    var buffer = this.buffer orelse return;

    if (@ptrToInt(&buffer.data) + this.current_index == @ptrToInt(buf.ptr) + buf.len)
        this.current_index -= buf.len;
}

pub fn deinit(this: *ArenaAllocator) void {
    while (this.buffer) |buffer| {
        var prev: ?*Buffer = buffer.prev;
        this.underlying.rawFree(@intToPtr([*]u8, @ptrToInt(buffer))[0..@sizeOf(Buffer)], @alignOf(Buffer), @returnAddress());
        this.buffer = prev;
    }
}
