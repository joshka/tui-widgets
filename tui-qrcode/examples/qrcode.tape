# A VHS tape. See https://github.com/charmbracelet/vhs
Output "target/card.gif"
Set Theme "Aardvark Blue"
Set Width 600
Set Height 600
Hide
Type "cargo run -p tui-qrcode --example qrcode" Enter
Sleep 2s
Show
Screenshot "target/qrcode.png"
Sleep 1s
Hide
Type "q"
