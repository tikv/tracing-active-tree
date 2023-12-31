// Copyright 2023 TiKV Project Authors. Licensed under Apache-2.0.

use std::fmt;

use smallvec::{Array, SmallVec};
use tracing::{field::Visit, span::Attributes};

#[cfg(feature = "coarsetime")]
use coarsetime::{Duration, Instant};
#[cfg(not(feature = "coarsetime"))]
use std::time::{Duration, Instant};

fn unify_dur(d: Duration) -> std::time::Duration {
    #[cfg(not(feature = "coarsetime"))]
    return d;
    #[cfg(feature = "coarsetime")]
    return std::time::Duration::from(d);
}

pub struct Data {
    pub start_at: Instant,
    pub fields: SmallVec<[(&'static str, Val); 8]>,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.fields.iter() {
            write!(f, "[{key}={value}] ")?;
        }
        write!(f, "[elapsed={:?}]", unify_dur(self.start_at.elapsed()))?;
        Ok(())
    }
}

impl Data {
    pub fn from_attribute(attr: &Attributes<'_>) -> Self {
        let mut fields = SmallVec::new();
        attr.record(&mut ValColl(&mut fields));
        Self {
            start_at: Instant::now(),
            fields,
        }
    }
}

pub enum Val {
    U64(u64),
    I64(i64),
    F64(f64),
    I128(i128),
    U128(u128),
    Bool(bool),

    String(String),
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::U64(val) => write!(f, "{}", val),
            Val::I64(val) => write!(f, "{}", val),
            Val::F64(val) => write!(f, "{}", val),
            Val::I128(val) => write!(f, "{}", val),
            Val::U128(val) => write!(f, "{}", val),
            Val::Bool(val) => write!(f, "{}", val),
            Val::String(val) => write!(f, "{:?}", val),
        }
    }
}
pub struct ValColl<'a, A: Array<Item = (&'static str, Val)>>(pub &'a mut SmallVec<A>);

impl<'a, A: Array<Item = (&'static str, Val)>> Visit for ValColl<'a, A> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .push((field.name(), Val::String(format!("{value:?}"))))
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0.push((field.name(), Val::F64(value)))
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0.push((field.name(), Val::I64(value)))
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0.push((field.name(), Val::U64(value)));
    }

    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.0.push((field.name(), Val::I128(value)));
    }

    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.0.push((field.name(), Val::U128(value)));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0.push((field.name(), Val::Bool(value)));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0.push((field.name(), Val::String(value.to_owned())));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0
            .push((field.name(), Val::String(format!("ERR:{value}"))));
    }
}
