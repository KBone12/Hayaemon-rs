#[derive(Debug)]
pub enum ErrorType {
    Memory,
    Driver,
    Format,
    Already,
    No3D,
    Device,
    DirectX,
    Unknown
}

impl ErrorType {
    pub fn from_value(value: i32) -> Vec<ErrorType> {
        let mut result = Vec::new();
        if value & 1 == 1 { result.push(ErrorType::Memory); }
        if value & 3 == 3 { result.push(ErrorType::Driver); }
        if value & 6 == 6 { result.push(ErrorType::Format); }
        if value & 14 == 14 { result.push(ErrorType::Already); }
        if value & 21 == 21 { result.push(ErrorType::No3D); }
        if value & 23 == 23 { result.push(ErrorType::Device); }
        if value & 39 == 39 { result.push(ErrorType::DirectX); }
        if value & -1 == -1 { result.push(ErrorType::Unknown); }
        result
    }
}
