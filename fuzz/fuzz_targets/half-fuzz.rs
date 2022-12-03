#![no_main]
use libfuzzer_sys::fuzz_target;
use half::{bf16, f16};

fuzz_target!(|data: &[u8]| {
    if data.len() > 1 {
        let opt = data[0];
        let mut i = 1;
        match opt {
            0=>{
                while i + 4 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f32;
                    let conv = f16::from_f32(float_in);
                    f16::to_f32(conv);
                    f16::to_f64(conv);
                    conv.to_be_bytes();
                    conv.to_ne_bytes();
                    conv.to_le_bytes();
                    i += 4;
                }
            },
            1=>{
                while i + 8 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f64;
                    let conv = f16::from_f64(float_in);
                    f16::to_f32(conv);
                    f16::to_f64(conv);
                    conv.to_be_bytes();
                    conv.to_ne_bytes();
                    conv.to_le_bytes();
                    i += 8;
                }
            },
            2=>{
                while i + 4 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f32;
                    let conv = bf16::from_f32(float_in);
                    bf16::to_f32(conv);
                    bf16::to_f64(conv);
                    conv.to_be_bytes();
                    conv.to_ne_bytes();
                    conv.to_le_bytes();
                    i += 4;
                }
            },
            3=>{
                while i + 8 < data.len() {
                    let float_in = unsafe{std::ptr::read(&data[i])} as f64;
                    let conv = bf16::from_f64(float_in);
                    bf16::to_f32(conv);
                    bf16::to_f64(conv);
                    conv.to_be_bytes();
                    conv.to_ne_bytes();
                    conv.to_le_bytes();
                    i += 8;
                }
            },
            4=>{
                while i + 2 < data.len() {
                    let float_in = f16::from(unsafe{std::ptr::read(&data[i])});
                    f16::to_f32(float_in);
                    f16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            5=>{
                while i + 2 < data.len() {
                    let float_in = bf16::from(unsafe{std::ptr::read(&data[i])});
                    bf16::to_f32(float_in);
                    bf16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            6=>{
                while i + 2 < data.len() {
                    let mut data_slice = [0 as u8; 2];
                    data_slice[0] = data[i];
                    data_slice[1] = data[i+1];
                    let float_in = f16::from_le_bytes(data_slice);
                    f16::to_f32(float_in);
                    f16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            7=>{
                while i + 2 < data.len() {
                    let mut data_slice = [0 as u8; 2];
                    data_slice[0] = data[i];
                    data_slice[1] = data[i+1];
                    let float_in = bf16::from_le_bytes(data_slice);
                    bf16::to_f32(float_in);
                    bf16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            8=>{
                while i + 2 < data.len() {
                    let mut data_slice = [0 as u8; 2];
                    data_slice[0] = data[i];
                    data_slice[1] = data[i+1];
                    let float_in = f16::from_be_bytes(data_slice);
                    f16::to_f32(float_in);
                    f16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            9=>{
                while i + 2 < data.len() {
                    let mut data_slice = [0 as u8; 2];
                    data_slice[0] = data[i];
                    data_slice[1] = data[i+1];
                    let float_in = bf16::from_be_bytes(data_slice);
                    bf16::to_f32(float_in);
                    bf16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            10=>{
                while i + 2 < data.len() {
                    let mut data_slice = [0 as u8; 2];
                    data_slice[0] = data[i];
                    data_slice[1] = data[i+1];
                    let float_in = f16::from_ne_bytes(data_slice);
                    f16::to_f32(float_in);
                    f16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            11=>{
                while i + 2 < data.len() {
                    let mut data_slice = [0 as u8; 2];
                    data_slice[0] = data[i];
                    data_slice[1] = data[i+1];
                    let float_in = bf16::from_ne_bytes(data_slice);
                    bf16::to_f32(float_in);
                    bf16::to_f64(float_in);
                    float_in.to_be_bytes();
                    float_in.to_ne_bytes();
                    float_in.to_le_bytes();
                    i += 2;
                }
            },
            _=>()
        }
    }
});
