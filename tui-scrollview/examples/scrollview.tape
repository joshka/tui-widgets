# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/colors.tape`
Output "target/scrollview.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "cargo run --example=scrollview"
Enter
Sleep 2s
Show
Sleep 2s
Down @0.2s 10
Sleep 2s
PageDown
Sleep 2s
PageDown
Sleep 2s
PageUp
Sleep 2s
PageUp
Sleep 2s
Up @0.2s 10
Sleep 2s
