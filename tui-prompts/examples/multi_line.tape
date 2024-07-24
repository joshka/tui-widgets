# charm/vhs script to create a gif of the multi_line example
# run with `vhs multi_line.tape` then `vhs publish multi_line.gif`
# for docs run `vhs new docs.tape`

Output "target/multi_line.gif"

Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600

Hide
Type "cargo -q run --example=multi_line -- --debug"
Enter
Sleep 2s
Show
Sleep 2s
Type "This is the multi-line text example. The text wraps at a character boundary and is constrained to the render area."
Sleep 2s
Hide
Sleep 2s
