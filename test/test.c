int printf(const char *, ...);

// Line comment
int main(void) {
    int _ident32 = 0x0114514;
    char *s = "\"";
    float a = .251e+3l;
    float k = 123.f;
    float f = .1e1;
    int i = 100lu;
    i <<= 2;
    /* This
     * is
     * a block comment */
    printf("ご機嫌よう%d\n", _ident32);
    return 0;
}
