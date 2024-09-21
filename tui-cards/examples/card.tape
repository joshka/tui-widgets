# A VHS tape. See https://github.com/charmbracelet/vhs
Output "target/card.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "cargo run -p tui-cards --example card" Enter
Sleep 2s
Show
Screenshot "target/card.png"
Sleep 1s
Hide
Type "q"
