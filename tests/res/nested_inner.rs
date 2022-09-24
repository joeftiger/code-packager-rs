fn foo() {
    // > outer
    // >> inside
    // < outer
}

fn bar() {
    // > outer
    // >> inner
    // << inner
    // < outer
}
