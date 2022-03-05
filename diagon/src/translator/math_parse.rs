use super::MathDraw;
use super::MathStyle;
use crate::translator::antlr::{
    mathLexer, mathParser, mathParserContextType, AddopContextAll, AddopContextAttrs,
    AtomContextAll, AtomContextAttrs, EquationContextAll, EquationContextAttrs,
    ExpressionContextAll, ExpressionContextAttrs, FactorContextAll, FactorContextAttrs,
    FunctionContextAll, FunctionContextAttrs, MulopContextAttrs, MultilineEquationContext,
    MultilineEquationContextAttrs, NewlinesContextAll, NewlinesContextAttrs, PowopContextAll,
    PowopContextAttrs, RelopContextAttrs, TermContextAll, TermContextAttrs, ValueBangContextAll,
    ValueBangContextAttrs, ValueContextAll, ValueContextAttrs, VariableContextAll,
    VariableContextAttrs,
};
use crate::translator::math::{
    compose_diagonal_down, compose_diagonal_up, compose_diagonal_up_and_down, compose_horizontal,
    compose_vertical,
};
use antlr_rust::tree::{ParseTree, TerminalNode};
use std::rc::Rc;

pub fn parse(context: Rc<MultilineEquationContext>, style: &MathStyle) -> String {
    let mut draw = MathDraw::default();
    for (i, equation) in context.equation_all().into_iter().enumerate() {
        draw = compose_vertical(draw, parse_equation(equation, style), 0);
        if i < context.newlines_all().len() {
            draw = compose_vertical(draw, parse_newline(context.newlines(i).unwrap()), 0);
        }
    }
    draw.to_string()
}

fn parse_equation(context: Rc<EquationContextAll>, style: &MathStyle) -> MathDraw {
    let mut draw = parse_expression(context.expression(0).unwrap(), style);
    for i in 1..context.expression_all().len() {
        let op = context.relop(i - 1).unwrap();
        let symbol = {
            if op.LT().is_some() {
                "<".to_string()
            } else if op.GT().is_some() {
                ">".to_string()
            } else if op.LE().is_some() {
                style.lower_or_equal.to_owned()
            } else if op.GE().is_some() {
                style.greater_or_equal.to_owned()
            } else if op.EQ().is_some() {
                "=".to_string()
            } else {
                String::default()
            }
        };

        let op_x = draw.dim_x + 1;
        draw = compose_horizontal(
            draw,
            parse_expression(context.expression(i).unwrap(), style),
            2 + symbol.len(),
        );

        for (i, c) in symbol.chars().enumerate() {
            draw.content[draw.center_y][op_x + i] = c;
        }
    }
    draw.center_x = 0;

    draw
}

fn parse_term(context: Rc<TermContextAll>, style: &MathStyle) -> MathDraw {
    let suppress_parenthesis =
        !context.mulop_all().is_empty() && context.mulop(0).unwrap().DIV().is_some();

    let mut draw = parse_factor(context.factor(0).unwrap(), style, suppress_parenthesis);

    for i in 1..context.factor_all().len() {
        if context.mulop(i - 1).unwrap().DIV().is_some() {
            let opt_y = draw.dim_y;
            draw = compose_vertical(
                draw,
                parse_factor(context.factor(i).unwrap(), style, true),
                1,
            );
            for x in 0..draw.dim_x {
                draw.content[opt_y][x] = style.divide;
            }
            draw.center_y = opt_y;
        } else {
            let opt_x = draw.dim_x + 1;
            draw = compose_horizontal(
                draw,
                parse_factor(context.factor(i).unwrap(), style, false),
                3,
            );
            draw.content[draw.center_y][opt_x] = style.multiply;
        }
    }
    draw
}

fn parse_factor(
    context: Rc<FactorContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> MathDraw {
    let suppress_parenthesis = suppress_parenthesis & (context.valueBang_all().len() == 1);
    let value_bang_size = context.valueBang_all().len();
    if value_bang_size == 0 {
        return MathDraw::default();
    }

    let mut draw = parse_value_bang(
        context.valueBang_all()[0].clone(),
        style,
        suppress_parenthesis,
    );
    if value_bang_size == 3 {
        if context.powop(0).unwrap().POW().is_some()
            && context.powop(1).unwrap().SUBSCRIPT().is_some()
        {
            return compose_diagonal_up_and_down(
                draw,
                parse_value_bang(context.valueBang(1).unwrap(), style, false),
                parse_value_bang(context.valueBang(2).unwrap(), style, false),
            );
        } else if context.powop(1).unwrap().POW().is_some()
            && context.powop(0).unwrap().SUBSCRIPT().is_some()
        {
            return compose_diagonal_up_and_down(
                draw,
                parse_value_bang(context.valueBang(2).unwrap(), style, false),
                parse_value_bang(context.valueBang(1).unwrap(), style, false),
            );
        }
    }

    for i in 1..value_bang_size {
        match context.powop(i - 1) {
            None => {
                draw = compose_diagonal_down(
                    draw,
                    parse_value_bang(context.valueBang(i).unwrap(), style, false),
                );
            }
            Some(_) => {
                draw = compose_diagonal_up(
                    draw,
                    parse_value_bang(context.valueBang(i).unwrap(), style, false),
                );
            }
        }
    }

    draw
}

fn parse_value_bang(
    context: Rc<ValueBangContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> MathDraw {
    if let Some(value) = context.value() {
        parse_value(value, style, suppress_parenthesis)
    } else if let Some(value_bang) = context.valueBang() {
        compose_horizontal(
            parse_value_bang(value_bang, style, suppress_parenthesis),
            MathDraw::new("!".to_string()),
            0,
        )
    } else {
        // not run
        MathDraw::default()
    }
}

fn parse_value(
    context: Rc<ValueContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> MathDraw {
    let suppress_parenthesis =
        suppress_parenthesis & (context.PLUS().is_none() && context.MINUS().is_none());
    if let Some(atom) = context.atom() {
        let draw = parse_atom(atom, style, suppress_parenthesis);
        if context.MINUS().is_some() {
            compose_horizontal(MathDraw::new("-".to_string()), draw, 0)
        } else if context.PLUS().is_some() {
            compose_horizontal(MathDraw::new("+".to_string()), draw, 0)
        } else {
            draw
        }
    } else {
        MathDraw::default()
    }
}

fn parse_newline(context: Rc<NewlinesContextAll>) -> MathDraw {
    let mut res = MathDraw::default();
    res.resize(0, context.EOL_all().len() - 1);
    res
}

fn parse_atom(
    context: Rc<AtomContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> MathDraw {
    if let Some(c) = context.variable() {
        parse_variable(c, style)
    } else if let Some(e) = context.expression() {
        let draw = parse_expression(e, style);
        if suppress_parenthesis || context.RBRACE().is_some() {
            draw
        } else {
            wrap_with_parenthesis(&draw, style)
        }
    } else if let Some(f) = context.function() {
        todo!()
    } else if let Some(m) = context.matrix() {
        todo!()
    } else if let Some(t) = context.STRING() {
        parse_string(t)
    } else {
        MathDraw::default()
    }
}

fn check_func_sqrt(context: Rc<FunctionContextAll>) -> bool {
    match context.equation_all().len() {
        1 => true,
        _ => {
            eprintln!(
                "Square root function (sqrt) only handle one argument, {} provided",
                context.equation_all().len()
            );
            false
        }
    }
}

fn parse_func_sqrt(context: Rc<FunctionContextAll>, style: &MathStyle) -> MathDraw {
    if !check_func_sqrt(context.clone()) {
        MathDraw::new("(error)".to_string())
    } else {
        let a = parse_equation(context.equation(0).unwrap(), style);
        let mut draw = MathDraw::default();
        draw.append(&a, 1 + a.dim_y, 1);
        *draw.content.last_mut().unwrap().first_mut().unwrap() = style.sqrt_0;
        let ll = draw.content.len() - 1;
        for y in 0..ll {
            draw.content[ll - y][1 + y] = style.sqrt_1;
        }

        for x in draw.content.len()..draw.content[0].len() {
            draw.content[0][x] = style.sqrt_2;
        }

        draw.center_x = draw.dim_x / 2;
        draw.center_y = a.center_y + 1;

        draw
    }
}

fn wrap_with_parenthesis(element: &MathDraw, style: &MathStyle) -> MathDraw {
    let mut d = MathDraw::default();
    d.resize(element.dim_x + 2, element.dim_y);
    for line in d.content.iter_mut() {
        *line.first_mut().unwrap() = '|';
        *line.last_mut().unwrap() = '|';
    }
    let ll = d.content.len();
    for i in 0..ll {
        let line = &mut d.content[i];
        let is_first = i == 0;
        let is_last = i == ll - 1;
        if is_first && is_last {
            *line.first_mut().unwrap() = style.left_parenthesis_0;
            *line.last_mut().unwrap() = style.right_parenthesis_0;
            continue;
        }
        if is_first {
            *line.first_mut().unwrap() = style.left_parenthesis_1;
            *line.last_mut().unwrap() = style.right_parenthesis_1;
            continue;
        }
        if is_last {
            *line.first_mut().unwrap() = style.left_parenthesis_3;
            *line.last_mut().unwrap() = style.right_parenthesis_3;
            continue;
        }
        *line.first_mut().unwrap() = style.left_parenthesis_2;
        *line.last_mut().unwrap() = style.right_parenthesis_2;
    }

    d.append(element, 1, 0);
    d.center_x = d.dim_x / 2;
    d.center_y = element.center_y;

    d
}

fn parse_func_sum(context: Rc<FunctionContextAll>, style: &MathStyle) -> MathDraw {
    todo!()
}

fn parse_func_int(context: Rc<FunctionContextAll>, style: &MathStyle) -> MathDraw {
    todo!()
}

fn parse_func_mult(context: Rc<FunctionContextAll>, style: &MathStyle) -> MathDraw {
    todo!()
}

fn parse_func_common(context: Rc<FunctionContextAll>, style: &MathStyle) -> MathDraw {
    let mut draw = parse_equation(context.equation(0).unwrap(), style);
    for i in 1..context.equation_all().len() {
        let x = draw.dim_x;
        draw = compose_horizontal(draw, parse_equation(context.equation(i).unwrap(), style), 2);
        draw.content[draw.center_y][x] = ',';
    }

    compose_horizontal(
        parse_variable(context.variable().unwrap(), style),
        wrap_with_parenthesis(&draw, style),
        if draw.dim_y == 1 { 0 } else { 1 },
    )
}

fn parse_function(context: Rc<FunctionContextAll>, style: &MathStyle) -> MathDraw {
    if let Some(f) = context.variable() {
        if let Some(t) = f.VARIABLE() {
            let func_name = t.get_text();
            if func_name == *"sqrt" {
                parse_func_sqrt(context, style)
            } else if func_name == *"sum" {
                parse_func_sum(context, style)
            } else if func_name == *"int" {
                parse_func_int(context, style)
            } else if func_name == *"mult" {
                parse_func_mult(context, style)
            } else {
                parse_func_common(context, style)
            }
        } else {
            MathDraw::default()
        }
    } else {
        // not run
        MathDraw::default()
    }
}

fn parse_string(node: Rc<TerminalNode<mathParserContextType>>) -> MathDraw {
    let a = node.get_text();
    MathDraw::new((&a[1..a.len() - 2]).to_string())
}

fn parse_expression(context: Rc<ExpressionContextAll>, style: &MathStyle) -> MathDraw {
    let mut draw = parse_term(context.term(0).unwrap(), style);
    for i in 1..context.term_all().len() {
        let op_x = draw.dim_x + 1;
        draw = compose_horizontal(draw, parse_term(context.term(i).unwrap(), style), 3);
        draw.content[draw.center_y][op_x] = match context.addop(i - 1).unwrap().PLUS() {
            None => '+',
            Some(_) => '-',
        };
    }

    draw
}

fn parse_variable(context: Rc<VariableContextAll>, style: &MathStyle) -> MathDraw {
    let key = context.VARIABLE().unwrap().get_text();
    MathDraw::new(
        style
            .variable_transform
            .get(key.as_str())
            .and_then(|v| Option::from(v.to_string()))
            .unwrap_or(key),
    )
}
