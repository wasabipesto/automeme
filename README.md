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

## Running

You can build this in rust:

```
# Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone this repo
git clone git@github.com:wasabipesto/automeme.git
cd automeme

# Test or run clippy
cargo test
cargo clippy

# Build the binary
cargo build -r
target/release/automeme

# Or run directly
cargo run -r
```

You can build and run with Docker:

```
# Clone this repo
git clone git@github.com:wasabipesto/automeme.git
cd automeme

# Build the image
docker build -t automeme .

# Run the image
docker run -d \
    -p 8888:8888 \
    --restart unless-stopped \
    --name automeme \
    automeme
```

## Sources

### Images

- `1984.jpg`: [Know Your Meme](templates/afraid-to-ask.json)
- `afraid-to-ask.jpg`: [Meming Wiki](https://en.meming.world/wiki/Afraid_To_Ask_Andy)
- `biggus.jpg`: [Know Your Meme](https://knowyourmeme.com/memes/biggus-dickus)
- `blink.png`: [Redd/r/MemeRestorationit](https://old.reddit.com/r/MemeRestoration/comments/hqygs3/blinking_white_guy_carefully_adjusted_upscale/)
- `cash-money.png`: Frame pulled directly from [this reddit video](https://old.reddit.com/r/GODZILLA/comments/kn4tbt/that_wasnt_very_cash_money_of_you_godzilla/)
- `finally.jpg`: [Know Your Meme](https://knowyourmeme.com/photos/1670182-finally-synthetic-watermelon)
- `pikachu.png`: [Meming Wiki](https://en.meming.world/wiki/Surprised_Pikachu), scaled up 2x
- `shaq-surprised.png` [Know Your Meme](https://knowyourmeme.com/photos/1474527-shaqs-hot-ones-interview)
- `shirt.png`: [KLE Custom Imaging](https://klecustomimaging.com/product/t-shirt/) (better versions desired)
- `spiderman-crew.jpg`: [Meming Wiki](https://en.meming.world/wiki/Me_and_the_Boys)
- `this-is-fine.jpg`: [Meming Wiki](https://en.meming.world/wiki/This_Is_Fine)
- `through-god.png`: Frame pulled directly from video file
- `thomas-bullshit.png`: [Know Your Meme](https://knowyourmeme.com/memes/thomas-had-never-seen-such-bullshit-before)
- `train-bus.jpg`: [Meming Wiki](https://en.meming.world/wiki/Train_Hitting_School_Bus)
- `von-ron-car.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/euoagt/ucantflys_request_vin_diesel_and_ron_weasley/)
- `weatherboy.png`: Frame pulled directly from [this Youtube video](https://youtu.be/py44k46RR_0)

### Fonts

- Anton: [Google Fonts](https://fonts.google.com/specimen/Anton)
- Bebas Neue: [Google Fonts](https://fonts.google.com/specimen/Bebas+Neue)
- Gabarito: [Google Fonts](https://fonts.google.com/specimen/Gabarito)
