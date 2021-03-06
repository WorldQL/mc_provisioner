use std::collections::HashSet;
use std::fs::{self, DirEntry};
use std::ops::Add;
use std::path::PathBuf;

use color_eyre::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::{error, info, warn};

use crate::config::{GlobalArgs, WorldManagementArgs};
use crate::utils;

const SYNC_DIRS: [&str; 3] = ["region", "entities", "poi"];

// region: Commands
pub fn combine(global_args: GlobalArgs, args: WorldManagementArgs) -> Result<()> {
    let args = check_args(args);

    // Clean existing combined directory
    if args.combined_directory.exists() {
        fs::remove_dir_all(&args.combined_directory)?;
    }

    // Create (now empty) directory
    fs::create_dir_all(&args.combined_directory)?;

    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    for (idx, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();

        // Region owners are 0-indexed
        let idx = idx - 1;
        let world_dir = directory.join(&global_args.level_name);

        // Copy level.dat from first server
        if idx == 0 {
            let level_dat_source = world_dir.join("level.dat");
            let level_dat_dest = args.combined_directory.join("level.dat");

            if level_dat_source.exists() {
                info!("copying `level.dat` from {}", &name);
                fs::copy(level_dat_source, level_dat_dest)?;
            }
        }

        for raw_dir in SYNC_DIRS {
            // Resolve and create destination directory
            let out_dir = args.combined_directory.join(&raw_dir);
            fs::create_dir_all(&out_dir)?;

            // Absolute directory path
            let dir = world_dir.join(&raw_dir);

            if !dir.exists() {
                warn!("directory {:?} does not exist, skipping sync", dir);
                continue;
            }

            info!("{}: copying region files from \"{}\"", &name, &raw_dir);
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let (path, filename) = match entry_is_region_file(entry)? {
                    Some(value) => value,
                    None => continue,
                };

                let region_coords = match parse_coords(&filename) {
                    Some(coords) => coords,

                    None => {
                        tracing::warn!("invalid region file name: {:?}", path);
                        continue;
                    }
                };

                let block_coords = region_coords.min_block_from_region();
                let owner = get_owner_of_location(&args, global_args.server_count, block_coords);

                // Negative owned regions are outside of the world area
                if owner < 0 {
                    continue;
                }

                // Only copy regions this server owns
                let owner = owner as u8;
                if owner != idx {
                    continue;
                }

                let destination = out_dir.join(&filename);
                fs::copy(&path, &destination)?;
            }
        }
    }

    Ok(())
}

pub fn optimize(global_args: GlobalArgs, args: WorldManagementArgs) -> Result<()> {
    let args = check_args(args);

    // Ensure combined directory exists
    if !args.combined_directory.exists() {
        error!("You must run `provisioner combine` first");
        std::process::exit(1);
    }

    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    for (idx, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();

        // Region owners are 0-indexed
        let idx = idx - 1;
        let world_dir = directory.join(&global_args.level_name);

        for raw_dir in SYNC_DIRS {
            // Resolve and check combined dir
            let master_dir = args.combined_directory.join(&raw_dir);
            if !master_dir.exists() {
                warn!(
                    "directory {:?} does not exist, skipping optimization",
                    master_dir
                );

                continue;
            }

            // Server-local world directory
            let world_dir = world_dir.join(&raw_dir);
            if !world_dir.exists() {
                warn!(
                    "directory {:?} does not exist, skipping optimization",
                    world_dir
                );

                continue;
            }

            let mut regions_to_copy = HashSet::new();

            info!(
                "{}: cleaning unused region files from \"{}\"",
                &name, &raw_dir
            );

            for entry in fs::read_dir(&world_dir)? {
                let entry = entry?;
                let (path, filename) = match entry_is_region_file(entry)? {
                    Some(value) => value,
                    None => continue,
                };

                let region_coords = match parse_coords(&filename) {
                    Some(coords) => coords,

                    None => {
                        tracing::warn!("invalid region file name: {:?}", path);
                        continue;
                    }
                };

                let block_coords = region_coords.min_block_from_region();
                let owner = get_owner_of_location(&args, global_args.server_count, block_coords);

                // Cast server index to i16 to allow less than zero comparisons
                // Lets us remove regions outside the world bounds
                let idx = i16::from(idx);
                if owner == idx {
                    // Track adjacent regions
                    for x in -1..=1i64 {
                        for z in -1..=1i64 {
                            let adjacent_region_coords = region_coords + (x, z);
                            let adjacent_block_coords =
                                adjacent_region_coords.min_block_from_region();

                            let adjacent_owner = get_owner_of_location(
                                &args,
                                global_args.server_count,
                                adjacent_block_coords,
                            );

                            // If adjacent region is not owned by this server, should be copied
                            if adjacent_owner != owner {
                                regions_to_copy.insert(adjacent_region_coords);
                            }
                        }
                    }
                } else {
                    // Remove file
                    fs::remove_file(&path)?;
                }
            }

            info!(
                "{}: copying adjacent region files to \"{}\"",
                &name, &raw_dir
            );

            for region in regions_to_copy {
                let filename = region.region_filename();
                let filepath = master_dir.join(&filename);

                // Copy from master directory if the file exists
                if filepath.exists() {
                    let dest_path = world_dir.join(&filename);
                    fs::copy(&filepath, &dest_path)?;
                }
            }
        }
    }

    Ok(())
}

fn entry_is_region_file(entry: DirEntry) -> Result<Option<(PathBuf, String)>> {
    if !entry.file_type()?.is_file() {
        return Ok(None);
    }

    let path = entry.path();
    match path.extension() {
        None => return Ok(None),
        Some(extension) => {
            if extension != "mca" {
                return Ok(None);
            }
        }
    }

    let filename = path.file_name().unwrap().to_string_lossy();
    let filename: String = filename.into();

    Ok(Some((path, filename)))
}
// endregion

// region: Args
#[derive(Debug)]
struct CheckedArgs {
    pub world_diameter: u32,
    pub slice_width: u32,
    pub avoid_slicing_origin: bool,
    pub origin_radius: u32,
    pub combined_directory: PathBuf,
}

fn check_args(args: WorldManagementArgs) -> CheckedArgs {
    // region: Check for Value
    let world_diameter = match args.world_diameter {
        Some(value) => value,
        None => {
            error!("you must specify the arg: world_diameter");
            std::process::exit(1);
        }
    };

    let slice_width = match args.slice_width {
        Some(value) => value,
        None => {
            error!("you must specify the arg: slice_width");
            std::process::exit(1);
        }
    };

    let avoid_slicing_origin = match args.avoid_slicing_origin {
        Some(value) => value,
        None => {
            error!("you must specify the arg: avoid_slicing_origin");
            std::process::exit(1);
        }
    };

    let origin_radius = match args.origin_radius {
        Some(value) => value,
        None => {
            error!("you must specify the arg: origin_radius");
            std::process::exit(1);
        }
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
        combined_directory: args.combined_directory,
    }
}
// endregion

// region Coords
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    pub x: i64,
    pub z: i64,
}

impl Add<(i64, i64)> for Coords {
    type Output = Self;

    fn add(self, (other_x, other_z): (i64, i64)) -> Self::Output {
        Self {
            x: self.x + other_x,
            z: self.z + other_z,
        }
    }
}
// endregion

// region: Slice Functions
fn in_unsliced_origin(args: &CheckedArgs, coords: Coords) -> bool {
    if !args.avoid_slicing_origin {
        return false;
    }

    let r = i64::from(args.origin_radius);
    let x = coords.x.abs();
    let z = coords.z.abs();

    x < r && z < r
}

fn get_owner_of_location(args: &CheckedArgs, server_count: u8, coords: Coords) -> i16 {
    if in_unsliced_origin(args, coords) {
        return 0;
    }

    let slices_per_row = i64::from(args.world_diameter / args.slice_width);
    let adjusted_x = coords.x + i64::from(args.world_diameter / 2);
    let adjusted_z = coords.z + i64::from(args.world_diameter / 2);

    let slice_width = i64::from(args.slice_width);
    let slice_x = adjusted_x / slice_width;
    let slice_z = adjusted_z / slice_width;

    let position = slice_x + (slice_z * slices_per_row);
    let owner = position % i64::from(server_count);

    owner as i16
}
// endregion

// region: Region Functions
impl Coords {
    pub fn min_block_from_chunk(&self) -> Self {
        Self {
            x: self.x << 4,
            z: self.z << 4,
        }
    }

    pub fn max_block_from_chunk(&self) -> Self {
        Self {
            x: ((self.x + 1) << 4) - 1,
            z: ((self.z + 1) << 4) - 1,
        }
    }

    pub fn min_chunk_from_region(&self) -> Self {
        Self {
            x: self.x << 5,
            z: self.z << 5,
        }
    }

    pub fn max_chunk_from_region(&self) -> Self {
        Self {
            x: ((self.x + 1) << 5) - 1,
            z: ((self.z + 1) << 5) - 1,
        }
    }

    pub fn min_block_from_region(&self) -> Self {
        self.min_chunk_from_region().min_block_from_chunk()
    }

    pub fn max_block_from_region(&self) -> Self {
        self.max_chunk_from_region().max_block_from_chunk()
    }
}

#[rustfmt::skip]
static REGION_RX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*r\.(-?\d+)\.(-?\d+)\.mca\s*$").unwrap());

fn parse_coords(string: &str) -> Option<Coords> {
    let captures = REGION_RX.captures(string)?;

    let x = captures.get(1)?.as_str();
    let z = captures.get(2)?.as_str();

    let x = x.parse::<i64>().ok()?;
    let z = z.parse::<i64>().ok()?;

    Some(Coords { x, z })
}

impl Coords {
    #[inline]
    #[must_use]
    pub fn region_filename(&self) -> String {
        format!("r.{}.{}.mca", self.x, self.z)
    }
}
// endregion
