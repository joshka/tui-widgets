# A VHS tape. See https://github.com/charmbracelet/vhs
Output "target/popup.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 600
Hide
Type "cargo run --example popup" Enter
Sleep 2s
Show
Screenshot "target/popup.png"
Sleep 1s
Hide
Space
