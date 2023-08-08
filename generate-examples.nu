#!/usr/bin/env nu

[
    'title "Hello, World"'

    'bookmarks 12 15 188 1234'

    'author "Alex Monad" email="alex@example.com" active=true'

    'contents {
  section "First section" {
    paragraph "This is the first paragraph"
    paragraph "This is the second paragraph"
  }
}'

    'node1; node2; node3;'


    'node "this\nhas\tescapes"
other r"C:\Users\zkat\"'

    'string "my
multiline
value"'

    'other-raw r#"hello"world"#'

    'num 1.234e-42'

    'my-hex 0xdeadbeef
my-octal 0o755
my-binary 0b10101101'

    'bignum 1_000_000'

    '// C style

/*
C style multiline
*/

tag /*foo=true*/ bar=false

/*/*
hello
*/*/'

    '// This entire node and its children are all commented out.
/-mynode "foo" key=1 {
  a
  b
  c
}

mynode /-"commented" "not commented" /-key="value" /-{
  a
  b
}'

'numbers (u8)10 (i32)20 myfloat=(f32)1.5 {
  strings (uuid)"123e4567-e89b-12d3-a456-426614174000" (date)"2021-02-03" filter=(regex)r"$\d+"
  (author)person name="Alex"
}'
] | each {|example|
    let result = $example | from kdl | table --expand
    [
        "```nushell"
        ("> '" ++ $example ++ "' | from kdl")
        "```"
        "```"
        $result
        "```"
    ] | str join "\n"
} | to text | ansi strip | save --force examples.md
