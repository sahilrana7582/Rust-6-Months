
#[derive(Debug)]
pub struct StrSplit<'a> {
    pub remains:Option<&'a str>,
    pub delimiter: &'a str
}

impl <'a> StrSplit<'a> {

    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self{
            remains: Some(haystack),
            delimiter
        }
    } 
}


impl<'a> Iterator for StrSplit<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.remains {
            Some(data) => {
                if let Some(deli_info) = data.find(self.delimiter) {
                    let (res, rem) = data.split_at(deli_info);
                    self.remains = Some(&rem[self.delimiter.len()..]);
                    Some(res)
                }else {
                        self.remains = None;
                     Some(data)
                }
            },
            None => None
        }
    }
}