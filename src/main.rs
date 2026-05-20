mod vm;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vm = vm::VirtualMachine::new();

    // 1. 파일을 일단 텍스트 문자열로 읽어옵니다.
    let content = fs::read_to_string("test.txt")?;

    // 2. 공백으로 쪼갠 뒤, 각각의 문자열을 실제 숫자(u8)로 변환(parse)하여 묶습니다.
    let codes: Vec<u8> = content
        .split_whitespace() // 공백이나 줄바꿈 기준으로 쪼갭니다.
        .map(|s| s.parse::<u8>()) // 문자열 "20"을 숫자 20u8로 변환합니다.
        .collect::<Result<Vec<u8>, _>>()?; // 에러가 없다면 Vec<u8>로 수집합니다.

    // 3. 이제 진짜 숫자 배열이 되었으므로 VM에 안전하게 넘겨줍니다!
    let len = codes.len();
    vm.eval(&codes, len);

    Ok(())
}
