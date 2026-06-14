
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder:Option<&'a str>,
    delimiter: &'a str
}

impl <'a> StrSplit<'a> {

    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self{
            remainder: Some(haystack),
            delimiter
        }
    } 
}


impl<'a> Iterator for StrSplit<'a> {

    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.remainder.take()?;

        if let Some(idx) = data.find(self.delimiter) {
            let (head, tail) = data.split_at(idx);

            self.remainder =
                Some(&tail[self.delimiter.len()..]);

            Some(head)
        } else {
            Some(data)
        }
    }
}