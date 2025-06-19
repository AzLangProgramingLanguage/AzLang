#include <cstdio>

int main() {
    for (int j = 1; j < 10000000; ++j) {
        printf("%d\n", j);
    }
    return 0;
}

//CPP  2,79s user 14,28s system 54% cpu 31,606 total
//Zig 8,33s user 25,78s system 78% cpu 21,593 total
