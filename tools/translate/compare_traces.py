import sys
a=[l.rstrip() for l in open(sys.argv[1]) if l.startswith("vb ")]
b=[l.rstrip() for l in open(sys.argv[2]) if l.startswith("vb ")]
print(len(a),"reference lines,",len(b),"translated lines")
for i,(x,y) in enumerate(zip(a,b)):
    if x!=y:
        print("first difference at line",i); print(" ref:",x); print(" new:",y); break
else:
    print("identical over", min(len(a),len(b)), "lines")
