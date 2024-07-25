# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/big_text.gif"
Set Theme "Aardvark Blue"
Set Width 600
Set Height 800
Hide
Type@0 "cargo run -p tui-big-text --example big_text --quiet"
Enter
Sleep 2s
Show
Screenshot "target/big_text.png"
Sleep 1s
Hide
Escape
