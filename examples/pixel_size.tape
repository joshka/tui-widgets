# VHS Tape (see https://github.com/charmbracelet/vhs)
Output "target/pixel_size.gif"
Set Theme "Aardvark Blue"
Set Width 1000
Set Height 840
Hide
Type@0 "cargo run --example pixel_size --quiet"
Enter
Sleep 3s
Show
Screenshot "target/pixel_size.png"
Sleep 2s
