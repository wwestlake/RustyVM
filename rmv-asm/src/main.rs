
#[derive(Debug, Clone)]
enum Token {
    Num(usize),
    Symbol(String),
    Push,
    Add,
    Dump,
    Halt,
    Empty,
}


fn main()  {
    println!("Rusty VM Assembler");

    let test = "\
start:
    push 10     ; this is a comment
    push 20     ; this is also a comment
    add         ; add the two values
    dump        ; dump the VM state
    halt        ; halt the VM
    ";

    let lexer = regex_lexer::LexerBuilder::new()
    .token(r";[\w\s]+\n", |_| None)
    .token(r"[a-z]+:", |l| Some(Token::Symbol(l.to_string())))
    .token(r"[0-9]+", |num| Some(Token::Num(num.parse().unwrap())))
    .token(r"add", |_| Some(Token::Add))
    .token(r"push", |_| Some(Token::Push))
    .token(r"dump", |_| Some(Token::Dump))

    .token(r"halt", |_| Some(Token::Halt))
    .token(r"\s+", |_| None) // skip whitespace
    // ...
    .build();

    match lexer {
        Ok(l) => {
            let tokens = l.tokens(test);
            let mut tokvec: Vec<Token> = vec![];
            for tok in tokens {
                tokvec.push(tok.clone());
            }
            let len = tokvec.len();

            for mut i in 1..len {
                let mut lah = Token::Empty;
                if i + 1 < len {
                    lah = tokvec[i + 1].clone();
                } else {
                    lah = Token::Empty;
                }

                match &tokvec[i] {
                    Token::Push => match lah {
                        Token::Num(n) => {
                            println!("push {}", n);
                            i += 2;
                        },
                        _ => {}
                    },
                    //t => println!("{:#?}", t),
                    _ => {}
                }
            }
        },
        Err(msg) => {
            println!("Error: {}", msg)
        }
    }
}
