# A VHS tape. See https://github.com/charmbracelet/vhs
Output "target/paragraph.gif"
Set Theme "Aardvark Blue"
Set Width 800
Set Height 600
Hide
Type "cargo run --example paragraph" Enter
Sleep 4s
Show
Down@100ms 10
Sleep 1s
Up@100ms 10
Sleep 1s
Hide
Type "q"
