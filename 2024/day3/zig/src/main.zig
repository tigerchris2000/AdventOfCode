const std = @import("std");
const page_allocator = std.heap.page_allocator;

pub fn main() !void {
    try task1();
}

fn task1() !void {
    const file = try std.fs.cwd().openFile("../input.txt", {});
    const fr = file.reader();
    const data = try fr.readAllAlloc(page_allocator, 1024 * 1024);
    defer page_allocator.free(data);
    const mul_str = "mul(";
    var mul: bool = false;
    var comma: bool = false;
    var relative = 0;
    for (data) |c| {
        if (!mul and c == mul_str[relative]) {
            if (c == '(') {
                mul = true;
            }
            relative += 1;
        } else if (!comma and c >= 48 and c <= 57) {
            comma = true;
        }
    }
}

fn task2() !void {}
