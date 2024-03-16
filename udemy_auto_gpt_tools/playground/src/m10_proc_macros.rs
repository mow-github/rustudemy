#[cfg(test)]
mod test {

    use my_proc_macro::function_to_string;

    const OUTPUT: &str = "";

    #[function_to_string]
    fn some_fn_for_ai_llm(_whatever_param: &str){
        /// This is an awsome function
        /// We shall give it an AI to guess and output
        /// In a structured manner
        println!("{}", OUTPUT);
    }     

    // cargo test tests_proc_macro -- --nocapture
    #[test]
    fn tests_proc_macro(){
        println!("\n---> mod 10 test start\n");

        let x = some_fn_for_ai_llm("some_input");
        dbg!(x);
    }

   
}