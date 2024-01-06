use crust_grammar::token::Token;

use crate::util::CrustCoreResult;

pub enum Expression {
    Binary {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Grouping {
        expr: Box<Expression>,
    },
    Literal {
        value: Token,
    },
    Unary {
        op: Token,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> CrustCoreResult<T> {
        visitor.visit(self)
    }
}

pub trait Visitor<T> {
    fn visit(&self, expression: &Expression) -> CrustCoreResult<T> {
        match expression {
            Expression::Binary { left, op, right } => self.visit_binary(left, op, right),
            Expression::Grouping { expr } => self.visit_grouping(expr),
            Expression::Literal { value } => self.visit_literal(value),
            Expression::Unary { op, right } => self.visit_unary(op, right),
        }
    }

    fn visit_binary(&self, left: &Expression, op: &Token, right: &Expression)
        -> CrustCoreResult<T>;
    fn visit_grouping(&self, expr: &Expression) -> CrustCoreResult<T>;
    fn visit_literal(&self, value: &Token) -> CrustCoreResult<T>;
    fn visit_unary(&self, op: &Token, right: &Expression) -> CrustCoreResult<T>;
}

pub struct AstPrinter;
impl Visitor<String> for AstPrinter {
    fn visit_binary(
        &self,
        left: &Expression,
        op: &Token,
        right: &Expression,
    ) -> CrustCoreResult<String> {
        let res = format!(
            "( {:?} {} {} )",
            op,
            left.accept(self)?,
            right.accept(self)?
        );
        Ok(res)
    }

    fn visit_grouping(&self, expr: &Expression) -> CrustCoreResult<String> {
        let res = format!("( group {} )", expr.accept(self)?);
        Ok(res)
    }

    fn visit_literal(&self, value: &Token) -> CrustCoreResult<String> {
        let res = match value {
            Token::Identifier(id) => format!("{:?}", id),
            Token::String(id) => format!("{:?}", id),
            Token::Float(id) => format!("{:?}", id),
            Token::Integer(id) => format!("{:?}", id),
            _ => "nil".to_string(),
        };
        Ok(res)
    }

    fn visit_unary(&self, op: &Token, right: &Expression) -> CrustCoreResult<String> {
        let res = format!("( {:?} {} )", op, right.accept(self)?);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_ast() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Unary {
                op: Token::Minus,
                right: Box::new(Expression::Literal {
                    value: Token::Float(2.0),
                }),
            }),
            op: Token::Star,
            right: Box::new(Expression::Grouping {
                expr: Box::new(Expression::Literal {
                    value: Token::Integer(15),
                }),
            }),
        };
        let visitor = AstPrinter {};
        assert_eq!(
            expr.accept(&visitor).unwrap(),
            "( Star ( Minus 2.0 ) ( group 15 ) )"
        );
    }
}
