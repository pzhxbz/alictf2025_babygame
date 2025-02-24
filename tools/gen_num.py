from PIL import Image, ImageDraw, ImageFont

# Create 32x32 images for digits 0-9 with larger font size
font_size = 32  # Larger font size to fill the image
font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", font_size)
print(font)
for digit in range(10):
    # Create a blank image with white background
    image = Image.new("RGBA", (32, 32))
    draw = ImageDraw.Draw(image)

    # Measure text size to center it
    text = str(digit)
    text_width = draw.textlength(text, font=font)
    text_height = font_size
    x = (32 - text_width) // 2
    y = (32 - text_height) // 2

    # Draw the digit
    draw.text((x, y), text, fill="white", font=font)

    # Save the image
    image.save(f"./digit_{digit}.png")
