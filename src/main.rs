use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use lib::startup::Application;
use lib::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("backendtemplate".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let application = Application::build().await?;;
    let application_task = tokio::spawn(application.run_until_stopped());
    tokio::select! {
        o = application_task => report_exit("Application API", o)
    }
    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "{} has failed", task_name)
        }
        Err(e) => {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "{} task failed to complete", task_name)
        }
    }
}