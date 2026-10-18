"""Stubs for what the reference build leaves undefined: the game's data tables get the translated program's
addresses (--defsym), so both programs share them; system functions return 0."""
import re, sys
sym_rs, undef_txt, stubs_c, defsyms_txt = sys.argv[1:5]
sym = {}
for l in open(sym_rs):
    m = re.match(r'pub const (\w+): u32 = (0x[0-9A-Fa-f]+);', l)
    if m:
        sym[m.group(1)] = int(m.group(2), 16)
und = sorted({l.strip() for l in open(undef_txt) if l.strip()})
special = {'__udivdi3', '__umoddi3', 'abs', 'atoi', 'strtol', 'sprintf', 'fprintf', 'getenv', 'pow'}
out = ['typedef unsigned long long u64; typedef unsigned int u32;',
       'u64 __udivdi3(u64 n,u64 d){u64 q=0,r=0;int i;if(!d)return 0;for(i=63;i>=0;i--){r=(r<<1)|((n>>i)&1);if(r>=d){r-=d;q|=1ULL<<i;}}return q;}',
       'u64 __umoddi3(u64 n,u64 d){u64 r=0;int i;if(!d)return n;for(i=63;i>=0;i--){r=(r<<1)|((n>>i)&1);if(r>=d)r-=d;}return r;}',
       'int abs(int x){return x<0?-x:x;}',
       'long strtol(const char*s,char**e,int b){long v=0;int neg=0;if(b==0)b=10;while(*s==32)s++;if(*s==45){neg=1;s++;}'
       'for(;;s++){int c=*s,d;if(c>=48&&c<=57)d=c-48;else if(c>=97&&c<=122)d=c-87;else if(c>=65&&c<=90)d=c-55;else break;'
       'if(d>=b)break;v=v*b+d;}if(e)*e=(char*)s;return neg?-v:v;}',
       'int atoi(const char*s){return (int)strtol(s,0,10);}',
       'int sprintf(char*b,const char*f,...){b[0]=0;return 0;}', 'int fprintf(void*f,const char*x,...){return 0;}',
       'char*getenv(const char*n){return 0;}', 'float pow(float a,float b){return 0;}']
ld = []
for u in und:
    if u in special:
        continue
    if u in sym:
        ld.append('--defsym=%s=0x%08X' % (u, sym[u]))
    else:
        out.append('int %s(){return 0;}' % u)
open(stubs_c, 'w').write('\n'.join(out) + '\n')
open(defsyms_txt, 'w').write('\n'.join(ld) + '\n')
print(len(ld), 'shared data symbols,', len(out) - 10, 'stub functions')
