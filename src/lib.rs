mod fun;
pub struct Captcha {
    pub value: u32,
    pub bytes: Vec<u8>,
}

impl Captcha {
    pub fn new(width: u32, height: u32) -> Self {
        //生成验证码字符数组
        let ask = fun::rand_math();
        let bytes = fun::draw_text(width, height, &ask.0);
        //根据验证码字符数组生成图片并且把图片转化成base64字符串
        Captcha { value: ask.1, bytes }
    }
}

#[test]
fn main_test() {
    let a = Captcha::new(150, 50);
    std::fs::write("./test.png", &a.bytes).unwrap();
    println!("value:{}, bytes:{}", a.value, a.bytes.len());
}
