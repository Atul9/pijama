use crate::{test_type, test_type_for_all_logical_binops};
use pijama::ty::{Ty, TyError};
use pijama::LangError;

// Test all logical operators with int arguments
test_type_for_all_logical_binops!(
    wrong_type_placeholder,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    })),
    OPERATOR
);

// Test all logical operators with bool and int arguments
test_type_for_all_logical_binops!(
    mixed_type_placeholder_first_is_bool,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    })),
    OPERATOR
);

// Test all logical operators with int and bool arguments
test_type_for_all_logical_binops!(
    mixed_type_placeholder_second_is_bool,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    })),
    OPERATOR
);

test_type!(
    wrong_type_not,
    Err(LangError::Ty(TyError::Mismatch {
        expected: Ty::Bool,
        found: Ty::Int
    }))
);