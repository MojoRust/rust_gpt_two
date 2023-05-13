mod a_condition_statement {
    pub mod if_statement;
    pub mod conditional_exp;
    pub mod syntax_usage;
    pub mod multiple_condition;

}

mod if_let_exp {
    pub mod syntax_usage;
}

mod b_loop {
    pub mod for_loop;
}
// use a_condition_statement::if_statement;
// use if_let_exp::syntax_usage;

fn main() {

    a_condition_statement::if_statement::run();
    a_condition_statement::conditional_exp::run();
    a_condition_statement::syntax_usage::run();
    a_condition_statement::multiple_condition::run();

    if_let_exp::syntax_usage::run();

    b_loop::for_loop::run();

}
