//! # Sword
//!
//! by *StarvinCulex @2021/10/23*

use std::fmt;
use std::fmt::{Formatter, Write};

use serde::{Deserialize, Serialize};

/// > SWord是Short Word的缩写
///
/// 占用8字节，可存储长度是12的字符串的数据结构。
///
/// 可以高效地进行字符串比较。
///
/// | 字符 | 存储规则 |
/// | :---------------------------- | :------------: |
/// |小写字母'`a`'到'`z`'            |正常存储|
/// |大写字母'`A`'到'`Z`'            |转换为小写字母|
/// |下划线'`_`' 空格'` `' 中划线'`-`'| 存储为下划线 '`_`' |
/// |`0` `1` `2` `3`               | 正常存储。其他数字存储为'`.`' |
/// |其他字符                        | 存储为`'.'`|
/// |第13、14、15个字符               | 都存储为`'.'`|
/// |16及之后的字符                  |不存储。整个串的长度截取为15|
///
///
/// ## Example:
/// ```rust
/// let sword = SWord::new("Hello World!");
/// println!("{}", sword); // hello_world.
/// assert_eq!(sword, SWord::new("hello-world?")); // true
///
/// println!("{}", SWord::new("0123456789ABC")); // 0123......ab.
/// println!("{}", SWord::new("0123456789ABCD")); // 0123......ab..
/// println!("{}", SWord::new("0123456789ABCDE")); // 0123......ab...
/// assert_eq!(
///     SWord::new("0123456789ABCDE"),
///     SWord::new("0123456789ABCDEF")
/// ); // true
///
/// assert!(SWord::new("a") > SWord::new(""));
/// assert!(SWord::new("ab") > SWord::new("aa"));
/// assert!(SWord::new("abc") > SWord::new("ab"));
/// assert!(SWord::new("aa") > SWord::new("a_"));
/// assert!(SWord::new("a_") > SWord::new("a0"));
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Default)]
pub struct SWord {
    bits: u64,
}

macro_rules! sword_bits_set {
    ($bits: expr, $chars: expr, $index: expr) => {
        if $index < $chars.len() {
            $bits |= SWord::char2word($chars[$index]) << (4 + 5 * (11 - $index as u64));
            true
        } else {
            false
        }
    };
}

#[allow(dead_code)]
impl SWord {
    pub const fn new(string: &'static str) -> SWord {
        let chars = string.as_bytes();
        let len = if chars.len() > 15 { 15 } else { chars.len() };
        let mut instance = SWord { bits: len as u64 };

        let _ = sword_bits_set!(instance.bits, chars, 0)
            && sword_bits_set!(instance.bits, chars, 1)
            && sword_bits_set!(instance.bits, chars, 2)
            && sword_bits_set!(instance.bits, chars, 3)
            && sword_bits_set!(instance.bits, chars, 4)
            && sword_bits_set!(instance.bits, chars, 5)
            && sword_bits_set!(instance.bits, chars, 6)
            && sword_bits_set!(instance.bits, chars, 7)
            && sword_bits_set!(instance.bits, chars, 8)
            && sword_bits_set!(instance.bits, chars, 9)
            && sword_bits_set!(instance.bits, chars, 10)
            && sword_bits_set!(instance.bits, chars, 11);
        instance
    }
}

impl From<&str> for SWord {
    fn from(string: &str) -> SWord {
        let chars = string.as_bytes();
        let len = if chars.len() > 15 { 15 } else { chars.len() };
        let mut instance = SWord { bits: len as u64 };

        let len_content = if chars.len() > 12 { 12 } else { chars.len() };
        for i in 0..len_content {
            sword_bits_set!(instance.bits, chars, i);
        }
        instance
    }
}

impl std::fmt::Display for SWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c in *self {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl PartialOrd for SWord {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl Ord for SWord {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bits.cmp(&other.bits)
    }
}

impl std::iter::Iterator for SWord {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let len = self.len();
        if len == 0 {
            None
        } else {
            let c = SWord::word2char((self.bits >> (5 * 11 + 4)) & 0x1F);
            let new_len = (len - 1) as u64;
            let new_data = (self.bits & 0xFFFF_FFFF_FFFF_FFF0) << 5;
            self.bits = new_data | new_len;
            Some(c)
        }
    }
}

impl std::iter::ExactSizeIterator for SWord {
    #[inline]
    fn len(&self) -> usize {
        (self.bits & 0xF) as usize
    }
}

// private
impl SWord {
    #[inline]
    const fn char2word(u: u8) -> u64 {
        let c = u as char;
        match c {
            'a'..'z' | 'z' => c as u64 - 'a' as u64 + 6,
            'A'..'Z' | 'Z' => c as u64 - 'A' as u64 + 6,
            '0' => 1,
            '1' => 2,
            '2' => 3,
            '3' => 4,
            '_' | ' ' | '-' => 5,
            _ => 0,
        }
    }
    #[inline]
    const fn word2char(u: u64) -> char {
        let s = u as u8;
        match s {
            0 => '.',
            1 => '0',
            2 => '1',
            3 => '2',
            4 => '3',
            5 => '_',
            6..32 => (b'a' - 6 + s) as char,
            _ => '!',
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::sword::SWord;

    fn test_eq(s: &'static str, len: usize, e: &'static str) {
        let sword_const = SWord::new(s);
        let sword_runtime = SWord::from(s);

        assert_eq!(sword_const.len(), len);
        assert_eq!(sword_runtime.len(), len);

        assert_eq!(sword_const, sword_runtime);

        assert_eq!(sword_const.to_string(), e);
        assert_eq!(sword_runtime.to_string(), e);
    }

    #[test]
    fn test() {
        test_eq("", 0, "");
        test_eq("abc", 3, "abc");
        test_eq("123?", 4, "123.");
        test_eq("1A2B.Ct", 7, "1a2b.ct");
        test_eq("to be or not to be", 15, "to_be_or_not...");
    }
}
