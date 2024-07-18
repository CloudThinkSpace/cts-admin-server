use chrono::Local;

#[test]
fn it_works() {
    let aa = "123,";
    let _bb = &aa[..aa.len() - 1];
    let _cc = &aa[..aa.len() - 2];
}
#[test]
fn it_time() {
    let aa = Local::now().naive_local();

    println!("{}", aa);
}

