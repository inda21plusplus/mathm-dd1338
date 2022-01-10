//! A buffer of memory with a run-time known length and an optional pointer to a
//! previous buffer. The actual memory that the buffer holds is stored after the
//! `Buffer` struct, and this is managed by the buffer itself. Because of this,
//! you may never have a `Buffer`, but must always have a pointer to it, i.e.
//! `*Buffer` which must never be dereferenced.

// TODO: check if its possible to add an
// `this.original_location = @ptrToInt(this)` only when assertions are turned on
// or whatever and to be able to check if the buffer has been moved when calling
// `data`.

const Allocator = @import("std").mem.Allocator;

const Buffer = @This();

prev: ?*Buffer,
len: usize,

pub fn init(len: usize, prev: ?*Buffer, a: Allocator) error{OutOfMemory}!*Buffer {
    var size = @sizeOf(Buffer) + len;
    // TODO: use `allocAdvancedWithOptions` or whatever
    var buf = try a.rawAlloc(size, @alignOf(Buffer), 1, @returnAddress());
    var this = @ptrCast(*Buffer, @alignCast(@alignOf(Buffer), buf));
    this.prev = prev;
    this.len = buf.len - @sizeOf(Buffer);
    return this;
}

pub fn deinit(this: *Buffer, a: Allocator) void {
    var ptr = @ptrCast([*]u8, this);
    var slice = ptr[0 .. @sizeOf(Buffer) + this.len];
    a.rawFree(slice, @alignOf(Buffer), @returnAddress());
}

pub fn data(this: *Buffer) []u8 {
    var ptr = @intToPtr([*]u8, @ptrToInt(this) + @sizeOf(Buffer));
    return ptr[0..this.len];
}
