use super::MathStyle;
use crate::translator::antlr::{
    mathParserContextType, AddopContextAttrs, AtomContextAll, AtomContextAttrs, EquationContextAll,
    EquationContextAttrs, ExpressionContextAll, ExpressionContextAttrs, FactorContextAll,
    FactorContextAttrs, FunctionContextAll, FunctionContextAttrs, MatrixContextAll,
    MatrixContextAttrs, MatrixLineContextAttrs, MulopContextAttrs, MultilineEquationContext,
    MultilineEquationContextAttrs, NewlinesContextAll, NewlinesContextAttrs, PowopContextAttrs,
    RelopContextAttrs, TermContextAll, TermContextAttrs, ValueBangContextAll,
    ValueBangContextAttrs, ValueContextAll, ValueContextAttrs, VariableContextAll,
    VariableContextAttrs,
};
use crate::translator::math_parse::{check_func_integral, check_func_mult, check_func_sum};
use antlr_rust::tree::{ParseTree, TerminalNode};
use antlr_rust::TidExt;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::rc::Rc;

pub fn parse_latex(context: Rc<MultilineEquationContext>, style: &MathStyle) -> String {
    let mut res = String::default();
    for (i, equation) in context.equation_all().iter().enumerate() {
        res += &parse_equation(equation.clone(), style);
        if i < context.newlines_all().len() {
            res += &parse_new_line(context.newlines(i).unwrap().clone(), style);
        }
    }

    res
}

fn parse_new_line(context: Rc<NewlinesContextAll>, style: &MathStyle) -> String {
    if context.EOL_all().is_empty() {
        "".to_string()
    } else {
        context
            .EOL_all()
            .iter()
            .skip(1)
            .fold(" \\\\\n".to_string(), |mut accum, _| {
                accum += "\\\\\n";
                accum
            })
    }
}

fn parse_equation(context: Rc<EquationContextAll>, style: &MathStyle) -> String {
    let mut res = parse_expression(context.expression(0).unwrap(), style);

    for i in 1..context.expression_all().len() {
        let op = context.relop(i - 1).unwrap();

        if op.LT().is_some() {
            res += " < ";
        } else if op.GT().is_some() {
            res += " > ";
        } else if op.LE().is_some() {
            res += " \\leq ";
        } else if op.GE().is_some() {
            res += " \\geq ";
        } else if op.EQ().is_some() {
            res += " = ";
        } else {
        }

        res += &parse_expression(context.expression(i).unwrap(), style);
    }

    res
}

fn parse_expression(context: Rc<ExpressionContextAll>, style: &MathStyle) -> String {
    let mut res = parse_term(context.term(0).unwrap(), style);

    for i in 1..context.term_all().len() {
        res += if context.addop(i - 1).unwrap().PLUS().is_some() {
            " + "
        } else {
            " - "
        };
        res += &parse_term(context.term(i).unwrap(), style);
    }
    res
}

fn parse_term(context: Rc<TermContextAll>, style: &MathStyle) -> String {
    let suppress_parenthesis =
        !context.mulop_all().is_empty() && context.mulop(0).unwrap().DIV().is_some();

    let mut res = parse_factor(context.factor(0).unwrap(), style, suppress_parenthesis);
    for i in 1..context.factor_all().len() {
        if context.mulop(i - 1).unwrap().DIV().is_some() {
            res = "\\frac{".to_string()
                + &res
                + "}{"
                + &parse_factor(context.factor(i).unwrap(), style, true)
                + "}";
        } else {
            res += (" \\cdot ".to_string()
                + &parse_factor(context.factor(i).unwrap(), style, false))
                .as_str();
        }
    }

    res
}

fn parse_factor(
    context: Rc<FactorContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> String {
    let suppress_parenthesis = suppress_parenthesis & (context.valueBang_all().len() == 1);

    let mut res = parse_value_bang(context.valueBang(0).unwrap(), style, suppress_parenthesis);

    for i in 1..context.valueBang_all().len() {
        res += if context.powop(i - 1).unwrap().POW().is_some() {
            "^"
        } else {
            "_"
        };
        res += ("{".to_string()
            + &parse_value_bang(context.valueBang(i).unwrap(), style, false)
            + "}")
            .as_str();
    }
    res
}

fn parse_value_bang(
    context: Rc<ValueBangContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> String {
    if let Some(value) = context.value() {
        parse_value(value, style, suppress_parenthesis)
    } else {
        parse_value_bang(context.valueBang().unwrap(), style, suppress_parenthesis) + "!"
    }
}

fn parse_value(
    context: Rc<ValueContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> String {
    let suppress_parenthesis =
        suppress_parenthesis & (context.PLUS().is_some() && context.MINUS().is_some());
    let res = parse_atom(context.atom().unwrap(), style, suppress_parenthesis);
    if context.MINUS().is_some() {
        return "-".to_string() + res.as_str();
    }
    if context.PLUS().is_some() {
        return "+".to_string() + res.as_str();
    }
    res
}

fn parse_atom(
    context: Rc<AtomContextAll>,
    style: &MathStyle,
    suppress_parenthesis: bool,
) -> String {
    if let Some(variable) = context.variable() {
        return parse_variable(variable, style);
    }

    if let Some(expression) = context.expression() {
        let res = parse_expression(expression, style);
        return if suppress_parenthesis || context.RBRACE().is_some() {
            res
        } else {
            wrap_with_parenthesis_latex(res)
        };
    }

    if let Some(function) = context.function() {
        return parse_function(function, style);
    }

    if let Some(matrix) = context.matrix() {
        return parse_matrix(matrix, style);
    }

    if let Some(node) = context.STRING() {
        return parse_string(node);
    }

    "".to_string()
}

fn wrap_with_parenthesis_latex(element: String) -> String {
    "\\left(".to_string() + &element + "\\right)"
}

fn parse_variable(context: Rc<VariableContextAll>, style: &MathStyle) -> String {
    let mut label = context.VARIABLE().unwrap().get_text();
    if style.variable_transform.contains_key(label.as_str()) {
        label = style
            .variable_transform
            .get(label.as_str())
            .unwrap()
            .to_string();
    }
    label
}

fn parse_matrix(context: Rc<MatrixContextAll>, style: &MathStyle) -> String {
    let mut res = "\\begin{pmatrix} ".to_string();
    let mut first_line = true;
    for line in context.matrixLine_all() {
        if !first_line {
            res += " \\\\ ";
        }
        first_line = false;
        let mut first_column = true;
        for content in line.expression_all() {
            if !first_column {
                res += " & ";
            }
            first_column = false;
            res += &parse_expression(content, style);
        }
    }

    res += " \\end{pmatrix}";
    res
}

fn func_sum(context: Rc<FunctionContextAll>, style: &MathStyle) -> String {
    if !check_func_sum(context.clone()) {
        return "(error)".to_string();
    }

    let mut res = "\\sum".to_string();
    if let Some(equation) = context.equation(1) {
        res += ("_{".to_string() + parse_equation(equation, style).as_str() + "}").as_str();
    }

    if let Some(equation) = context.equation(2) {
        res += ("^{".to_string() + parse_equation(equation, style).as_str() + "}").as_str();
    }

    res + " " + parse_equation(context.equation(0).unwrap(), style).as_str()
}

fn func_integral(context: Rc<FunctionContextAll>, style: &MathStyle) -> String {
    if !check_func_integral(context.clone()) {
        return "(error)".to_string();
    }

    let mut res = "\\int".to_string();
    if let Some(equation) = context.equation(1) {
        res += ("_{".to_string() + parse_equation(equation, style).as_str() + "}").as_str();
    }

    if let Some(equation) = context.equation(2) {
        res += ("^{".to_string() + parse_equation(equation, style).as_str() + "}").as_str();
    }

    res + " " + parse_equation(context.equation(0).unwrap(), style).as_str()
}

fn func_mult(context: Rc<FunctionContextAll>, style: &MathStyle) -> String {
    if !check_func_mult(context.clone()) {
        return "(error)".to_string();
    }

    let mut res = "\\prod".to_string();
    if let Some(equation) = context.equation(1) {
        res += ("_{".to_string() + parse_equation(equation, style).as_str() + "}").as_str();
    }

    if let Some(equation) = context.equation(2) {
        res += ("^{".to_string() + parse_equation(equation, style).as_str() + "}").as_str();
    }

    res + " " + parse_equation(context.equation(0).unwrap(), style).as_str()
}

fn func_common_helper(context: Rc<FunctionContextAll>, style: &MathStyle) -> String {
    let mut content = parse_equation(context.equation(0).unwrap(), style);

    for i in 1..context.equation_all().len() {
        content += (",".to_string() + parse_equation(context.equation(i).unwrap(), style).as_str())
            .as_str();
    }

    content
}

fn func_sqrt(context: Rc<FunctionContextAll>, style: &MathStyle) -> String {
    "\\sqrt{".to_string() + func_common_helper(context, style).as_str() + "}"
}

fn func_known(context: Rc<FunctionContextAll>, style: &MathStyle, name: String) -> String {
    let content = func_common_helper(context, style);
    name + wrap_with_parenthesis_latex(content).as_str()
}

fn func_common(context: Rc<FunctionContextAll>, style: &MathStyle) -> String {
    let content = func_common_helper(context.clone(), style);

    parse_variable(context.variable().unwrap(), style)
        + wrap_with_parenthesis_latex(content).as_str()
}

fn parse_function(context: Rc<FunctionContextAll>, style: &MathStyle) -> String {
    static KNOWN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
        vec![
            ("sin", "\\sin"),
            ("cos", "\\cos"),
            ("tan", "\\tan"),
            ("cot", "\\cot"),
            ("arcsin", "\\arcsin"),
            ("arccos", "\\arccos"),
            ("arctan", "\\arctan"),
            ("sinh", "\\sinh"),
            ("cosh", "\\cosh"),
            ("tanh", "\\tanh"),
            ("coth", "\\coth"),
            ("ln", "\\ln"),
            ("log", "\\log"),
            ("exp ", "\\exp "),
            ("max", "\\max"),
            ("min", "\\min"),
            ("ker", "\\ker"),
        ]
        .into_iter()
        .collect::<_>()
    });

    let function_name = context.variable().unwrap().VARIABLE().unwrap().get_text();
    if function_name == *"sqrt" {
        func_sqrt(context, style)
    } else if function_name == *"sum" {
        func_sum(context, style)
    } else if function_name == *"int" {
        func_integral(context, style)
    } else if function_name == *"mult" {
        func_mult(context, style)
    } else if let Some(trans_name) = KNOWN.get(function_name.as_str()) {
        func_known(context, style, trans_name.to_string())
    } else {
        func_common(context, style)
    }
}

fn parse_string(node: Rc<TerminalNode<mathParserContextType>>) -> String {
    node.get_text()
}
