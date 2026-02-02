# Mos
```
                ___            ___
               /   \          /   \        
               \_   \        /  __/
                _\   \      /  /__          
                \___  \____/   __/         
                    \_       _/            Modern mode-oriented configurable and
                      | 0 0  \_            extendable terminaled-based text editor
                      |                    
                    _/     /\
                   /o)  (o/\ \_
                   \_____/ /
                     \____/ 
```
- (hopefully going to be a) Modern mode-oriented configurable and extendable terminal-based text editor

## TODO
- [ ] some times when moving the cursor, and backspacing the cursor goes to the end of the line when it shouldnt, it should be before the letters that were pushed up by the backsapce..

## InDev images
<img width="1919" height="1018" alt="image" src="https://github.com/user-attachments/assets/b8df7e28-06f2-4a5e-9c75-69d6ddf3a28e" />
<img width="974" height="552" alt="image" src="https://github.com/user-attachments/assets/e76cb7b1-5552-450e-be04-914244fde3a8" />
<img width="949" height="577" alt="image" src="https://github.com/user-attachments/assets/dd3552fd-8ca6-4611-b403-4b5bcbf1d322" />

## Config (will look something like this)
```toml
[editor]
tab_size = 4
show_gutter = true

[keybindings]
[keybindings.normal]
left = "j|left"
right = "ø|right"
up = "k|up"
down = "l|down"

[keybindings.insert]
left = "left"
right = "right"
up = "up"
down = "down"

[keybindings.command]
left = "left"
right = "right"

```
