use aleph_syntax_tree::syntax::AlephTree as at;

fn gen(ast: at, indent: i64) -> String {
    let c_indent = aleph_syntax_tree::comp_indent(indent);
    match ast {
        at::Unit => String::new(),
        at::Break => format!("{}throw(:break)", c_indent),
        at::Continue => format!("{}throw(:continue)", c_indent),
        at::Ellipsis => format!("..."),
        at::Int{value} => format!("{}{}", c_indent, value),
        at::Float{value} => format!("{}{}", c_indent, value),
        at::Bool{value} => format!("{}{}", c_indent, if value == "true" { "true" } else { "false" }),
        at::String{value} => format!("{}{}", c_indent, value),
        at::Ident{value} => format!("{}{}", c_indent, value),
        at::Complex{..} => String::new(),
        at::Bytes{elems} => {
            let bytes = elems.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ");
            format!("{}<<{}>>", c_indent, bytes)
        },
        at::Tuple{elems} => {
            format!("{}{{{}}}", c_indent, aleph_syntax_tree::gen_list_expr_sep(elems, gen, ", "))
        },
        at::Array{elems} => {
            format!("{}[{}]", c_indent, aleph_syntax_tree::gen_list_expr_sep(elems, gen, ", "))
        },
        at::Neg{expr} => format!("{}-{}", c_indent, gen(*expr, 0)),
        at::Not{bool_expr} => format!("{}not {}", c_indent, gen(*bool_expr, 0)),
        at::And{bool_expr1, bool_expr2} => format!("{}{} and {}", c_indent, gen(*bool_expr1, 0), gen(*bool_expr2, 0)),
        at::Or{bool_expr1, bool_expr2} => format!("{}{} or {}", c_indent, gen(*bool_expr1, 0), gen(*bool_expr2, 0)),
        at::Add{number_expr1, number_expr2} => format!("{}{} + {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Sub{number_expr1, number_expr2} => format!("{}{} - {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Mul{number_expr1, number_expr2} => format!("{}{} * {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Div{number_expr1, number_expr2} => format!("{}{} / {}", c_indent, gen(*number_expr1, 0), gen(*number_expr2, 0)),
        at::Eq{expr1, expr2} => format!("{}{} == {}", c_indent, gen(*expr1, 0), gen(*expr2, 0)),
        at::LE{expr1, expr2} => format!("{}{} <= {}", c_indent, gen(*expr1, 0), gen(*expr2, 0)),
        at::In{expr1, expr2} => format!("{}Enum.member?({}, {})", c_indent, gen(*expr1, 0), gen(*expr2, 0)),
        at::If{condition, then, els} => match *els {
            at::Unit => format!("{}case {} do\n{}true -> {}\n{}false -> nil\n{}end",
                c_indent, gen(*condition, 0),
                aleph_syntax_tree::comp_indent(indent+1), gen(*then, 0),
                aleph_syntax_tree::comp_indent(indent+1),
                c_indent),
            _ => format!("{}case {} do\n{}true -> {}\n{}false -> {}\n{}end",
                c_indent, gen(*condition, 0),
                aleph_syntax_tree::comp_indent(indent+1), gen(*then, 0),
                aleph_syntax_tree::comp_indent(indent+1), gen(*els, 0),
                c_indent)
        },
        at::While{condition, loop_expr, ..} => {
            format!("{}Stream.iterate(nil, fn _ -> if {}, do: {{:cont, {}}}, else: :halt end) |> Enum.to_list()",
                c_indent, gen(*condition, 0), gen(*loop_expr, 0))
        },
        at::Let{var, is_pointer: _, value, expr} => match *expr {
            at::Unit => format!("{}{} = {}", c_indent, var, gen(*value, 0)),
            _ => format!("{}{} = {}\n{}", c_indent, var, gen(*value, 0), gen(*expr, indent))
        },
        at::LetRec{name, args, body} => format!("{}def {}({}) do\n{}\n{}end",
            c_indent, name, aleph_syntax_tree::gen_list_expr(args, gen), gen(*body, indent+1), c_indent),
        at::Get{array_name, elem} => format!("{}Enum.at({}, {})", c_indent, array_name, gen(*elem, 0)),
        at::Put{array_name, elem, value, insert} => {
            if insert == "true" {
                format!("{}List.insert_at({}, {}, {})", c_indent, array_name, gen(*elem, 0), gen(*value.clone(), 0))
            } else {
                let elem_str = gen(*elem, 0);
                format!("{}List.replace_at({}, {}, {})", c_indent, array_name, elem_str, gen(*value.clone(), 0))
            }
        },
        at::Remove{array_name, elem, is_value} => {
            if is_value == "true" {
                format!("{}List.delete({}, {})", c_indent, array_name, gen(*elem, 0))
            } else {
                let elem_str = gen(*elem, 0);
                format!("{}List.delete_at({}, {})", c_indent, array_name, elem_str)
            }
        },
        at::Length{var} => format!("{}length({})", c_indent, var),
        at::Match{expr, case_list} => format!("{}case {} do\n{}\n{}end", c_indent, gen(*expr, 0),
            case_list.iter().map(|case| gen(*case.clone(), indent+1)).collect::<Vec<_>>().join("\n"),
            c_indent),
        at::MatchLine{condition, case_expr} => format!("{}{} -> {}", c_indent, gen(*condition, 0), gen(*case_expr, 0)),
        at::Var{var, ..} => format!("{}{}", c_indent, var),
        at::App{object_name, fun, param_list} => {
            let call = if object_name.is_empty() {
                format!("{}({})", gen(*fun, 0), aleph_syntax_tree::gen_list_expr(param_list, gen))
            } else {
                format!("{}.{}({})", object_name, gen(*fun, 0), aleph_syntax_tree::gen_list_expr(param_list, gen))
            };
            format!("{}{}", c_indent, call)
        },
        at::Stmts{expr1, expr2} => format!("{}{}\n{}", c_indent, gen(*expr1, indent), gen(*expr2, indent)),
        at::Iprt{name} => format!("{}import {}", c_indent, name),
        at::Clss{..} => String::new(),
        at::Return{value} => gen(*value, indent),
        at::Comment{value} => format!("{}#{}", c_indent, value),
        at::CommentMulti{value} => format!("{}#{}", c_indent, value),
        at::Assert{condition, message} => format!("{}if !({}), do: raise({})", c_indent, gen(*condition, 0), gen(*message, 0)),

        // nodes ignored when no Elixir equivalent
        _ => String::new(),
    }
}

pub fn generate(ast: at) -> String {
    gen(ast, 0)
}

