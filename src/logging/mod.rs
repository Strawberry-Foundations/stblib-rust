pub mod featureset;
pub mod formats;
pub mod level;

use std::fmt::Display;
use crate::logging::formats::LogFormat;
use crate::logging::featureset::FeatureSet;
use crate::utilities::current_time;
use crate::logging::level::LogLevel;

pub struct Logger {
    pub feat_set: FeatureSet,
    pub formatting: LogFormat,
}

impl Logger {
    #[must_use]
    pub const fn new(feature_set: FeatureSet, formatter: LogFormat) -> Self {
        Self {
            feat_set: feature_set,
            formatting: formatter
        }
    }

    fn loglevel_parser(&self, level: &LogLevel) -> String {
        let level_str = match level {
            LogLevel::DEFAULT => "DEFAULT",
            LogLevel::INFO => "INFO",
            LogLevel::ERROR => "ERROR",
            LogLevel::WARNING => "WARNING",
            LogLevel::CRITICAL => "CRITICAL"
        };

        if self.formatting.extensions.levelname_lowercase {
            level_str.to_lowercase()
        } else {
            String::from(level_str)
        }
    }

    fn parse(&self, level: &LogLevel, content: impl ToString) -> String {
        match level {
            LogLevel::DEFAULT => {
                self.formatting.default
                    .replace("[%<levelname>%]", &self.loglevel_parser(level))
                    .replace("[%<message>%]", &content.to_string())
                    .replace("[%<time>%]", current_time(&self.formatting.extensions.time_fmt).as_str())
            },

            LogLevel::INFO => {
                self.formatting.info
                    .replace("[%<levelname>%]", &self.loglevel_parser(level))
                    .replace("[%<message>%]", &content.to_string())
                    .replace("[%<time>%]", current_time(&self.formatting.extensions.time_fmt).as_str())
            },

            LogLevel::ERROR => {
                self.formatting.error
                    .replace("[%<levelname>%]", &self.loglevel_parser(level))
                    .replace("[%<message>%]", &content.to_string())
                    .replace("[%<time>%]", current_time(&self.formatting.extensions.time_fmt).as_str())
            },

            LogLevel::WARNING => {
                self.formatting.warning
                    .replace("[%<levelname>%]", &self.loglevel_parser(level))
                    .replace("[%<message>%]", &content.to_string())
                    .replace("[%<time>%]", current_time(&self.formatting.extensions.time_fmt).as_str())
            },

            LogLevel::CRITICAL => {
                self.formatting.critical
                    .replace("[%<levelname>%]", &self.loglevel_parser(level))
                    .replace("[%<message>%]", &content.to_string())
                    .replace("[%<time>%]", current_time(&self.formatting.extensions.time_fmt).as_str())
            },
        }
    }

    pub fn default(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::DEFAULT, log_message));
    }

    pub fn info(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::INFO, log_message));
    }

    pub fn error(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::ERROR, log_message));
    }

    pub fn warning(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::WARNING, log_message));
    }

    pub fn critical(&self, log_message: impl Display) {
        println!("{}", self.parse(&LogLevel::CRITICAL, log_message));
    }

    pub fn panic(&self, log_message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::ERROR, log_message));
        std::process::exit(1);
    }

    pub fn panic_critical(&self, log_message: impl Display) -> ! {
        println!("{}", self.parse(&LogLevel::CRITICAL, log_message));
        std::process::exit(1);
    }
}