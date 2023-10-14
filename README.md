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

- `afraid-to-ask.jpg`: [Meming Wiki](https://en.meming.world/wiki/Afraid_To_Ask_Andy)
- `biggus.jpg`: [Know Your Meme](https://knowyourmeme.com/memes/biggus-dickus)
- `pikachu.png`: [Meming Wiki](https://en.meming.world/wiki/Surprised_Pikachu), scaled up 2x
- `spiderman-crew.jpg`: [Meming Wiki](https://en.meming.world/wiki/Me_and_the_Boys)
- `through-god.png`: Frame pulled directly from video file
- `thomas-bullshit.png`: [Know Your Meme](https://knowyourmeme.com/memes/thomas-had-never-seen-such-bullshit-before)
- `train-bus.jpg`: [Meming Wiki](https://en.meming.world/wiki/Train_Hitting_School_Bus)
- `weatherboy.png`: Frame pulled directly from [this Youtube video](https://youtu.be/py44k46RR_0)

### Fonts

- Anton: [Google Fonts](https://fonts.google.com/specimen/Anton)
- Bebas Neue: [Google Fonts](https://fonts.google.com/specimen/Bebas+Neue)
- Gabarito: [Google Fonts](https://fonts.google.com/specimen/Gabarito)
