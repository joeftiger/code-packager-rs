fn foo() {
    // > outer
    // >> inside
    // < outer
}

fn bar() {
    // > outer
    // >> inner
    // >>> innermost
    // << inner
    // < outer
}
