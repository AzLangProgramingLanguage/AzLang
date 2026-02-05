// Source - https://stackoverflow.com/a/79817054
// Posted by spicy.dll, modified by community. See post 'Timeline' for change history
// Retrieved 2026-02-05, License - CC BY-SA 4.0

const std = @import("std");

var input_buf: [1024]u8 = undefined;
var stdin_reader = std.fs.File.stdin().reader(&input_buf);

const stdin = &stdin_reader.interface;

// Same goes for stdout
var output_buf: [1024]u8 = undefined;
var stdout_writer = std.fs.File.stdout().writer(&output_buf);
const stdout = &stdout_writer.interface;

fn ask_user(reader: *std.Io.Reader, writer: *std.Io.Writer) !i64 {
    try writer.print("A number please: ", .{});
    try writer.flush();

    // Reads data into input_buf and returns a slice to it
    // This slice is only valid until the next "peek" operation (take does a peek)
    const line = try reader.takeDelimiterExclusive('\n');
    return std.fmt.parseInt(i64, line, 10);
}

pub fn main() !void {
    const value = try ask_user(stdin, stdout);

    try stdout.print("You wrote: {d}\n", .{value});
    // don't forget to flush
    try stdout.flush();
}

