// Copyright (c)  YISH. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use ast::Expression;

#[derive(Clone, Debug)]
pub struct HourSubFn {
    pub expr: Box<Expression>,
    pub offset: Box<Expression>
}

impl HourSubFn {
    pub fn new(expr: Box<Expression>, offset: Box<Expression>) -> HourSubFn {
        HourSubFn{expr, offset}
    }
}
