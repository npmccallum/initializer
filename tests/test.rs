use initializer::foreach;

#[test]
fn func_name() {
    const fn foo(i: u16) -> u16 {
        i + 1
    }

    static FOO: [u16; 512] = foreach![do foo; 512];

    for i in 0..512 {
        assert_eq!(FOO[i as usize], i + 1);
    }
}

#[test]
fn func_path() {
    mod foo {
        pub const fn foo(i: u16) -> u16 {
            i + 1
        }
    }

    static FOO: [u16; 512] = foreach![do foo::foo; 512];

    for i in 0..512 {
        assert_eq!(FOO[i as usize], i + 1);
    }
}

#[test]
fn closure() {
    const FOO: [u16; 512] = foreach![do |i| i + 1; 512];

    for i in 0..512 {
        assert_eq!(FOO[i as usize], i + 1);
    }
}

#[test]
fn eval_add() {
    const FOO: [u16; 512] = foreach![do |i| i + 1; 256 + 256];

    for i in 0..512 {
        assert_eq!(FOO[i as usize], i + 1);
    }
}

#[test]
fn eval_sub() {
    const FOO: [u16; 512] = foreach![do |i| i + 1; 1024 - 512];

    for i in 0..512 {
        assert_eq!(FOO[i as usize], i + 1);
    }
}

#[test]
fn eval_mul() {
    const FOO: [u16; 512] = foreach![do |i| i + 1; 256 * 2];

    for i in 0..512 {
        assert_eq!(FOO[i as usize], i + 1);
    }
}

#[test]
fn eval_div() {
    const FOO: [u16; 512] = foreach![do |i| i + 1; 1024 / 2];

    for i in 0..512 {
        assert_eq!(FOO[i as usize], i + 1);
    }
}
