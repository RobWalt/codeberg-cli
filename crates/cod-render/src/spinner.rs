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
