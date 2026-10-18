"""Draws the text of the title screen's third menu plate ("Load Game") in the style of the game's own plate texts
(the 512 x 64 textures of the title movie: a gradient fill, a 3-pixel dark outline and a faint shadow), one row of
32 pixels for the lit plate and one for the plate off. game/src/host/title.rs puts it on the plate,
with the colours of the game's palette.

    python3 make_title_load.py <out folder: game/src/host> [text] [preview.png]
"""
import sys, os
import numpy as np
from PIL import Image, ImageDraw, ImageFont, ImageFilter

FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"
W, H = 512, 32
CAP = 18       # the height of a capital, as in the game's texts (rows 6..23 of a row of 32)
TOP = 6
OUTLINE = 3
WIDEN = 1.066  # the game's letters are this much wider than the font's

# the fill of each pixel row of a capital, from the game's lit texts (top to bottom)
LIT = [(243, 234, 135), (242, 229, 133), (241, 228, 132), (238, 216, 127), (237, 208, 123), (235, 200, 120),
       (233, 191, 116), (230, 182, 113), (229, 178, 110), (227, 168, 106), (224, 155, 101), (221, 146, 96),
       (218, 138, 90), (216, 130, 86), (215, 122, 84), (213, 115, 80), (210, 104, 75), (210, 104, 75)]
STYLES = {
    "on": dict(fill=LIT, outline=(44, 21, 27)),
    "off": dict(fill=[(203, 203, 203)] * CAP, outline=(70, 0, 89)),
}


def mask(text):
    """The text's coverage (0..1), 4x oversampled then reduced, its capitals CAP pixels high from row TOP."""
    S = 4
    size = 10
    while True:
        f = ImageFont.truetype(FONT, size * S)
        b = f.getbbox("H")
        if b[3] - b[1] >= CAP * S:
            break
        size += 0.25
    img = Image.new("L", (W * S, H * S), 0)
    d = ImageDraw.Draw(img)
    tw = d.textlength(text, font=f)
    d.text(((W * S - tw) / 2, TOP * S - b[1]), text, font=f, fill=255)
    # the game's letters are a little wider than the font's: stretched about the middle
    sw = round(W * WIDEN)
    img = img.resize((sw, H), Image.LANCZOS)
    out = Image.new("L", (W, H), 0)
    out.paste(img, ((W - sw) // 2, 0))
    return np.asarray(out).astype(np.float32) / 255.0


def disc(r):
    y, x = np.mgrid[-r - 1:r + 2, -r - 1:r + 2]
    d = np.sqrt(x * x + y * y)
    return np.clip(r + 0.9 - d, 0, 1)


def dilate(m, k):
    out = np.zeros_like(m)
    r = k.shape[0] // 2
    p = np.pad(m, r)
    for dy in range(k.shape[0]):
        for dx in range(k.shape[1]):
            if k[dy, dx] > 0:
                out = np.maximum(out, p[dy:dy + m.shape[0], dx:dx + m.shape[1]] * k[dy, dx])
    return out


def draw(text, style):
    st = STYLES[style]
    m = mask(text)
    rows = np.zeros((H, 3), np.float32)
    for y in range(H):
        rows[y] = st["fill"][min(max(y - TOP, 0), CAP - 1)]
    fill = np.repeat(rows[:, None, :], W, axis=1)
    edge = dilate(m, disc(OUTLINE))
    shadow = np.roll(np.roll(edge, 1, 0), 1, 1)
    shadow = np.asarray(Image.fromarray((shadow * 255).astype(np.uint8)).filter(ImageFilter.GaussianBlur(1.2))).astype(np.float32) / 255
    rgba = np.zeros((H, W, 4), np.float32)
    # the shadow, the outline over it, the fill over that
    a_sh = shadow * 0.18
    oc = np.array(st["outline"], np.float32)
    a_ol = edge
    a = a_ol + a_sh * (1 - a_ol)
    col = (oc[None, None] * a_ol[..., None]) / np.maximum(a, 1e-6)[..., None]
    col = fill * m[..., None] + col * (1 - m[..., None])
    rgba[..., :3] = col
    rgba[..., 3] = np.maximum(a, m) * 253
    return np.clip(rgba + 0.5, 0, 255).astype(np.uint8)


def main():
    out = sys.argv[1]
    text = sys.argv[2] if len(sys.argv) > 2 else "Load Game"
    img = np.concatenate([draw(text, "on"), draw(text, "off")], axis=0)
    Image.fromarray(img, "RGBA").save(os.path.join(out, "title_load.png"))
    if len(sys.argv) > 3:
        Image.fromarray(img, "RGBA").save(sys.argv[3])


if __name__ == "__main__":
    main()
