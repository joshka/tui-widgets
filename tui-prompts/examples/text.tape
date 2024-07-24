# charm/vhs script to create a gif of the text example
# run with `vhs text.tape` then `vhs publish text.gif`
# for docs run `vhs new docs.tape`

Output "target/text.gif"

Set Theme "Aardvark Blue"
Set Width 1200
Set Height 1080

Hide
Type "cargo -q run --example=text -- --debug"
Enter
Sleep 0.5s
Show
Sleep 2s
Type "ferris"
Sleep 0.5s
Enter
Sleep 2s
Type "wrong password"
Sleep 0.5s
Escape # simulate incorrect password
Sleep 2s
Ctrl+U # clear the line
Sleep 0.5s
Type "password"
Sleep 0.5s
Enter
Sleep 2s
Type "a secret message"
Sleep 0.5s
Enter
Sleep 0.5s
Hide
Sleep 2s
