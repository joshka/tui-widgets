# A VHS tape. See https://github.com/charmbracelet/vhs
Output "target/scrollbar_mouse.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 600
Set Framerate 50
Hide
Type "cargo run --example scrollbar_mouse --features crossterm"
Enter
Sleep 2s
Show
Right @20ms 100
Down @20ms 100
Left @20ms 100
Up @20ms 100
Hide
Type "q"
Sleep 200ms
