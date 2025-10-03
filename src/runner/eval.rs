use std::rc::Rc;

use crate::dd;
use crate::parser::ast::{BuiltInFunction, Expr};
use crate::runner::Runner;

pub fn eval<'a>(expr: &Expr<'a>, ctx: &Runner<'a>) -> Expr<'a> {
    match expr {
        Expr::Number(n) => Expr::Number(*n),
        Expr::Float(f) => Expr::Float(*f),
        Expr::Bool(b) => Expr::Bool(*b),
        Expr::Char(c) => Expr::Char(*c),
        Expr::String(s, t) => Expr::String(s, *t),
        Expr::DynamicString(s) => Expr::DynamicString(s.clone()),
        Expr::List(list) => {
            let elems: Vec<Expr> = list.iter().map(|e| eval(e, ctx)).collect();
            Expr::List(elems)
        }
        Expr::VariableRef { name, .. } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                eval(&var.value, ctx)
            } else {
                /* TODO: Burası Enum initilization olmalıdır amma başqa kod yazılmış Diqqet et. */
                Expr::DynamicString(Rc::new(name.to_string()))
            }
        }

        Expr::StructInit { name, args } => {
            /* TODO: Burası Best Practice Deyil. Random yazılıb */
            /*             let structdef = ctx.structdefs.get(&name.to_string()).unwrap();
             */
            Expr::StructInit {
                name: name.to_string().into(),
                args: args.to_vec(),
            }
        }

        Expr::BinaryOp { left, op, right } => {
            let left_val = eval(left, ctx);
            let right_val = eval(right, ctx);

            match (&left_val, &right_val) {
                (Expr::Number(l), Expr::Number(r)) => match *op {
                    "+" => Expr::Number(l + r),
                    "-" => Expr::Number(l - r),
                    "*" => Expr::Number(l * r),
                    "/" => Expr::Number(l / r),
                    "==" => Expr::Bool(l == r),
                    _ => Expr::Bool(false), // naməlum operator
                },
                (Expr::Time(l), Expr::Time(r)) => match *op {
                    "-" => Expr::Number(l.duration_since(*r).as_millis() as i64),
                    _ => Expr::Bool(false),
                },

                (Expr::Bool(l), Expr::Bool(r)) => match *op {
                    "&&" => Expr::Bool(*l && *r),
                    "||" => Expr::Bool(*l || *r),
                    "==" => Expr::Bool(l == r),
                    _ => Expr::Bool(false),
                },

                _ => Expr::Bool(false),
            }
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Ceil => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::Float(f) => Expr::Float(f.ceil()),
                    Expr::Number(n) => Expr::Float(n as f64),
                    Expr::UnaryOp { op, expr } => {
                        let expr = eval(&*expr, ctx);
                        match expr {
                            Expr::Float(f) => Expr::Float(f.ceil()),
                            Expr::Number(n) => Expr::Float(n as f64),
                            _ => Expr::Float(0.0),
                        }
                    }
                    _ => Expr::Float(0.0),
                }
            }
            BuiltInFunction::Floor => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::Float(f) => Expr::Float(f.floor()),
                    Expr::Number(n) => Expr::Float(n as f64),
                    Expr::UnaryOp { op, expr } => {
                        let expr = eval(&*expr, ctx);
                        match expr {
                            Expr::Float(f) => Expr::Float(f.floor()),
                            Expr::Number(n) => Expr::Float(n as f64),
                            _ => Expr::Float(0.0),
                        }
                    }
                    _ => Expr::Float(0.0),
                }
            }
            BuiltInFunction::Mod => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::Number(n) => {
                        if n < 0 {
                            Expr::Number(-n)
                        } else {
                            Expr::Number(n)
                        }
                    }
                    Expr::Float(f) => {
                        if f < 0.0 {
                            Expr::Float(-f)
                        } else {
                            Expr::Float(f)
                        }
                    }
                    _ => Expr::Number(0),
                }
            }
            BuiltInFunction::ConvertString => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::Number(n) => Expr::DynamicString(Rc::new(n.to_string())),
                    Expr::Float(f) => Expr::DynamicString(Rc::new(f.to_string())),
                    Expr::Bool(b) => Expr::DynamicString(Rc::new(b.to_string())),
                    Expr::Char(c) => Expr::DynamicString(Rc::new(c.to_string())),
                    _ => Expr::DynamicString(Rc::new("".to_string())),
                }
            }
            BuiltInFunction::Number => {
                let arg = eval(&args[0], ctx);
                match arg {
                    /* TODO: Burası buglu ola biler çünki fail olarsa 0 olacaq. Ona göre validatorda mütleq yoxlanılmalıdır. */
                    Expr::DynamicString(s) => Expr::Number(s.parse().unwrap_or(0)),
                    _ => Expr::Number(0),
                }
            }
            BuiltInFunction::StrLower => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::DynamicString(s) => Expr::DynamicString(Rc::new(s.to_lowercase())),
                    Expr::String(s, _) => Expr::DynamicString(Rc::new(s.to_lowercase())),
                    _ => Expr::DynamicString(Rc::new("".to_string())),
                }
            }
            BuiltInFunction::StrUpper => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::DynamicString(s) => Expr::DynamicString(Rc::new(s.to_uppercase())),
                    Expr::String(s, _) => Expr::DynamicString(Rc::new(s.to_uppercase())),
                    _ => Expr::DynamicString(Rc::new("".to_string())),
                }
            }
            BuiltInFunction::Trim => {
                let arg: Expr<'_> = eval(&args[0], ctx);
                match arg {
                    Expr::DynamicString(s) => Expr::DynamicString(Rc::new(s.trim().to_string())),
                    Expr::String(s, _) => Expr::DynamicString(Rc::new(s.trim().to_string())),
                    _ => Expr::DynamicString(Rc::new("".to_string())),
                }
            }
            BuiltInFunction::Len => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::List(list) => Expr::Number(list.len() as i64),
                    Expr::String(s, _) => Expr::Number(s.len() as i64),
                    _ => Expr::Number(0),
                }
            }
            BuiltInFunction::Sum => {
                if args.len() != 1 {
                    let mut sum = 0;
                    for arg in args {
                        let arg = eval(&arg, ctx);
                        match arg {
                            Expr::Number(n) => sum += n,
                            _ => return Expr::Number(0),
                        }
                    }
                    return Expr::Number(sum);
                }
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::List(list) => Expr::Number(list.len() as i64),
                    _ => Expr::Number(0),
                }
            }
            BuiltInFunction::Min => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::List(list) => {
                        let mut min = i64::MAX;
                        for arg in list {
                            let arg = eval(&arg, ctx);
                            match arg {
                                Expr::Number(n) => {
                                    if n < min {
                                        min = n;
                                    }
                                }
                                _ => return Expr::Number(0),
                            }
                        }
                        return Expr::Number(min);
                    }
                    _ => Expr::Number(0),
                }
            }
            BuiltInFunction::Timer => {
                let a = std::time::Instant::now();
                return Expr::Time(a);
            }

            BuiltInFunction::Max => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::List(list) => {
                        let mut max = i64::MIN;
                        for arg in list {
                            let arg = eval(&arg, ctx);
                            match arg {
                                Expr::Number(n) => {
                                    if n > max {
                                        max = n;
                                    }
                                }
                                _ => return Expr::Number(0),
                            }
                        }
                        return Expr::Number(max);
                    }
                    _ => Expr::Number(0),
                }
            }
            BuiltInFunction::Round => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::Number(n) => Expr::Number(n),
                    Expr::Float(f) => Expr::Number(f.round() as i64),
                    _ => Expr::Number(0),
                }
            }
            BuiltInFunction::Sqrt => {
                let arg = eval(&args[0], ctx);
                match arg {
                    Expr::Number(n) => Expr::Number(n.isqrt() as i64),
                    Expr::Float(f) => Expr::Number(f.sqrt() as i64),
                    _ => Expr::Number(0),
                }
            }

            BuiltInFunction::Range => {
                /* TODO: Burası Xoşuma gelmedi */
                let arg1 = eval(&args[0], ctx);
                let num1;
                let arg2 = eval(&args[1], ctx);
                let num2;
                let mut numbers = Vec::new();
                match arg1 {
                    Expr::Number(n) => num1 = n,
                    _ => return Expr::Number(0),
                }
                match arg2 {
                    Expr::Number(n) => num2 = n,
                    _ => return Expr::Number(0),
                }
                for i in num1..num2 {
                    numbers.push(Expr::Number(i));
                }
                Expr::List(numbers)
            }

            _ => Expr::Number(0),
        },

        other => {
            println!(" Other {:?}", other);
            other.clone()
        }
    }
}
