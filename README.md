# Elden Ring Poise Overlay

## Don't use it online
### Last updated 1.12.3 (works as long as it works)

A tool to watch the toughness value (poise / hyperarmor) live

Features:
- Reads the toughness value live from the game process
- Records unique values and displays them as horizontal lines (reset with F5)
- Listens for LMB and F key and displays the timing as horizontal lines &rarr; For timing the input and hyperarmor activation
- Accepts `--pid PID` in case the Elden Ring process is not named "eldenring.exe"

Will probably get flagged by antivirus because it reads memory from another process and listens to global keyboard and mouse input

Cheat Engine most of the time also gets flagged and this does pretty much the same thing

Credit:
- https://www.nexusmods.com/eldenring/mods/48 for AOB patterns and offsets