fn foo() {
    // @outer
    // > outer
    // @inner
    // >> inside
    // @end
    // < outer
    // @end
}

fn bar() {
    // @outer
    // > outer
    // @inner
    // >> inner
    // @innermost
    // >>> innermost
    // @end
    // << inner
    // @end
    // < outer
    // @end
}
