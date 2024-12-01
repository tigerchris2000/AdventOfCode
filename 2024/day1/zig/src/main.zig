const std = @import("std");
const print = std.debug.print;
const page_allcoator = std.heap.page_allocator;

pub fn main() !void {
    const reader = std.io.getStdIn().reader();
    var left = std.ArrayList(u32).init(page_allcoator);
    defer left.deinit();
    var right = std.ArrayList(u32).init(page_allcoator);
    defer right.deinit();
    while (true) {
        const data = reader.readUntilDelimiterAlloc(page_allcoator, '\n', 256) catch |err| {
            if (err == error.EndOfStream) {
                break;
            } else {
                return err;
            }
        };
        var it = std.mem.split(u8, data, "   ");
        var val = try std.fmt.parseInt(u32, it.next().?, 10);
        try left.append(val);
        val = try std.fmt.parseInt(u32, it.next().?, 10);
        try right.append(val);
        page_allcoator.free(data);
    }
    std.mem.sort(u32, left.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, right.items, {}, comptime std.sort.asc(u32));

    print("task 1 solution:\n", .{});
    try solve_task1(left, right);
    print("task 2 solution:\n", .{});
    try solve_task2(left, right);
}

fn cmpByValue(context: void, a: u32, b: u32) bool {
    return std.sort.asc(u32)(context, a, b);
}

fn solve_task1(left: std.ArrayList(u32), right: std.ArrayList(u32)) !void {
    var val: u32 = 0;
    for (left.items, 0..) |_, i| {
        if (right.items[i] > left.items[i]) {
            val += right.items[i] - left.items[i];
        } else {
            val += left.items[i] - right.items[i];
        }
    }
    std.debug.print("val: {d}\n", .{val});
}
fn solve_task2(left: std.ArrayList(u32), right: std.ArrayList(u32)) !void {
    var val: u32 = 0;
    for (left.items) |l| {
        var count: u32 = 0;
        for (right.items) |r| {
            if (l == r) {
                count += 1;
            }
        }
        val += l * count;
    }
    std.debug.print("val: {d}\n", .{val});
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
