pub struct ByteBuilder{
    vec: Vec<u8>
}

impl ByteBuilder{
    // ***** Public Functions *****
    pub fn with<T: ByteBuilderCompatible>(mut self, obj: T) -> ByteBuilder{
        let bytes_to_add = obj.to_bytes();
        self.vec.extend(bytes_to_add);
        self
    }
    
    pub fn align(self, alignment: usize) -> ByteBuilder{
        let offset = (alignment - (self.vec.len() % alignment)) % alignment;
        let mut aligner: Vec<u8> = Vec::new();
        
        for _ in 0..offset{
            aligner.push(0);
        }
        
        self.with(aligner)
    }
    
    // ***** Private Functions *****
    
    // ***** Init Struct *****
    pub fn from<T: ByteBuilderCompatible>(obj: T) -> ByteBuilder{
        ByteBuilder{
            vec: obj.to_bytes()
        }
    }
}

pub trait ByteBuilderCompatible{
    fn to_bytes(self) -> Vec<u8>;
}

impl ByteBuilderCompatible for ByteBuilder{
    fn to_bytes(self) -> Vec<u8>{
        self.vec
    }
}

impl ByteBuilderCompatible for u32{
    fn to_bytes(self) -> Vec<u8>{
        self.to_ne_bytes().to_vec()
    }
}

impl ByteBuilderCompatible for u16{
    fn to_bytes(self) -> Vec<u8>{
        self.to_ne_bytes().to_vec()
    }
}

impl ByteBuilderCompatible for u8{
    fn to_bytes(self) -> Vec<u8>{
        vec!(self)
    }
}

impl ByteBuilderCompatible for &str{
    fn to_bytes(self) -> Vec<u8>{
        self.as_bytes().to_vec()
    }
}

impl ByteBuilderCompatible for String{
    fn to_bytes(self) -> Vec<u8>{
        self.as_bytes().to_vec()
    }
}

impl<T> ByteBuilderCompatible for Vec<T> where T: ByteBuilderCompatible{
    fn to_bytes(self) -> Vec<u8>{
        let mut vec = Vec::new();
        
        for item in self{
            vec.extend(item.to_bytes());
        }
        
        vec
    }
}