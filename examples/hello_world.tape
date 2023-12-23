# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/hello_world.gif"
Set Theme "Aardvark Blue"
Set Width 680
Set Height 600
Hide
Type@0 "cargo run --example hello_world --quiet"
Enter
Sleep 3s
Show
Screenshot "target/hello_world.png"
Sleep 2s
