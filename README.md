# 😂 automeme

Automeme generates memes and serves them over HTTP in a human-friendly way. URLs are designed to be easily type-able to predictably generate the desired image, and then fetched by e.g. a chatroom's link preview service.

## Methods

### Default

`/{template_name}`

Generates a template with default settings.

### Full-Text

`/{template_name}/f/{full_text}`

Replaces all text in the template with the given text. Use `|` to move to the next section.

### Sed (unimplemented)

`/{template_name}/s/{old_text}/{new_text}`

Replaces text in the template with the given regular expression.

### Twitter (unimplemented)

`/{template_name}/t/{top_text}`

Adds text above the image, adding whitespace as necessary. Use `|` to move to a new line.

## Sources

### Images

- `pikachu.png`: [Meming Wiki](https://en.meming.world/wiki/Surprised_Pikachu), converted to PNG
- `weatherboy.png`: [This Youtube video](https://youtu.be/py44k46RR_0)

### Fonts

- Anton: [Google Fonts](https://fonts.google.com/specimen/Anton)
- Bebas Neue: [Google Fonts](https://fonts.google.com/specimen/Bebas+Neue)
- Gabarito: [Google Fonts](https://fonts.google.com/specimen/Gabarito)
