[impeccable.b -- compute impeccable numbers
(c) 2016 Daniel B. Cristofani
http://brainfuck.org/]

[ This is Daniel's original code

>>>->+[
    [
        [<<+>+>-]++++++[<<++++++++>>-]<<-.[-]<
    ]++++++++++.[-]>>>++<[
        [-[[>>>]<<<-[+>>>]<<<[<<<]+>]<-[+>++++++++++>>]>]>>>
        [[>+<-]>>>]<<[-[<++>-[<++>-[<++>-[<++>-[<[-]>-[<++>-]>>[<+<]>[->]<++<<]]]]]<+<<]
	>
    ]>[>>>]<<<
]

This program outputs sequence (http://oeis.org/A014221). Although this sequence
is technically nonterminating, computing its eighth term would require more
storage than can exist in this universe, so you may as well kill this program
after the seventh term.

]

Print out the zero that's supposed to start the sequence
++++++[->++++++++<]>.[----<+>]<--.

Only print six more to avoid the end of the universe
[-]++++++[->>
    >>>++<[
        [-[[>>>]<<<-[+>>>]<<<[<<<]+>]<-[+>++++++++++>>]>]>>>
        [[>+<-]>>>]<<[-[<++>-[<++>-[<++>-[<++>-[<[-]>-[<++>-]>>[<+<]>[->]<++<<]]]]]<+<<]
        >
    ]>[>>>]<<<

    Print the number we've found
    [
        [<<+>+>-]++++++[<<++++++++>>-]<<-.[-]<
    ]++++++++++.[-]

<<]

>[-]<[-]++++++[->+++++++++++<]>+.-[---<+++++>]<+.--.+++.+++++.-.>+++++[<
---->-]<+.->+++++[<++++>-]<.-----------.++++++.-.--[--->+<]>----.+++++[-
<+++>]<.---------.[--->+<]>--.---[-<++++>]<.------------.---.--[--->+<]>
-.+[-<+++>]<++.++++.--.+.++++++++++++.------------.--[--->+<]>--.+++[-<+
++>]<.++++.+++.-----------.--..--.+.++++++++++.-------.--[--->+<]>-.++++
[-<+++>]<++.+++++++.--------.-----------.+++.+++++++++++++.[--->+<]>----
--.[-<+++>]<+.+.+++++++++++++.+++.++.---------------.-.-[--->+<]>-.+[-<+
++>]<+.+>++++[<++++>-]<.>++++[<---->-]<.--[--->+<]>-.---[-<++++>]<.-----
.[--->+<]>-----.---[-<++++>]<.------------.---.--[--->+<]>-.+++[-<+++>]<
.++++..----.+++++.---------.+++++++++.++++++.[---->+<]>+++.+[-<+++>]<++.
+++++++++.----------.-[--->+<]>-.+++++[-<+++>]<.---------.[--->+<]>--.--
-[-<++++>]<.------------.---.--[--->+<]>-.---[-<++++>]<+.-------.-----.+
++++++++++++.>++++[<---->-]<-.+++++++++++++.+.--------------.-[-->+<]>--
--.[-]++++++++++.[-]<

