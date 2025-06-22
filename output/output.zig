const std = @import("std");




pub fn main() !void {
    const Rengler = enum {
    Qirmizi,
    Yasil,
};
    const reng = Rengler.Qirmizi;
    switch (reng) {
.Qirmizi => {
    std.debug.print("{s}\n", .{"Qirmizi"});
},
.Yasil => {
    std.debug.print("{s}\n", .{"Yasil"});
},
}
}
