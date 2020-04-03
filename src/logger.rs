

pub mod logger {
  use log::LevelFilter;
  use log4rs::append::file::FileAppender;
  use log4rs::encode::pattern::PatternEncoder;
  use log4rs::config::{Appender, Config, Root};
  pub fn initialize_logger() -> log4rs::Handle  {

    let default_log = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S %Z)(utc)} - {m}{n}")))
        .build("log/default_log.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("default_log", Box::new(default_log)))
        .build(Root::builder().appender("default_log").build(LevelFilter::Trace))
        .unwrap();

    return log4rs::init_config(config).unwrap()

}
}
