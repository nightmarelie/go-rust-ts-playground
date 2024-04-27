#include <vector>
#include <stdio.h>

void print(std::vector<int> a) {
    printf("Vector");

    for (int i = 0; i < a.size(); i++) {
        printf("%d\n", a[i]);
    }
}

int main() {
    std::vector<int> a;
    std::vector<int> b = a;

    print(a);
    print(b);
}
