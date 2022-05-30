use std::intrinsics::{likely, unlikely};

#[macro_export]
macro_rules! if_likely {
    ($cond:expr => $then: block) => {
        if likely($cond) {
            $then
        }
    };
    ($cond:expr => $then: block else $else: block) => {{
        if likely($cond) {
            $then
        } else {
            $else
        }
    }};
    (let $value: pat = $cond:expr => $then: block) => {{
        let __cond__ = $cond;
        if likely(if let $value = &__cond__ { true } else { false }) {
            if let $value = __cond__ {
                $then
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        }
    }};
    (let $value: pat = $cond:expr => $then: block else $else: block) => {{
        let __cond__ = $cond;
        if likely(if let $value = &__cond__ { true } else { false }) {
            if let $value = __cond__ {
                $then
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        } else {
            $else
        }
    }};
}

#[macro_export]
macro_rules! if_unlikely {
    ($cond:expr => $then: block) => {
        if unlikely($cond) {
            $then
        }
    };
    ($cond:expr => $then: block else $else: block) => {{
        if unlikely($cond) {
            $then
        } else {
            $else
        }
    }};
    (let $value: pat = $cond:expr => $then: block) => {{
        let __cond__ = $cond;
        if unlikely(if let $v = &__cond__ { true } else { false }) {
            if let $value = __cond__ {
                $then
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        }
    }};
    (let $value: pat = $cond:expr => $then: block else $else: block) => {{
        let __cond__ = $cond;
        if unlikely(if let $v = &__cond__ { true } else { false }) {
            if let $value = __cond__ {
                $then
            } else {
                unsafe { std::hint::unreachable_unchecked() }
            }
        } else {
            $else
        }
    }};
}
