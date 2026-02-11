const std = @import("std");
const c = @cImport({
    @cInclude("stdio.h");
});

pub fn main() void {
    var isim: [100]u8 = undefined;

    _ = c.printf("Adınızı yazın: ");
    _ = c.scanf("%s", &isim);

    const temiz_isim = std.mem.span(@as([*c]u8, &isim));
    
    std.debug.print("Merhaba: {s}\n", .{temiz_isim});
}
