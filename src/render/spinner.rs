use std::future::Future;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub async fn spin_until_ready<F, FO>(future: F) -> FO
where
    F: Future<Output = FO>,
{
    //     const LOADING: &str = "
    // x     xxxxx   x   xxxx   xxx  xx  x xxxxx
    // x     x   x  x x  x   x   x   x x x x
    // x     x   x xxxxx x   x   x   x x x x  xx
    // x     x   x x   x x   x   x   x  xx x   x
    // xxxxx xxxxx x   x xxxx   xxx  x  xx xxxxx
    //         ";
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner());
    let result = tokio::select! {
        _ = spin_endless(&spinner) => { unreachable!("future running indefinitely"); }
        output = future => { output }
    };
    spinner.finish_and_clear();
    result
}

async fn spin_endless(spinner: &ProgressBar) -> ! {
    loop {
        spinner.tick();
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

pub async fn spin_and_try_every_second_for<F, FG, FO>(
    future: FG,
    num_tries: usize,
) -> anyhow::Result<FO>
where
    FG: Fn() -> F,
    F: Future<Output = anyhow::Result<FO>>,
{
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner());
    let result = tokio::select! {
        _ = spin_endless(&spinner) => { unreachable!("future running indefinitely"); }
        output = try_future_x_times(future, num_tries) => { output }
    };
    spinner.finish_and_clear();
    result
}

async fn try_future_x_times<F, FG, FO>(future: FG, num_tries: usize) -> anyhow::Result<FO>
where
    FG: Fn() -> F,
    F: Future<Output = anyhow::Result<FO>>,
{
    let mut tries = num_tries;
    loop {
        tries -= 1;
        tokio::time::sleep(Duration::from_secs(1)).await;
        let res = future().await;
        if res.is_ok() || tries == 0 {
            return res;
        }
    }
}
