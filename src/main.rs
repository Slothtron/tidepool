use clap::Parser;
use tidepool_gvm::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    env_logger::init();

    // 解析命令行参数
    let cli = Cli::parse();

    // 执行命令
    cli.run().await?;

    Ok(())
}
