"""Writes the font of the port's own screens (save slots, the saving indicator): game/src/host/ui_font.png,
a grid of 16 x N cells of white glyphs on transparent (DejaVu Sans Bold, Bitstream Vera licence), and ui_font.txt:
`cell_w cell_h baseline x_offset y_offset` (where a glyph starts in its cell), then one line per glyph `code advance`.
    python3 make_ui_font.py <out folder> [font.ttf]"""
import sys, os
from PIL import Image, ImageDraw, ImageFont
out = sys.argv[1]
ttf = sys.argv[2] if len(sys.argv) > 2 else "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"
SIZE = 36
chars = [chr(c) for c in range(32, 127)] + list("áàâãéêíóôõúüçÁÀÂÃÉÊÍÓÔÕÚÜÇ…•·©ºª°↑↓←→✕○△□")
font = ImageFont.truetype(ttf, SIZE)
asc, desc = font.getmetrics()
cw, ch = int(max(font.getlength(c) for c in chars)) + 16, asc + desc + 14
cols = 16
rows = (len(chars) + cols - 1) // cols
img = Image.new("RGBA", (cols * cw, rows * ch), (255, 255, 255, 0))
d = ImageDraw.Draw(img)
lines = ["%d %d %d %d %d" % (cw, ch, asc + 7, 6, 6)]
for i, c in enumerate(chars):
    x, y = (i % cols) * cw, (i // cols) * ch
    d.text((x + 6, y + 7), c, font=font, fill=(255, 255, 255, 255))
    lines.append("%d %d" % (ord(c), round(font.getlength(c))))
img.save(os.path.join(out, "ui_font.png"), optimize=True)
open(os.path.join(out, "ui_font.txt"), "w").write("\n".join(lines) + "\n")
print(len(chars), img.size)
