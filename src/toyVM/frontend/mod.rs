use log::{Level, warn, error, debug, info, trace};

struct binary_stream{
    data: Vec<u8>, cursor: usize, length: usize, name: std::string::String
}

impl binary_stream {
    fn read_un(&mut self, n: usize) -> Result<u64, std::string::String>{
        if self.cursor + n - 1 >= self.length{
            return Err(format!("Out of bound on object {}: request size: {}, index: {}, length: {}", self.name, n, self.cursor, self.length));
        } else if n > 8{
            return Err(format!("Access error on object {}: Request size {} larger than 8(64bit).", self.name, n));
        } else {
            let mut value: u64 = 0;
            for i in 0..n{
                self.cursor += 1;
                value = (value << 8) | (self.data[self.cursor - 1] as u64);
            }
            return Ok(value);
        }
    }

    pub fn read_u1(&mut self) -> Result<u8, std::string::String>{
        let res = self.read_un(1);
        match res {
            Ok(val) => Ok(val as u8),
            Err(val) => Err(val)
        }
    }

    pub fn read_u2(&mut self) -> Result<u8, std::string::String>{
        let res = self.read_un(2);
        match res {
            Ok(val) => Ok(val as u8),
            Err(val) => Err(val)
        }
    }

    pub fn read_u4(&mut self) -> Result<u8, std::string::String>{
        let res = self.read_un(4);
        match res {
            Ok(val) => Ok(val as u8),
            Err(val) => Err(val)
        }
    }

    
}

#[repr(u8)]
enum constant_pool_tags_enums{
    CONSTANT_Utf8 = 1,
    CONSTANT_Integer = 3,
    CONSTANT_Float = 4,
    CONSTANT_Long = 5,
    CONSTANT_Double	= 6,
    CONSTANT_Class = 7,
    CONSTANT_String = 8,
    CONSTANT_Fieldref = 9,
    CONSTANT_Methodref = 10,
    CONSTANT_InterfaceMethodref = 11,
    CONSTANT_NameAndType = 12,
    CONSTANT_MethodHandle = 15,
    CONSTANT_MethodType = 16,
    CONSTANT_Dynamic = 17,
    CONSTANT_InvokeDynamic = 18,
    CONSTANT_Module = 19,
    CONSTANT_Package = 20,
}

struct cp_info{
    tag: constant_pool_tags_enums, info: Vec<u8>
}

struct attribute_info{
    // info should be attribute_length size
    attribute_name_index: u16, attribute_length: u32, info: Vec<u8>
}

struct field_info{
    access_flags: u16, name_index: u16, descriptor_index: u16, 
    attributes_count: u16, attributes: Vec<attribute_info>
}

struct method_info{
    access_flags: u16, name_index: u16, descriptor_index: u16, 
    attributes_count: u16, attributes: Vec<attribute_info>
}

struct classfile{
    magic: u32, minor_version: u16, major_version: u16,
    // Constant pool information
    constant_pool_count: u16, constant_pool: Vec<cp_info>,
    access_flags: u16, this_class: u16, super_class: u16, 
    // interface information
    interface_count: u16, interfaces: Vec<u16>,
    // fields information
    fields_count: u16, fields: Vec<field_info>,
    // methods information
    methods_count: u16, methods: Vec<method_info>,
    // attribute information
    attributes_count: u16, attributes: Vec<attribute_info>
}

impl cp_info{
    pub fn read(&self, stream: &binary_stream){
        
    }
}