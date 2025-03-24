# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/tui-bar-graph-solid-plasma.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Hide
Type@0 "cargo run --quiet -p tui-bar-graph --example tui-bar-graph solid plasma"
Enter
Sleep 2s
Show
Screenshot "target/tui-bar-graph-solid-plasma.png"
Sleep 1s
Hide
Escape