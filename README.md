# 😂 automeme

Automeme generates memes and serves them over HTTP in a human-friendly way. URLs are designed to be easily type-able to predictably generate the desired image, and then fetched by e.g. a chatroom's link preview service.

## Methods

### Default

`/{template_name}`

Generates a template with default settings.

### Full-Text

`/{template_name}/f/{full_text}`

Replaces all text in the template with the given text. Use `|` to move to the next section.

### Sed

`/{template_name}/s/{old_text}/{new_text}`

Replaces text in the template with the given regular expression. No pattern matching, just basic replacement.

## Sources

### Images

- `biggus.png`: [Know Your Meme](https://knowyourmeme.com/memes/biggus-dickus), converted to PNG
- `pikachu.png`: [Meming Wiki](https://en.meming.world/wiki/Surprised_Pikachu), converted to PNG
- `spiderman-crew.png`: [Meming Wiki](https://en.meming.world/wiki/Me_and_the_Boys), converted to PNG
- `weatherboy.png`: [This Youtube video](https://youtu.be/py44k46RR_0), scaled up 2x

### Fonts

- Anton: [Google Fonts](https://fonts.google.com/specimen/Anton)
- Bebas Neue: [Google Fonts](https://fonts.google.com/specimen/Bebas+Neue)
- Gabarito: [Google Fonts](https://fonts.google.com/specimen/Gabarito)
