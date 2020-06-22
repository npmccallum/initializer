use initializer::func;

#[test]
fn func_name() {
    const fn foo(i: usize) -> u16 {
        i as u16 + 1
    }

    #[func]
    const FOO: [u16; 512] = [foo; 512];

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

    #[func]
    const FOO: [u16; 512] = [foo::foo; 512];

    for i in 0..512 {
        assert_eq!(FOO[i], i as u16 + 1);
    }
}

#[test]
fn closure() {
    #[func]
    const FOO: [usize; 512] = [|i| i + 1; 512];

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn static_const() {
    #[func]
    static FOO: [usize; 512] = [|i| i + 1; 512];

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn static_mut() {
    #[func]
    static mut FOO: [usize; 512] = [|i| i + 1; 512];

    for i in 0..512 {
        assert_eq!(unsafe { FOO[i] }, i + 1);
    }
}

#[test]
fn eval_add() {
    #[func]
    const FOO: [usize; 512] = [|i| i + 1; 256 + 256];

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn eval_sub() {
    #[func]
    const FOO: [usize; 512] = [|i| i + 1; 1024 - 512];

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn eval_mul() {
    #[func]
    const FOO: [usize; 512] = [|i| i + 1; 256 * 2];

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}

#[test]
fn eval_div() {
    #[func]
    const FOO: [usize; 512] = [|i| i + 1; 1024 / 2];

    for i in 0..512 {
        assert_eq!(FOO[i], i + 1);
    }
}
