use initializer::init_with;

#[test]
fn func_name() {
    const fn foo(i: usize) -> u16 {
        i as u16 + 1
    }

    static FOO: [u16; 512] = init_with!([foo; 512]);

    for i in 0..512 {
        assert_eq!(FOO[i], i as u16 + 1);
    }
}

#[test]
fn func_path() {
    mod foo {
        pub const fn foo(i: usize) -> u16 {
            i as u16 + 1
        }
    }

    static FOO: [u16; 512] = init_with!([foo::foo; 512]);

    for i in 0..512 {
        assert_eq!(FOO[i], i as u16 + 1);
    }
}

#[test]
fn closure() {
    const FOO: [usize; 512] = init_with!([|i| i + 1; 512]);

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn eval_add() {
    const FOO: [usize; 512] = init_with!([|i| i + 1; 256 + 256]);

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn eval_sub() {
    const FOO: [usize; 512] = init_with!([|i| i + 1; 1024 - 512]);

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn eval_mul() {
    const FOO: [usize; 512] = init_with!([|i| i + 1; 256 * 2]);

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn eval_div() {
    const FOO: [usize; 512] = init_with!([|i| i + 1; 1024 / 2]);

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}
