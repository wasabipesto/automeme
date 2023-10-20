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

- `1984.jpg`: [Know Your Meme](https://knowyourmeme.com/memes/living-in-1984)
- `afraid-to-ask.jpg`: [Meming Wiki](https://en.meming.world/wiki/Afraid_To_Ask_Andy)
- `agnes-wink.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `all-the-things.jpeg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `always-has-been.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `ambulance.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `amongus-meeting.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `angry-fan.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `back-up.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `bane-vs-pink.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `bbq-dog.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `big-bullet.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `biggus.jpg`: [Know Your Meme](https://knowyourmeme.com/memes/biggus-dickus)
- `bike-sabotage.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `bill-gates-ping-pong.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `blink.png`: [Redd/r/MemeRestorationit](https://old.reddit.com/r/MemeRestoration/comments/hqygs3/blinking_white_guy_carefully_adjusted_upscale/)
- `bliss.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `bugs-no.jpg`: [Know Your Meme](https://knowyourmeme.com/memes/bugs-bunnys-no)
- `bugs-shoot.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `buzz-everywhere.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `cash-money.png`: [This reddit video](https://old.reddit.com/r/GODZILLA/comments/kn4tbt/that_wasnt_very_cash_money_of_you_godzilla/)
- `cat-wtf.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `change-team.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `club-penguin-banned.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `community-chaos.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `confused-grandma.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `coraline-dad.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `couch-explain.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `daisy-ridley-pointing.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `disappear.jpeg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `disaster-girl.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `disgusted-girl.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `django-fancy.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `domino-effect.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `dragon-heads.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `eggman-button.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `epic-handshake.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `exit-drift.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `fight-dab.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `finally.jpg`: [Know Your Meme](https://knowyourmeme.com/photos/1670182-finally-synthetic-watermelon)
- `floating-boy.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `fry-not-sure.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `god-said.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `gta-go-again.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `honest-work.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `hulk-regrets.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `hulk-tacos.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `i-guess.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `if-i-had-one.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `invincible-look.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `josh-videogames.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `kekw.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `kermit-despair.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `kirk-shock.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `knights-agreement.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `kowalski-point.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `masters-blessing.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `mememan-helth.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `mememan-kemist.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `mememan-mekanik.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `mememan-shef.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `military-protect.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `monkey-nervous.png`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `nemo-mine.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `office-same-picture.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `one-does-not-simply.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `pikachu.png`: [Meming Wiki](https://en.meming.world/wiki/Surprised_Pikachu)
- `political-compass.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `seagull-yell.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `shaq-surprised.png` [Know Your Meme](https://knowyourmeme.com/photos/1474527-shaqs-hot-ones-interview)
- `shirt.png`: [KLE Custom Imaging](https://klecustomimaging.com/product/t-shirt/) (better versions desired)
- `shrek-yell.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `simpsons-toss-barney.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `sleep-brain.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `society-if.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `soda-mix.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `soyjack-point.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `spiderman-crew.jpg`: [Meming Wiki](https://en.meming.world/wiki/Me_and_the_Boys)
- `spiderman-explain.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `spiderman-hey.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `spiderman-learn.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `spongebob-imagination.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `spongebob-window.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `stress-vein.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `taken-skills.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `this-is-fine.jpg`: [Meming Wiki](https://en.meming.world/wiki/This_Is_Fine)
- `this-is-worthless.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `thomas-bullshit.png`: [Know Your Meme](https://knowyourmeme.com/memes/thomas-had-never-seen-such-bullshit-before)
- `through-god.png`: Frame pulled directly from video file
- `trade-offer.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `train-bus.jpg`: [Meming Wiki](https://en.meming.world/wiki/Train_Hitting_School_Bus)
- `trap.jpeg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `trust-nobody.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `virgin-chad.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `volume-up.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `von-ron-car.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/euoagt/ucantflys_request_vin_diesel_and_ron_weasley/)
- `weatherboy.png`: [This Youtube video](https://youtu.be/py44k46RR_0)
- `woman-yelling.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)
- `zac-efron-dunno.jpg`: [/r/MemeRestoration](https://old.reddit.com/r/MemeRestoration/comments/rfjhs0/hd_meme_templates_database_800_files/)

### Fonts

- Anton: [Google Fonts](https://fonts.google.com/specimen/Anton)
- Bebas Neue: [Google Fonts](https://fonts.google.com/specimen/Bebas+Neue)
- Gabarito: [Google Fonts](https://fonts.google.com/specimen/Gabarito)
