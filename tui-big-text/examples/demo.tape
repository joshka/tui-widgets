# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/demo.gif"
Set Theme "Aardvark Blue"
Set Width 600
Set Height 800
Hide
Type@0 "cargo run --example demo --quiet"
Enter
Sleep 2s
Show
Screenshot "target/demo.png"
Sleep 1s
