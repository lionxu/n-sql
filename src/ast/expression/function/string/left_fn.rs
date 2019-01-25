// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression as UnBoxExpression;

type Expression = Box<UnBoxExpression>;

#[derive(Clone, Debug)]
pub struct LeftFn {
    pub text: Expression,
    pub length: Expression
}

impl LeftFn {
    pub fn new(text: Expression, length: Expression) -> LeftFn {
        LeftFn {text, length}
    }
}
