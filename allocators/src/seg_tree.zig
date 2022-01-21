const std = @import("std");
const Allocator = std.mem.Allocator;

/// Source: https://github.com/kth-competitive-programming/kactl/blob/main/content/data-structures/SegmentTree.h
/// `f` must be an associative function and `unit` must be the unit to that
/// function, i.e. `f(unit, x) == x` for every `x: T`
pub fn SegTree(comptime T: type, comptime f: fn (T, T) T, comptime unit: T) type {
    return struct {
        len: usize,
        array: []T,

        const This = @This();

        /// Asserts that the length of `array` is at least twice `len`
        pub fn initArray(len: usize, array: []T) This {
            std.debug.assert(array.len >= len * 2);
            return This{ .array = array, .len = len };
        }

        /// Uses a slice from `allocator`. `deinit` must be called when caller
        /// is done with the `SegTree`.
        pub fn initAlloc(len: usize, allocator: Allocator) !This {
            return This{ .array = try allocator.alloc(T, len * 2), .len = len };
        }

        /// Must be called if this `SegTree` was created with `initAlloc`,
        /// passing in the same allocator. Must not be called otherwise.
        pub fn deinit(this: *This, allocator: Allocator) void {
            allocator.free(this.array);
        }

        /// Queries [from, to). `O(log(len))` time complexity.
        pub fn query(this: *This, from: usize, to: usize) T {
            var ra = unit;
            var rb = unit;
            var a = from + this.len;
            var b = to + this.len;
            while (a < b) {
                if (a % 2 == 1) {
                    ra = f(ra, this.array[a]);
                    a += 1;
                }
                if (b % 2 == 1) {
                    b -= 1;
                    rb = f(this.array[b], rb);
                }
                a /= 2;
                b /= 2;
            }
            return f(ra, rb);
        }

        /// Sets the `idx`:th element to `val`. `O(log(len))` time complexity.
        pub fn update(this: *This, idx: usize, val: T) void {
            var i = idx + this.len;
            this.array[i] = val;
            i /= 2;
            while (i != 0) : (i /= 2) {
                this.array[i] = f(this.array[i * 2], this.array[i * 2 + 1]);
            }
        }

        /// "Fixes" all intermediary values so that operations return the
        /// correct values. Must be called after modifying the return value of
        /// `slice`. At other times, it has no effect. `O(len)` time complexity.
        pub fn fix(this: *This) void {
            var i = this.len - 1;
            while (i >= 1) : (i -= 1) {
                this.array[i] = f(this.array[i * 2], this.array[i * 2 + 1]);
            }
        }

        /// Returns the items that the segment tree in built on. If this is
        /// modified, `fix` must be called before other operations are
        /// performed.
        pub fn slice(this: *This) []T {
            return this.array[this.len .. this.len * 2];
        }
    };
}

const testing = std.testing;

fn max(a: usize, b: usize) usize {
    return if (a > b) a else b;
}

test "segtree" {
    var arr = [_]usize{0} ** 16;
    var s = SegTree(usize, max, 0).initArray(8, &arr);
    s.update(0, 10);
    try testing.expect(s.slice()[0] == 10);
    try testing.expect(s.slice()[1] == 0);
    s.update(1, 3);
    try testing.expect(s.slice()[1] == 3);
    try testing.expectEqual(s.query(0, 2), 10);
    var slice = s.slice();
    std.mem.copy(usize, slice[3..], &[_]usize{ 5, 7, 11 });
    s.fix();
    try testing.expectEqual(s.query(0, s.len), 11);
    try testing.expectEqual(s.query(3, 5), 7);
}
