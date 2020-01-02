use std::convert::TryFrom;

static M: &[&'static str] = &["", "m", "mm", "mmm"];
static C: &[&'static str] = &["", "c", "cc", "ccc", "cd", "d",  
"dc", "dcc", "dccc", "cm"];
static X: &[&'static str] = &["", "x", "xx", "xxx", "xl", "l",  
"lx", "lxx", "lxxx", "xc"];
static I: &[&'static str] = &["", "i", "ii", "iii", "iv", "v",  
"vi", "vii", "viii", "ix"];

pub struct Roman {
    value: String
}

impl Roman {
    pub const MAX: i32 = 4999;
    pub const MIN: i32 = 1;

    pub fn to_uppercase(&self) -> Roman {
        let value = self.value.to_uppercase();
        Roman{value}
    }
}

impl Into<String> for Roman {
    fn into(self) -> String {
        self.value
    }
}

impl TryFrom<i32> for Roman {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < Roman::MIN || value > Roman::MAX {
            return Err("Valid values are bigger than zero and smaller than 5000!")
        }
    
        let thousands = M[(value/1000) as usize];
        let hundreds = C[((value%1000)/100) as usize];
        let tens = X[((value%100)/10) as usize];
        let ones = I[(value%10) as usize];

        let value = format!("{}{}{}{}", thousands, hundreds, tens, ones);
        Ok(Roman{value})
    }
}
