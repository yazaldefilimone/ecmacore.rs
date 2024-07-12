use super::ArrayValue;
use super::BigIntValue;
use super::BooleanValue;
use super::NullValue;
use super::NumberValue;
use super::ObjectValue;
use super::StringValue;
use super::SymbolValue;
use super::UndefinedValue;

#[derive(Debug)]
pub enum Value {
  Undefined(UndefinedValue),
  Null(NullValue),
  Boolean(BooleanValue),
  String(StringValue),
  Symbol(SymbolValue),
  Number(NumberValue),
  BigInt(BigIntValue),
  Object(ObjectValue),
  Array(ArrayValue),
}

impl Value {
  pub fn create_undefined_value() -> Self {
    Value::Undefined(UndefinedValue::new())
  }

  pub fn create_null_value() -> Self {
    Value::Null(NullValue::new())
  }

  pub fn create_boolean_value(value: bool) -> Self {
    Value::Boolean(BooleanValue::new(value))
  }

  pub fn create_string_value(value: String) -> Self {
    Value::String(StringValue::new(value))
  }

  pub fn create_symbol_value(value: String) -> Self {
    Value::Symbol(SymbolValue::new(value))
  }

  pub fn create_number_value(value: i64) -> Self {
    Value::Number(NumberValue::new(value))
  }

  pub fn create_bigint_value(value: i128) -> Self {
    Value::BigInt(BigIntValue::new(value))
  }

  pub fn create_object_value() -> Self {
    Value::Object(ObjectValue::new())
  }

  pub fn create_array_value() -> Self {
    Value::Array(ArrayValue::new())
  }
}
