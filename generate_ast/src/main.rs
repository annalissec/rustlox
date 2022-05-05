use std::fs::File;
use std::io::{self, Write};

fn main() {
    generate_ast("src").ok();
}

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &["error", "token", "object", "rc"],
        &[
            "Assign   : Token name, Rc<Expr> value",
            "Binary   : Rc<Expr> left, Token operator, Rc<Expr> right",
            //"Call     : Rc<Expr> callee, Token paren, Vec<Rc<Expr>> arguments",
            //"Get      : Rc<Expr> object, Token name",
            "Grouping : Rc<Expr> expression",
            "Literal  : Option<Object> value",
            //"Logical  : Rc<Expr> left, Token operator, Rc<Expr> right",
            //"Set      : Rc<Expr> object, Token name, Rc<Expr> value",
            "Unary    : Token operator, Rc<Expr> right",
            "Variable : Token name",
        ],
    )?;

    define_ast(
    output_dir,
    "Stmt",
    &["error", "expr", "token", "rc"],
    &[
        "Block      : Rc<Vec<Rc<Stmt>>> statements",
        // "Break      : Token token",
        "Expression : Rc<Expr> expression",
        // "Function   : Token name, Rc<Vec<Token>> params, Rc<Vec<Rc<Stmt>>> body",
        // "If         : Rc<Expr> condition, Rc<Stmt> then_branch, Option<Rc<Stmt>> else_branch",
        "Print      : Rc<Expr> expression",
        // "Return     : Token keyword, Option<Rc<Expr>> value",
        "Var        : Token name, Option<Rc<Expr>> initializer",
        // "While      : Rc<Expr> condition, Rc<Stmt> body",
    ],
)?;

    Ok(())
}

fn define_ast(
    output_dir: &str,
    base_name: &str,
    imports: &[&str],
    types: &[&str],
) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    // use crate:: {imports} and std::
    for i in imports {
        if i == &"rc" {
            writeln!(file, "use std::rc::Rc;")?;
        } else {
            writeln!(file, "use crate::{}::*;", i)?;
        }
    }

    // parsing ast
    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name);
        let arg_split = args.split(',');
        let mut fields = Vec::new();
        for arg in arg_split {
            let (t2type, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, t2type));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.trim().to_string(),
            class_name,
            fields,
        });
    }

    /*
    pub enum Expr {
        Binary(Rc<BinaryExpr>),
        Grouping(Rc<GroupingExpr>),
        Literal(Rc<LiteralExpr>),
        Unary(Rc<UnaryExpr>),
    }
    */

    writeln!(file, "\npub enum {base_name} {{")?;
    for t in &tree_types {
        writeln!(file, "    {}(Rc<{}>),", t.base_class_name, t.class_name)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {} {{", base_name)?;
    writeln!(file, "    pub fn accept<T>(&self, {}_visitor: &dyn {base_name}Visitor<T>) -> Result<T, LoxError> {{", base_name.to_lowercase())?;
    writeln!(file, "        match self {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "            {0}::{1}(v) => v.accept({2}_visitor),",
            base_name,
            t.base_class_name,
            //t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;

    /*
    pub trait ExprVisitor<T> {
        fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
    }
    */

    writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;
    for t in &tree_types {
        writeln!(file, "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, LoxError>;", 
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
    );
    }
    writeln!(file, "}}\n");


    /*
    pub struct BinaryExpr {
        pub left: Rc<Expr>,
        pub operator: Token,
        pub right: Rc<Expr>,
    }
    */
    
    for t in &tree_types {
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for f in &t.fields {
            writeln!(file, "    pub {},", f)?;
        }
        writeln!(file, "}}\n")?;
    }

    /*
    impl BinaryExpr {
        pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxResult> {
            visitor.visit_binary_expr(self)
        }
    }
    */

    for t in &tree_types {
        writeln!(file, "impl {} {{", t.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            t.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
    }
    
    Ok(())
}
