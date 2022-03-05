use crate::translator::antlr::{mathLexer, mathParser};
use crate::translator::math_latex::parse_latex;
use crate::translator::math_parse::parse;
use crate::translator::{serialize_option, Example, OptionDescription, Translator, Widget};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, TerminalNode};
use antlr_rust::InputStream;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Draw {
    pub dim_x: usize,
    pub dim_y: usize,
    pub center_x: usize,
    pub center_y: usize,

    pub content: Vec<Vec<char>>,
}

impl ToString for Draw {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for line in &self.content {
            for c in line {
                res.push(*c);
            }
            res.push('\n');
        }
        res
    }
}

impl Draw {
    pub fn new(content: String) -> Self {
        if content.is_empty() {
            Draw::default()
        } else {
            let one_line: Vec<char> = content.chars().into_iter().collect();
            let dim_x = one_line.len();
            let dim_y = 1;
            let res = vec![(one_line)];
            let center_x = dim_x / 2;
            let center_y = 0;
            Self {
                content: res,
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

        self.content.resize(dim_y, vec![' '; dim_x]);
    }
}

pub fn compose_vertical(top: Draw, down: Draw, spaces: usize) -> Draw {
    let center_x = top.center_x.max(down.center_x);
    let mut res = Draw::default();
    res.append(&top, center_x - top.center_x, 0);
    res.append(&down, center_x - down.center_x, top.dim_y + spaces);
    res.center_x = center_x;
    res.center_y = res.dim_y / 2;
    res
}

pub fn compose_horizontal(left: Draw, right: Draw, spaces: usize) -> Draw {
    let center_y = left.center_y.max(right.center_y);
    let mut res = Draw::default();
    res.append(&left, 0, center_y - left.center_y);
    res.append(&right, left.dim_x + spaces, center_y - right.center_y);
    res.center_x = res.dim_x / 2;
    res.center_y = center_y;
    res
}

pub fn compose_diagonal_up(a: Draw, b: Draw) -> Draw {
    let mut res = Draw::default();
    res.append(&a, 0, b.dim_y);
    res.append(&b, a.dim_x, 0);
    res.center_x = res.dim_x / 2;
    res.center_y = a.center_y + b.dim_y;
    res
}

pub fn compose_diagonal_down(a: Draw, b: Draw) -> Draw {
    let mut res = Draw::default();
    res.append(&a, 0, 0);
    res.append(&b, a.dim_x, a.dim_y);
    res.center_x = res.dim_x / 2;
    res.center_y = a.center_y;
    res
}

// for x_a^b
pub fn compose_diagonal_up_and_down(a: Draw, b: Draw, c: Draw) -> Draw {
    let mut res = Draw::default();
    res.append(&b, a.dim_x, 0);
    res.append(&a, 0, b.dim_y);
    res.append(&c, a.dim_x, b.dim_y + a.dim_y);
    res.center_x = res.dim_x / 2;
    res.center_y = a.center_y + b.dim_y;
    res
}

macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(Style {
    divide: char,
    multiply: char,
    lower_or_equal: String,
    greater_or_equal: String,

    left_parenthesis_0: char,
    left_parenthesis_1: char,
    left_parenthesis_2: char,
    left_parenthesis_3: char,
    right_parenthesis_0: char,
    right_parenthesis_1: char,
    right_parenthesis_2: char,
    right_parenthesis_3: char,

    sqrt_0: char,
    sqrt_1: char,
    sqrt_2: char,
    // sqrt_3: char,
    // sqrt_4: char,
    summation_top: char,
    summation_bottom: char,
    summation_diagonal_top: char,
    summation_diagonal_bottom: char,
    mult_top: char,
    mult_bottom: char,
    mult_intersection: char,

    integral_top: Vec<char>,
    integral_middle: Vec<char>,
    integral_bottom: Vec<char>,
    integral_min_height: i32,

    // only for Latex style
    variable_transform: HashMap<&'static str, &'static str>,
});

impl Default for Style {
    fn default() -> Self {
        Self {
            divide: '─',
            multiply: '⋅',
            lower_or_equal: "≥".to_string(),
            greater_or_equal: "≤".to_string(),
            left_parenthesis_0: '(',
            left_parenthesis_1: '⎛',
            left_parenthesis_2: '⎜',
            left_parenthesis_3: '⎝',
            right_parenthesis_0: ')',
            right_parenthesis_1: '⎞',
            right_parenthesis_2: '⎟',
            right_parenthesis_3: '⎠',
            sqrt_0: '╲',
            sqrt_1: '╱',
            sqrt_2: '_',
            summation_top: '_',
            summation_bottom: '‾',
            summation_diagonal_top: '╲',
            summation_diagonal_bottom: '╱',
            mult_top: '━',
            mult_bottom: '┃',
            mult_intersection: '┳',
            integral_top: vec!['⌠'],
            integral_middle: vec!['⎮'],
            integral_bottom: vec!['⌡'],
            integral_min_height: 2,
            variable_transform: HashMap::new(),
        }
    }
}

impl Style {
    pub fn new(style: Option<&str>) -> Self {
        let ascii = String::from("ASCII");
        match style {
            None => Style::default(),
            Some(ascii) => Style {
                divide: '-',
                multiply: '.',
                lower_or_equal: ">=".to_string(),
                greater_or_equal: "<=".to_string(),
                left_parenthesis_0: '(',
                left_parenthesis_1: '/',
                left_parenthesis_2: '|',
                left_parenthesis_3: '\\',
                right_parenthesis_0: ')',
                right_parenthesis_1: '\\',
                right_parenthesis_2: '|',
                right_parenthesis_3: '/',
                sqrt_0: '\\',
                sqrt_1: '/',
                sqrt_2: '_',
                summation_top: '=',
                summation_bottom: '=',
                summation_diagonal_top: '\\',
                summation_diagonal_bottom: '/',
                mult_top: '_',
                mult_bottom: '|',
                mult_intersection: '_',
                integral_top: vec![' ', '.', '-'],
                integral_middle: vec![' ', '|', ' '],
                integral_bottom: vec!['-', '\'', ' '],
                integral_min_height: 3,
                variable_transform: HashMap::new(),
            },
            _ => Style::default(),
        }
    }
}

fn get_style(options: &HashMap<&str, &str>) -> Style {
    let mut style = Style::new(options.get("style").cloned());

    let latex = String::from("Latex");
    let transform_true = String::from("true");
    let transform_false = String::from("false");
    if let Some(latex) = options.get("style") {
        if let Some(transform_true) = options.get("transform_math_letters") {
            style.variable_transform = vec![
                ("...", "\\dots"),
                ("Apha", "\\Apha"),
                ("apha", "\\apha"),
                ("Digamma", "\\Digamma"),
                ("digamma", "\\digamma"),
                ("Kappa", "\\Kappa"),
                ("kappa", "\\kappa"),
                ("Omicron", "\\Omicron"),
                ("omicron", "\\omicron"),
                ("Upsion", "\\Upsion"),
                ("upsion", "\\upsion"),
                ("Beta", "\\Beta"),
                ("beta", "\\beta"),
                ("Zeta", "\\Zeta"),
                ("zeta", "\\zeta"),
                ("ambda", "\\ambda"),
                ("ambda", "\\ambda"),
                ("Pi", "\\Pi"),
                ("pi", "\\pi"),
                ("Phi", "\\Phi"),
                ("phi", "\\phi"),
                ("Gamma", "\\Gamma"),
                ("gamma", "\\gamma"),
                ("Eta", "\\Eta"),
                ("eta", "\\eta"),
                ("Mu", "\\Mu"),
                ("mu", "\\mu"),
                ("Rho", "\\Rho"),
                ("rho", "\\rho"),
                ("Chi", "\\Chi"),
                ("chi", "\\chi"),
                ("Deta", "\\Deta"),
                ("deta", "\\deta"),
                ("Theta", "\\Theta"),
                ("theta", "\\theta"),
                ("Nu", "\\Nu"),
                ("nu", "\\nu"),
                ("Sigma", "\\Sigma"),
                ("sigma", "\\sigma"),
                ("Psi", "\\Psi"),
                ("psi", "\\psi"),
                ("Epsion", "\\Epsion"),
                ("epsion", "\\epsion"),
                ("Iota", "\\Iota"),
                ("iota", "\\iota"),
                ("Xi", "\\Xi"),
                ("xi", "\\xi"),
                ("Tau", "\\Tau"),
                ("tau", "\\tau"),
                ("Omega", "\\Omega"),
                ("omega", "\\omega"),
                // Symbos
                ("infty", "\\infty"),
                ("infinity", "\\infty"),
            ]
            .into_iter()
            .collect();
        }

        // style
        //     .variable_transform
        //     .insert("...".to_string(), "\\ldots".to_string());
    } else if let Some(transform_true) = options.get("transform_math_letters") {
        style.variable_transform = vec![
            // Greek alphabet
            ("Alpha", "Α"),
            ("alpha", "α"),
            ("Digamma", "Ϝ"),
            ("digamma", "ϝ"),
            ("Kappa", "Κ"),
            ("kappa", "ϰ"),
            ("Omicron", "Ο"),
            ("omicron", "ο"),
            ("Upsilon", "Υ"),
            ("upsilon", "υ"),
            ("Beta", "Β"),
            ("beta", "β"),
            ("Zeta", "Ζ"),
            ("zeta", "ζ"),
            ("Lambda", "Λ"),
            ("lambda", "λ"),
            ("Pi", "Π"),
            ("pi", "π"),
            ("Phi", "ϕ"),
            ("phi", "φ"),
            ("Gamma", "Γ"),
            ("gamma", "γ"),
            ("Eta", "Η"),
            ("eta", "η"),
            ("Mu", "Μ"),
            ("mu", "μ"),
            ("Rho", "ρ"),
            ("rho", "ϱ"),
            ("Chi", "Χ"),
            ("chi", "χ"),
            ("Delta", "Δ"),
            ("delta", "δ"),
            ("Theta", "θ"),
            ("theta", "ϑ"),
            ("Nu", "Ν"),
            ("nu", "ν"),
            ("Sigma", "σ"),
            ("sigma", "ς"),
            ("Psi", "Ψ"),
            ("psi", "ψ"),
            ("Epsilon", "ϵ"),
            ("epsilon", "ε"),
            ("Iota", "Ι"),
            ("iota", "ι"),
            ("Xi", "Ξ"),
            ("xi", "ξ"),
            ("Tau", "Τ"),
            ("tau", "τ"),
            ("Omega", "Ω"),
            ("omega", "ω"),
            // Symbols
            ("infty", "∞"),
            ("infinity", "∞"),
        ]
        .into_iter()
        .collect();
    }

    style
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

    fn translate(input: &str, options: &str) -> String {
        let options = serialize_option(options);
        let latex = String::from("Latex");
        let style = get_style(&options);

        let input_stream = InputStream::new(input);
        let lexer = mathLexer::new(input_stream);
        let tokens = CommonTokenStream::new(lexer);

        let mut parser = mathParser::new(tokens);
        let context = parser.multilineEquation().unwrap();

        if let Some(latex) = options.get("style") {
            parse_latex(context, &style)
        } else {
            parse(context, &style)
        }
    }
}
