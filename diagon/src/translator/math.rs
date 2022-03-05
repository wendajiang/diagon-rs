use crate::translator::{Example, OptionDescription, Translator, Widget};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Draw {
    dim_x: usize,
    dim_y: usize,
    center_x: usize,
    center_y: usize,

    content: Vec<Vec<char>>,
}

impl Draw {
    pub fn new(content: Vec<Vec<char>>) -> Self {
        if content.is_empty() {
            Draw::default()
        } else {
            let dim_x = content[0].len();
            let dim_y = 1;
            let center_x = dim_x / 2;
            let center_y = 0;
            Self {
                content,
                dim_x,
                dim_y,
                center_x,
                center_y,
            }
        }
    }

    // todo refactor
    pub fn append(&mut self, other: &Draw, x: usize, y: usize) {
        self.resize(
            self.dim_x.max(self.dim_x + other.dim_x),
            self.dim_y.max(self.dim_y + other.dim_y),
        );

        for dy in 0..other.dim_y {
            for dx in 0..other.dim_x {
                self.content[y + dy][x + dx] = other.content[y][x];
            }
        }
    }

    pub fn resize(&mut self, dim_x: usize, dim_y: usize) {
        self.dim_x = dim_x;
        self.dim_y = dim_y;
    }
}

pub struct Math;

impl Translator for Math {
    fn identifier() -> String {
        "Math".to_string()
    }
    fn name() -> String {
        "Mathematical expression".to_string()
    }
    fn description() -> String {
        "Math description".to_string()
    }
    fn options() -> HashMap<&'static str, OptionDescription> {
        vec![
            (
                "style",
                OptionDescription {
                    name: "style".to_string(),
                    values: vec!["Unicode", "ASCII", "Latex"],
                    default_value: "Unicode".to_string(),
                    description: "Use the full unicode charset or only ASCII. Or even latex."
                        .to_string(),
                    r#type: Widget::Combobox,
                },
            ),
            (
                "transform_math_letters",
                OptionDescription {
                    name: "transform_math_letters".to_string(),
                    values: vec!["false", "true"],
                    default_value: "true".to_string(),
                    description: "Transform letter name into their unicode glyph. alpha -> α."
                        .to_string(),
                    r#type: Widget::CheckBox,
                },
            ),
        ]
        .into_iter()
        .collect()
    }
    fn examples() -> Vec<Example> {
        vec![
            ("1-fraction", "f(x) = 1 + x / (1 + x)").into(),
            ("2-square-root", "sqrt(1+sqrt(1+x/2))").into(),
            ("3-power", "f(x) = 1 + x^2 + x^3 + x^(1+1/2)").into(),
            ("4-subscript", "S_n = u_1 + u_2 + ... + u_n").into(),
            ("5-summation", "sum(i^2,i=0,n) = n^3/2+n^2/2+n/6").into(),
            ("6-integral", "int(x^2 * dx ,0,1) = n^3/3").into(),
            (
                "7-product",
                "mult(i^2,i=1,n) = (mult(i,i=1,n))^2\n\n\n\nmult(1/2,1,100) = \n\
7.8886091e-31",
            )
                .into(),
            ("8-vector", "[a;b] + [c;d] = [a+c; b+d]").into(),
            ("9-matrix", "[1,2;3,4] * [x;y] = [1*x+2*y; 3*x+4*y]").into(),
            ("10-factorial", "[n;k] = n! / (k! *(n-k)!)").into(),
            (
                "11-quoted-string",
                "\"x_n\"\n\
x_n\n",
            )
                .into(),
            (
                "12-braces-vs-parenthesis",
                "A_(1+2)\n\
\n\
A_{1+2}\n\
\n\
A^{1+2}\n",
            )
                .into(),
            (
                "13-Math-symbols",
                "Alpha + alpha + Digamma + digamma + Kappa + kappa + Omicron \n\
omicron + Upsilon + upsilon + Beta + beta + Zeta + zeta + Lambda \n\
lambda + Pi + pi + Phi + phi + Gamma + gamma + Eta + eta + Mu + mu \n\
Rho + rho + Chi + chi + Delta + delta + Theta + theta + Nu + nu \n\
Sigma + sigma + Psi + psi + Epsilon + epsilon + Iota + iota + Xi\n\
xi + Tau + tau + Omega + omega",
            )
                .into(),
            (
                "100-continued-fraction",
                "psi = 1 + 1/(1+1/(1+1/(1+1/(1+...))))",
            )
                .into(),
        ]
        .into_iter()
        .collect()
    }

    fn translate(_input: &str, _options: &str) -> String {
        todo!()
    }
}
