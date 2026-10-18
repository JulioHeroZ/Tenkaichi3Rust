"""Checks sampled frames of a translated battle against the reference: the given calls (default Battle_Update)
are probed in one run of the translated game and each is replayed on the reference from the same state."""
import argparse, os, shutil
import descend

ap = argparse.ArgumentParser()
ap.add_argument('--seed', default='0x12345678')
ap.add_argument('--func', default='Battle_Update')
ap.add_argument('ks', help='call numbers, e.g. 1,50,100')
ap.add_argument('--descend', action='store_true', help='descend into the first divergent call')
a = ap.parse_args()
descend.ENV['BT3_SEED'] = a.seed
ks = [int(k) for k in a.ks.split(',')]
for i in range(0, len(ks), descend.BATCH):
    batch = [(a.func, k) for k in ks[i:i + descend.BATCH]]
    d = descend.run_probes(batch)
    for f, k in batch:
        r, counts = descend.check(d, f, k)
        n = descend.report('', f, k, r)
        if n and a.descend:
            keep = d + '_keep'
            shutil.rmtree(keep, ignore_errors=True); shutil.move(d, keep)
            descend.descend(f, k, keep)
            raise SystemExit
    shutil.rmtree(d, ignore_errors=True)
