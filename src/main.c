#include <stdint.h>
#include <stdio.h>

extern int32_t double_input(int32_t input);

int main() {
    int c = 2;
    int a = add(c);
    int input = 4;
    int output = double_input(input);
    printf("%d * 2 = %d\n", input, output);
    return 0;
}
