# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/pixel_size.gif"
Set Theme "Aardvark Blue"
Set Width 1020
Set Height 860
Hide
Type@0 "cargo run -p tui-big-text --example pixel_size --quiet"
Enter
Sleep 2s
Show
Screenshot "target/pixel_size.png"
Sleep 1s
Hide
Escape