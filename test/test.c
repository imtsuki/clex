int printf(const char *, ...);

int main(void) {
    int _ident32 = 0x0114514;
    char *s = "";
    int a = 1;
    int i = 2;
    a += 1;
    i <<= 2;
    printf("ご機嫌よう%d%d\n", _ident32);
    return 0;
}
