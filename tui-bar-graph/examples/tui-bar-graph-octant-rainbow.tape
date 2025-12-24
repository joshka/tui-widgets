# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/tui-bar-graph-octant-rainbow.gif"
Set Theme "Aardvark Blue"
Set FontSize 20
Set FontFamily "Iosevka"
Set Width 1200
Set Height 600
Hide
Type@0 "cargo run --quiet -p tui-bar-graph --example tui-bar-graph octant rainbow"
Enter
Sleep 2s
Show
Screenshot "target/tui-bar-graph-octant-rainbow.png"
Sleep 1s
Hide
Escape
