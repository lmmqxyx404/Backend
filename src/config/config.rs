///服务启动配置
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
    ///当前服务地址
    pub server_url: String,
    ///redis地址
    pub redis_url: String,
    /// 数据库地址
    pub database_url: String,
    /// 逻辑删除字段
    /* pub logic_column: String,
    pub logic_un_deleted: i64,
    pub logic_deleted: i64, */
    ///日志目录 "target/logs/"
    pub log_dir: String,
    /// "100MB" 日志分割尺寸-单位KB,MB,GB
    pub log_temp_size: String,
    /// 日志打包格式可选“”（空-不压缩）“gzip”（gz压缩包）“zip”（zip压缩包）“lz4”（lz4压缩包（非常快））
    pub log_pack_compress: String,
    ///日志滚动配置   保留全部:All,按时间保留:KeepTime(Duration),按版本保留:KeepNum(i64)
    pub log_rolling_type: String,
    ///日志等级
    pub log_level: String,
    /// 日志类型 choose one of  ("mmap","file")
    pub log_type: String,
    /// 日志相关
    /// Log channel length: null for unbounded queue, non-null for bounded queue (better performance)
    pub log_chan_len: Option<usize>,
    ///短信缓存队列（mem/redis）
    pub sms_cache_send_key_prefix: String,
    ///jwt 秘钥
    pub jwt_secret: String,
    ///白名单接口
    pub white_list_api: Vec<String>,
    ///权限缓存类型
    pub cache_type: String,
    ///重试
    pub login_fail_retry: u64,
    ///重试等待时间
    pub login_fail_retry_wait_sec: u64,
    /// 日期格式化
    pub datetime_format: String,
    /// TOKEN 失效时间
    pub jwt_exp: usize,
}

///默认配置
impl Default for ApplicationConfig {
    fn default() -> Self {
        let json_data = include_str!("../../application.json5");
        // let yml_data = include_str!("../../config.yml");
        //读取配置
        let mut result: ApplicationConfig =
            json5::from_str(json_data).expect("load config file fail");
        if cfg!(debug_assertions) {
            result.debug = true;
        } else {
            result.debug = false;
        }
        if result.debug {
            println!("[backend] load config:{:?}", result);
            println!(
                "[backend] ///////////////////// Start On Debug Mode ////////////////////////////"
            );
        } else {
            println!("[backend] release_mode is enable!")
        }
        result
    }
}
