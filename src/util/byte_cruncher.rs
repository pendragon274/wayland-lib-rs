pub trait ByteCrunchCompatibleSized{
    fn crunch_out_of(out_of: &Vec<u8>) -> Self where Self: Sized;
    fn size_of() -> usize;
}

pub trait ByteCrunchCompatibleUnsized{
    fn crunch_out_of(out_of: &Vec<u8>, len: usize) -> Self where Self: Sized;
}

pub struct ByteCruncher{
    vec: Vec<u8>
}

impl ByteCruncher {
    // ***** Public Functions *****
    pub fn crunch_sized<T: ByteCrunchCompatibleSized>(&mut self) -> T{
        let size = T::size_of();
        let input = self.vec.drain(0..size).collect();
        T::crunch_out_of(&input)
    }

    pub fn crunch_unsized<T: ByteCrunchCompatibleUnsized>(&mut self, len: usize) -> T{
        let input = self.vec.drain(0..len).collect();
        T::crunch_out_of(&input, len)
    }

    // ***** Private Functions *****

    // ***** Init Struct *****
    pub fn from(my_vec: Vec<u8>) -> ByteCruncher {
        ByteCruncher{
            vec: my_vec
        }
    }
}

impl ByteCrunchCompatibleSized for u32{
    fn crunch_out_of(out_of: &Vec<u8>) -> u32 {
        if out_of.len() < 4 {
            panic!("ByteCruncher given a shorter vector than can be parsed.");
        }

        u32::from_ne_bytes(out_of[0..4].try_into().unwrap())
    }

    fn size_of() -> usize {
        4
    }
}

impl ByteCrunchCompatibleSized for u16{
    fn crunch_out_of(out_of: &Vec<u8>) -> u16 {
        if out_of.len() < 2 {
            panic!("ByteCruncher given a shorter vector than can be parsed.");
        }

        u16::from_ne_bytes(out_of[0..2].try_into().unwrap())
    }

    fn size_of() -> usize {
        2
    }
}

impl ByteCrunchCompatibleSized for u8{
    fn crunch_out_of(out_of: &Vec<u8>) -> u8 {
        if out_of.len() < 1 {
            panic!("ByteCruncher given a shorter vector than can be parsed.");
        }

        u8::from_ne_bytes(out_of[0..1].try_into().unwrap())
    }

    fn size_of() -> usize {
        1
    }
}

impl ByteCrunchCompatibleUnsized for String{
    fn crunch_out_of(out_of: &Vec<u8>, len: usize) -> String {
        String::from_utf8(out_of[0..len].to_vec()).unwrap()
    }
}