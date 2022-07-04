pub fn map(x:usize, from_min:usize, from_max:usize, to_min:usize, to_max:usize) -> usize {
    return (x - from_min) * (to_max - to_min) / (from_max - from_min) + to_min;
}
#[allow(dead_code)]
pub fn extract_url(string:String) -> String {
    let reg:regex::Regex = regex::Regex::new(r"(http|https)://([w_-]+(?:(?:.[w_-]+)+))([w.,@?^=%&:/~+#-]*[w@?^=%&/~+#-])").unwrap();
    let res = reg.find(string.as_str());
    let string_final: String = match res {
        None => "yappppppp".to_owned(),
        Some(st) => {
            println!("{:#?}  {:#?}",st.start(),st.end());
            string[st.start()..st.end()].to_owned()
        },
    };
    println!("{:#?}",string_final);
    return string_final;
}