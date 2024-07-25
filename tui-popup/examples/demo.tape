# A VHS tape. See https://github.com/charmbracelet/vhs
Output "target/demo.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 600
Hide
Type "cargo run --example demo" Enter
Sleep 2s
Show
Screenshot "target/demo.png"
Sleep 1s
Hide
Space
