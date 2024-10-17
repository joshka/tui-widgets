# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/box_text.gif"
Set Theme "Aardvark Blue"
Set Width 1890
Set Height 420
Hide
Type@0 "cargo run -p tui-box-text --example box_text --quiet"
Enter
Sleep 2s
Show
Screenshot "target/box_text.png"
Sleep 1s
Hide
Escape
