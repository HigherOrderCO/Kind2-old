use crate::book::name::Ident;
use crate::book::term::{Operator, Term};
use crate::book::Book;
use crate::book::Entry;
use crate::book::Rule;

pub fn to_hvm_term(book: &Book, term: &Term) -> String {
    if let Some(as_string) = term.interpret_as_string() {
        return format!("\"{}\"", as_string);
    }
    match term {
        Term::Typ { .. } => {
            format!("Type")
        }
        Term::Var { orig: _, name } => {
            format!("{}", name)
        }
        Term::Lam {
            orig: _,
            name,
            body,
        } => {
            let body = to_hvm_term(book, body);
            format!("@{} {}", name, body)
        }
        Term::App {
            orig: _,
            func,
            argm,
        } => {
            let func = to_hvm_term(book, func);
            let argm = to_hvm_term(book, argm);
            format!("({} {})", func, argm)
        }
        Term::All {
            orig: _,
            name: _,
            tipo: _,
            body,
        } => {
            let _body = to_hvm_term(book, body);
            format!("0")
        }
        Term::Let {
            orig: _,
            name,
            expr,
            body,
        } => {
            let expr = to_hvm_term(book, expr);
            let body = to_hvm_term(book, body);
            format!("let {} = {}; {}", name, expr, body)
        }
        Term::Ann {
            orig: _,
            expr,
            tipo: _,
        } => {
            let expr = to_hvm_term(book, expr);
            format!("{}", expr)
        }
        Term::Sub {
            orig: _,
            expr,
            name: _,
            indx: _,
            redx: _,
        } => {
            let expr = to_hvm_term(book, expr);
            format!("{}", expr)
        }
        Term::Ctr {
            orig: _,
            name,
            args,
        } => {
            let entr = book.entrs.get(name).unwrap();
            let args = args
                .iter()
                .enumerate()
                .filter(|(i, _x)| !entr.args[*i].eras)
                .map(|x| &**x.1)
                .collect::<Vec<&Term>>();
            format!(
                "({}{})",
                name,
                args.iter()
                    .map(|x| format!(" {}", to_hvm_term(book, x)))
                    .collect::<String>()
            )
        }
        Term::Fun {
            orig: _,
            name,
            args,
        } => {
            let entr = book.entrs.get(name).unwrap();
            let args = args
                .iter()
                .enumerate()
                .filter(|(i, _x)| !entr.args[*i].eras)
                .map(|x| &**x.1)
                .collect::<Vec<&Term>>();
            format!(
                "({}{})",
                name,
                args.iter()
                    .map(|x| format!(" {}", to_hvm_term(book, x)))
                    .collect::<String>()
            )
        }
        Term::Hlp { orig: _ } => {
            format!("0")
        }
        Term::U60 { orig: _ } => {
            format!("0")
        }
        Term::Num { orig: _, numb } => {
            format!("{}", numb)
        }
        Term::Op2 {
            orig: _,
            oper,
            val0,
            val1,
        } => {
            let val0 = to_hvm_term(book, val0);
            let val1 = to_hvm_term(book, val1);
            format!("({} {} {})", oper, val0, val1)
        }
        Term::Hol { orig: _, numb: _ } => {
            format!("_")
        }
        Term::Mat { .. } => {
            panic!("Internal error."); // removed after adjust()
        }
    }
}

pub fn to_hvm_oper(oper: &Operator) -> String {
    match oper {
        Operator::Add => format!("+"),
        Operator::Sub => format!("-"),
        Operator::Mul => format!("*"),
        Operator::Div => format!("/"),
        Operator::Mod => format!("%"),
        Operator::And => format!("&"),
        Operator::Or => format!("|"),
        Operator::Xor => format!("^"),
        Operator::Shl => format!("<<"),
        Operator::Shr => format!(">>"),
        Operator::Ltn => format!("<"),
        Operator::Lte => format!("<="),
        Operator::Eql => format!("=="),
        Operator::Gte => format!(">="),
        Operator::Gtn => format!(">"),
        Operator::Neq => format!("!="),
    }
}

pub fn to_hvm_rule(book: &Book, rule: &Rule) -> String {
    let name = &rule.name;
    let entry = book.entrs.get(name).unwrap();
    let mut pats = vec![];
    for (arg, pat) in entry.args.iter().zip(rule.pats.iter()) {
        if !arg.eras {
            pats.push(" ".to_string());
            pats.push(to_hvm_term(book, pat));
        }
    }
    let body = to_hvm_term(book, &rule.body);
    format!("({}{}) = {}", name, pats.join(""), body)
}

pub fn to_hvm_entry(book: &Book, entry: &Entry) -> String {
    let kind_name = if let Some(kdln) = &entry.kdln {
        Ident(format!("{} #{}", entry.name, kdln))
    } else {
        entry.name.clone()
    };
    let hvm_name = &entry.name;
    if hvm_name.0 == "HVM.log" {
        return "".to_string();
    }
    let mut args = vec![];
    for arg in &entry.args {
        args.push(format!(
            " {}({}: {})",
            if arg.eras { "-" } else { "" },
            arg.name,
            &arg.tipo
        ));
    }
    if entry.rules.len() > 0 {
        let mut rules = vec![];
        for rule in &entry.rules {
            rules.push(format!("\n{}", to_hvm_rule(book, rule)));
        }
        return format!(
            "// {}{} : {}{}\n\n",
            kind_name,
            args.join(""),
            &entry.tipo,
            rules.join("")
        );
    }
    return "".to_string();
}

pub fn to_hvm_book(book: &Book) -> String {
    let mut lines = vec![];
    for name in &book.names {
        lines.push(to_hvm_entry(
            book,
            book.entrs.get(&Ident(name.to_string())).unwrap(),
        ));
    }
    lines.join("")
}
