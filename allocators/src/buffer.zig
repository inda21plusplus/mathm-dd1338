const mem = @import("std").mem;
const Allocator = mem.Allocator;

/// A buffer of memory with a run-time known length and an optional pointer to a
/// previous buffer. The actual memory that the buffer holds is stored after the
/// `Buffer` struct, and this is managed by the buffer itself. Because of this,
/// you may never have a `Buffer`, but must always have a pointer to it, i.e.
/// a `*Buffer` which must never be dereferenced as this results in unchecked
/// undefind behavior.
pub fn Buffer(comptime T: type) type {
    return struct {
        pub const alignment = @maximum(@alignOf(This), @alignOf(T));
        pub const header_size = mem.alignForward(@sizeOf(This), @alignOf(T));

        const This = @This();

        prev: ?*This,
        // The size of the usable buffer in bytes. Does not include the bytes of
        // this struct itself.
        size: usize,

        pub fn init(len: usize, prev: ?*This, a: Allocator) error{OutOfMemory}!*This {
            var min_size = header_size + @sizeOf(T) * len;
            var buf = try a.allocAdvanced(u8, alignment, min_size, .at_least);
            var this = @ptrCast(*This, @alignCast(alignment, buf));
            this.prev = prev;
            this.size = buf.len - header_size;
            return this;
        }

        pub fn deinit(this: *This, a: Allocator) void {
            var ptr = @ptrCast([*]u8, this);
            var slice = ptr[0 .. header_size + this.size];
            a.rawFree(slice, alignment, @returnAddress());
        }

        pub fn data(this: *This) []T {
            var ptr = @ptrToInt(this) + header_size;
            var len = this.size / @sizeOf(T);

            return @intToPtr([*]T, ptr)[0..len];
        }
    };
}
