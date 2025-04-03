mod cli;
mod clierror;
mod config;

use chrono::{Days, Utc};
use cli::Cli;
use clierror::CliError;
use config::Config;
use crossbeam_channel::{Receiver, bounded, select};
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{env, fs, io::Write, path::PathBuf, thread};
use structopt::StructOpt;

use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), CliError> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let mut cfg: Config = confy::load("grrs", "config")?;
    info!("配置文件： {:?}", cfg);

    {
        cfg.name = "王二小".into();
        cfg.last_modified = Utc::now()
            .checked_add_days(Days::new(10))
            .ok_or(CliError("modify date fail".to_owned()))?
            .timestamp();
        confy::store("grrs", "config", cfg)?;
    }

    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            info!("received signal: {:?}", sig);
        }
    });

    let args = Cli::from_args();
    info!("输入内容： {:?}", args);
    let content = fs::read_to_string(&args.path)
        .map_err(|_| CliError(format!("读取文件失败 {:?}", args.path)))?;

    let mut result = Vec::new();
    find_matches(&content, &args.pattern, &mut result)?;
    info!("{:?}", String::from_utf8(result));

    let ctl_c_event = ctl_channel()?;

    loop {
        select! {
            default => {}
            recv(ctl_c_event) -> msg => {
                info!("ctrl-c received: {:?}", msg);
                break;
            }
        }
    }

    Ok(())
}

fn ctl_channel() -> Result<Receiver<String>, CliError> {
    let (sender, receiver) = bounded(100);
    // 设置ctrl-c的回调函数
    ctrlc::set_handler(move || {
        // 发送一个空消息
        let _ = sender.send("EXIT".to_owned());
    })?;

    Ok(receiver)
}

fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> Result<(), CliError> {
    for (id, line) in content.lines().enumerate() {
        let indexs = line
            .match_indices(pattern)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        if !indexs.is_empty() {
            writeln!(writer, "row: {}, col: {:?}. line = {}", id, indexs, line)?;
        }
    }
    Ok(())
}

#[allow(unused)]
fn use_std_parse() -> Result<(), CliError> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let pattern = args.get(0).ok_or(CliError("no pattern input".into()))?;
    let path = args.get(1).ok_or(CliError("no file path input".into()))?;

    let args = Cli {
        pattern: pattern.into(),
        path: PathBuf::from(path),
    };
    info!("输入内容： {:?}", args);
    Ok(())
}

#[test]
fn test_find_matches() {
    let mut result = Vec::new();
    find_matches("laladdwq1lalala\n876", "la", &mut result).unwrap();
    assert_eq!(
        result,
        b"row: 0, col: [0, 2, 9, 11, 13]. line = laladdwq1lalala\n"
    );
}
