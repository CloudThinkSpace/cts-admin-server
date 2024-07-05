use chrono::Local;

#[test]
fn it_works() {
    let aa = "123,";
    let bb = &aa[..aa.len()-1];
    let cc = &aa[..aa.len()-2];

    println!("")
}
#[test]
fn it_time() {

    let aa = Local::now().naive_local();

    println!("{}",aa);
}