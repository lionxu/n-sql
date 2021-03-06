// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use std::fmt::{Debug, Error, Formatter};

use super::PredicateExpression;

#[derive(Clone, Debug)]
pub struct NotExpression {
    pub expr: Box<PredicateExpression>,
}

impl NotExpression {
    pub fn new(expr: Box<PredicateExpression>) -> NotExpression {
        NotExpression { expr }
    }
}
