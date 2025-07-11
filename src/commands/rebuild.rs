use log::{debug, error, info};
use tokio::process::Command;

pub async fn rebuild_cmd(
    program: &crate::util::cli::Program,
    args: &nixos_cli_def::commands::rebuild::RebuildArgs,
) {
    let cli = &program.cli;
    let extra_args = &program.extra_args;
    let subcommand = &program.subcommand;

    debug!("Resolving project {}", cli.project);
    let Ok(project) = crate::util::project::resolve(&cli.project).await else {
        return error!("Could not find project {}", cli.project);
    };

    let mut path = project.get_path();

    debug!("Resolved project {path:?}");

    path.push("nilla.nix");

    match path.try_exists() {
        Ok(false) | Err(_) => return error!("File not found"),
        _ => {}
    }

    let hostname = if let Some(name) = args.name.clone() {
        if name.contains('.') {
            return error!("Invalid hostname {}", name);
        } else {
            name
        }
    } else {
        gethostname::gethostname().into_string().unwrap()
    };

    let attribute = &format!("systems.nixos.\"{hostname}\".result");

    let mut info_flags: Vec<String> = Vec::with_capacity(5);

    info_flags.push(format!("--{}", &str::repeat("v", cli.verbose.into())));

    // Arbitrary value
    if cli.verbose >= 2 {
        info_flags.push(String::from("--show-trace"));
    }

    if cli.quiet {
        info_flags.extend(vec![
            String::from("--quiet"),
            String::from("--no-build-output"),
        ]);
    }

    debug!(
        "Passing these extra args to nixos-rebuild: [{}]",
        extra_args.join(",")
    );

    debug!(
        "Passing these information flags to nixos-rebuild: [{}]",
        info_flags.join(",")
    );

    info!("Building system {hostname}");
    Command::new("nixos-rebuild")
        .arg(subcommand)
        .arg("--file")
        .arg(path.display().to_string())
        .arg("--attr")
        .arg(attribute)
        .args(extra_args)
        .args(info_flags)
        // NOTE:
        // This flag will cause a deprecation warning to be printed if the user has
        // nixos-rebuild-ng installed as their nixos-rebuild
        // Until nixos-rebuild-ng becomes the default and/or there is almost 0% chance the user
        // will be using the old nixos-rebuild, we should continue to use this flag
        // Also note that this flag will (to my understanding) not ask for the sudo password unless
        // it is needed
        .arg("--use-remote-sudo")
        .status()
        .await
        .unwrap();
}
