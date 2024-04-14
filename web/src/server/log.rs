
/// 增加日志初始化函数
/// 使用tracing进行日志初始化
pub fn init_log(){
    tracing_subscriber::fmt::fmt()
        .init();
}