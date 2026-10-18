/* Freestanding i386 harness: runs functions of the reference C build on commands from stdin. */
typedef unsigned int u32; typedef unsigned long long u64; typedef unsigned char u8;
static int sys3(int n, int a, int b, int c) { int r; __asm__ volatile("int $0x80" : "=a"(r) : "a"(n), "b"(a), "c"(b), "d"(c) : "memory"); return r; }
static int sys6(int n, int a, int b, int c, int d, int e, int f) {
    int r;
    int args[6] = {a, b, c, d, e, f};
    __asm__ volatile("push %%ebx\n push %%ebp\n movl 20(%%ebx), %%ebp\n movl 16(%%ebx), %%edi\n movl 12(%%ebx), %%esi\n movl 8(%%ebx), %%edx\n movl 4(%%ebx), %%ecx\n movl 0(%%ebx), %%ebx\n int $0x80\n pop %%ebp\n pop %%ebx"
                     : "=a"(r) : "a"(n), "b"(args) : "memory", "ecx", "edx", "esi", "edi");
    return r;
}
static void xexit(int c) { sys3(1, c, 0, 0); for (;;); }
static int xread(void *b, int n) { int got = 0; while (got < n) { int r = sys3(3, 0, (int)((u8 *)b + got), n - got); if (r <= 0) return got; got += r; } return got; }
static void xwrite(const void *b, int n) { sys3(4, 1, (int)b, n); }
void *memcpy(void *d, const void *s, u32 n) { u8 *a = d; const u8 *b = s; while (n--) *a++ = *b++; return d; }
void *memmove(void *d, const void *s, u32 n) { u8 *a = d; const u8 *b = s; if (a < b) { while (n--) *a++ = *b++; } else { while (n--) a[n] = b[n]; } return d; }
void *memset(void *d, int v, u32 n) { u8 *a = d; while (n--) *a++ = (u8)v; return d; }
int memcmp(const void *x, const void *y, u32 n) { const u8 *a = x, *b = y; for (; n; n--, a++, b++) if (*a != *b) return *a - *b; return 0; }
u32 strlen(const char *s) { u32 n = 0; while (s[n]) n++; return n; }
char *strcpy(char *d, const char *s) { char *r = d; while ((*d++ = *s++)); return r; }
int strcmp(const char *a, const char *b) { while (*a && *a == *b) { a++; b++; } return (u8)*a - (u8)*b; }
static void hexout(u32 v) { char b[11]; int i; b[0]='0'; b[1]='x'; for (i = 0; i < 8; i++) b[2 + i] = "0123456789abcdef"[(v >> (28 - 4 * i)) & 15]; b[10] = ' '; sys3(4, 2, (int)b, 11); }
static void on_segv(int sig, u32 *info, u32 *uc) {
    sys3(4, 2, (int)"ref32 fault: addr eip ", 22); hexout(info[3]); hexout(uc[5 + 14]); sys3(4, 2, (int)"\n", 1);
    xexit(139);
}
static void restorer(void) { __asm__ volatile("movl $173, %eax\n int $0x80"); }
typedef u64 (*fn10)(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);
void _start(void) {
    __asm__ volatile("andl $-16, %esp");
    { u32 act[5] = {(u32)on_segv, 0x04000004, (u32)restorer, 0, 0}; sys6(174, 11, (int)act, 0, 8, 0, 0); sys6(174, 7, (int)act, 0, 8, 0, 0); sys6(174, 4, (int)act, 0, 8, 0, 0); }
    for (;;) {
        u8 op;
        if (xread(&op, 1) != 1 || op == 'Q') xexit(0);
        if (op == 'M') { u32 h[2]; xread(h, 8); int r = sys6(192, h[0], h[1], 7, 0x32, -1, 0); xwrite(&r, 4); }
        else if (op == 'W') { u32 h[2]; xread(h, 8); xread((void *)h[0], h[1]); }
        else if (op == 'D') { u32 h[2]; xread(h, 8); xwrite((void *)h[0], h[1]); }
        else if (op == 'C') {
            u32 h[14]; xread(h, 56); /* fn, nargs, 12 args */
            u64 r = ((fn10)h[0])(h[2], h[3], h[4], h[5], h[6], h[7], h[8], h[9], h[10], h[11], h[12], h[13]);
            xwrite(&r, 8);
        }
    }
}
