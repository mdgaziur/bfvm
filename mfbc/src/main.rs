#![feature(auto_traits)]
mod lexer;
mod error;
mod parser;
mod codegen;

fn writefile(contents: &String, name: &String) {
    std::fs::write(name, contents).unwrap();
}

fn main() {
    let mut output_lex = false;
    let mut output_ast = false;
    let mut output_ir = false;
    let mut output_code = false;
    let mut compile = false; // will try to use gcc to compile it to [filename].out

    if std::env::args().collect::<Vec<String>>()[1] == "--ast" {
        output_ast = true;
    }
    else if std::env::args().collect::<Vec<String>>()[1] == "--lex" {
        output_lex = true;
    }
    else if std::env::args().collect::<Vec<String>>()[1] == "--ir" {
        output_ir = true;
    }
    else if std::env::args().collect::<Vec<String>>()[1] == "--code" {
        output_code = true;
    }
    else if std::env::args().collect::<Vec<String>>()[1] == "--compile" {
        compile = true;
    }

    // currently only get the first argument for source
    let source = std::env::args().collect::<Vec<String>>()[2].clone();

    // open the file
    let file = std::fs::read_to_string(&source).unwrap();

    // error context
    let errctx = error::ErrorCtx::new(&file);

    println!("Generating code...");
    let lexical_analysis = lexer::lex(&file);
    if output_lex {
        writefile(&format!("{:#?}", lexical_analysis), &(source + ".lex"));
        return;
    }

    let mut ast = parser::Parser::new(&lexical_analysis, errctx);
    ast.parse();
    if output_ast {
        writefile(&format!("{:#?}", ast.get_module()), &(source + ".ast"));
        return;
    }

    let ir = codegen::codegen(&ast.get_module());
    if output_ir {
        writefile(&format!("{:#?}", ast.get_module()), &(source + ".ir"));
        return;
    }

    let code = bfvm::c::codegen::codegen(&ir);
    if output_code {
        writefile(&code, &(source + ".c"));
        return;
    }

    if compile {
        println!("Compiling...");
        let tempname = (source.clone() + &rand::random::<i32>().to_string()) + ".c";

        // write content to that
        writefile(&code, &tempname);

        // compile
        let proc = std::process::Command::new("/bin/gcc")
            .arg(&tempname)
            .arg(String::from("-o") + &(source + ".out"))
            .output()
            .unwrap();
        let stdout = String::from_utf8(proc.stdout).unwrap();
        let stderr = String::from_utf8(proc.stderr).unwrap();

        println!("Compiler Stdout: {}", stdout);
        println!("Compiler Stderr: {}", stderr);
    }
}
