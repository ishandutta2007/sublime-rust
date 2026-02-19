from PIL import ImageFont
import json

# Define your exact font and size
font = ImageFont.truetype("arial.ttf", 12)
char_widths = {}

# Loop through printable ASCII characters
for i in range(32, 127):
    char = chr(i)
    # getlength() provides the precise float width
    char_widths[char] = font.getlength(char)

print(json.dumps(char_widths, indent=2))
