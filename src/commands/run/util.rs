use rust_i18n::t;

use crate::util::user_error;

pub fn scripts_field_not_found() {
    user_error(t!("scripts-field-not-found"), exitcode::CONFIG);
}
