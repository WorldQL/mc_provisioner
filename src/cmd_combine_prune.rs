use color_eyre::Result;
use tracing::error;

use crate::config::{GlobalArgs, WorldManagementArgs};

pub fn combine(global_args: GlobalArgs, args: WorldManagementArgs) -> Result<()> {
    let args = check_args(args);
    dbg!(&args);

    todo!()
}

pub fn prune(global_args: GlobalArgs, args: WorldManagementArgs) -> Result<()> {
    let args = check_args(args);
    dbg!(&args);

    todo!()
}

#[derive(Debug)]
struct CheckedArgs {
    pub world_diameter: u32,
    pub slice_width: u32,
    pub avoid_slicing_origin: bool,
    pub origin_radius: u32,
}

fn check_args(args: WorldManagementArgs) -> CheckedArgs {
    // region: Check for Value
    let world_diameter = match args.world_diameter {
        Some(value) => value,
        None => {
            error!("you must specify the arg: world_diameter");
            std::process::exit(1);
        },
    };

    let slice_width = match args.slice_width {
        Some(value) => value,
        None => {
            error!("you must specify the arg: slice_width");
            std::process::exit(1);
        },
    };

    let avoid_slicing_origin = match args.avoid_slicing_origin {
        Some(value) => value,
        None => {
            error!("you must specify the arg: avoid_slicing_origin");
            std::process::exit(1);
        },
    };

    let origin_radius = match args.origin_radius {
        Some(value) => value,
        None => {
            error!("you must specify the arg: origin_radius");
            std::process::exit(1);
        },
    };
    // endregion

    // Slices must be > 0 and divisible by 512
    if slice_width == 0 || slice_width % 512 != 0 {
        error!("`slice_width` must be greater than 0 and a multiple of 512");
        std::process::exit(1);
    }

    if world_diameter % slice_width != 0 {
        error!("`world_diameter` must be a multiple of `slice_width`");
        std::process::exit(1);
    }

    if world_diameter < slice_width {
        error!("`world_diameter` must greater than or equal to `slice_width`");
        std::process::exit(1);
    }

    // The origin must be double the width of a slice to line up nicely
    if origin_radius != slice_width {
        error!("`origin_radius` must match `slice_width`");
        std::process::exit(1);
    }

    CheckedArgs {
        world_diameter,
        slice_width,
        avoid_slicing_origin,
        origin_radius,
    }
}
