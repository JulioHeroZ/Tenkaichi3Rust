"""Draws the texts of the options screen's PC page, "Graphics" (game/src/host/options.rs), in the style of the
game's plate texts (make_title_load.py): 16 rows of 512 x 32, first all lit, then all off. Each row is a setting
with one of its values, so changing a value only changes the row the plate shows.

    python3 make_option_labels.py <out folder: game/src/host> [preview.png]
"""
import sys, os
import numpy as np
from PIL import Image
import make_title_load as t

# keep in step with game/src/host/options.rs (ROW_*)
ROWS = ["Graphics"]
ROWS += ["Resolution  ◂ %s ▸" % v for v in ("720p", "1080p", "1440p", "4K")]
ROWS += ["Aspect Ratio  ◂ %s ▸" % v for v in ("4:3", "16:10", "16:9", "21:9")]
ROWS += ["Textures  ◂ %s ▸" % v for v in ("Low", "Medium", "High", "Ultra")]
ROWS += ["Full Screen  \u25c2 %s \u25b8" % v for v in ("Off", "On")]
ROWS += [""]
assert len(ROWS) == 16


# the plates show their text right of the screen's middle, so a long row runs off the screen's edge: the rows
# with a value are drawn this much left of the texture's middle
SHIFT = 72


def row(text, style):
    if not text:
        return np.zeros((t.H, t.W, 4), np.uint8)
    img = t.draw(text, style)
    if "\u25c2" in text:
        img = np.concatenate([img[:, SHIFT:], np.zeros((t.H, SHIFT, 4), np.uint8)], axis=1)
    return img


def main():
    out = sys.argv[1]
    img = np.concatenate([row(r, "on") for r in ROWS] + [row(r, "off") for r in ROWS], axis=0)
    Image.fromarray(img, "RGBA").save(os.path.join(out, "options_labels.png"))
    if len(sys.argv) > 2:
        Image.fromarray(img, "RGBA").save(sys.argv[2])


if __name__ == "__main__":
    main()


# ---- the page's title in the icon window ("Screen", "Sound" in the game): 512 x 64, left-aligned
HEADER = "Graphics"
HEADER_X = 6          # where the game's titles start in their texture (matched by picture)
HEADER_CAP = 27       # the height of a capital
HEADER_TOP = 9


def header():
    from PIL import ImageDraw, ImageFont, ImageFilter
    S = 4
    size = 10
    while True:
        f = ImageFont.truetype(t.FONT, size * S)
        b = f.getbbox("H")
        if b[3] - b[1] >= HEADER_CAP * S:
            break
        size += 0.25
    W, H = 512, 64
    img = Image.new("L", (W * S, H * S), 0)
    d = ImageDraw.Draw(img)
    d.text((HEADER_X * S, HEADER_TOP * S - b[1]), HEADER, font=f, fill=255)
    # the game's titles are a little narrower than the font
    img = img.resize((round(W * 0.92), H), Image.LANCZOS)
    full = Image.new("L", (W, H), 0)
    full.paste(img, (0, 0))
    m = np.asarray(full).astype(np.float32) / 255
    inner = t.dilate(m, t.disc(2.5))
    outer = t.dilate(m, t.disc(5))
    # fill: yellow at the top to red at the bottom of the capitals
    rows = np.zeros((H, 3), np.float32)
    for y in range(H):
        k = min(max((y - HEADER_TOP) / HEADER_CAP, 0), 1)
        top, mid, bot = np.array((255, 236, 120)), np.array((250, 150, 40)), np.array((215, 40, 20))
        rows[y] = top + (mid - top) * (k / 0.5) if k < 0.5 else mid + (bot - mid) * ((k - 0.5) / 0.5)
    fill = np.repeat(rows[:, None, :], W, axis=1)
    dark = np.array((70, 20, 10), np.float32)
    light = np.array((255, 244, 214), np.float32)
    col = light[None, None] * np.ones((H, W, 3), np.float32)
    col = col * (1 - inner[..., None]) + dark * inner[..., None]
    col = col * (1 - m[..., None]) + fill * m[..., None]
    rgba = np.zeros((H, W, 4), np.float32)
    rgba[..., :3] = col
    rgba[..., 3] = outer * 253
    return np.clip(rgba + 0.5, 0, 255).astype(np.uint8)


def write_header(out):
    Image.fromarray(header(), "RGBA").save(os.path.join(out, "options_header.png"))


if __name__ == "__main__" and len(sys.argv) > 1:
    write_header(sys.argv[1])
