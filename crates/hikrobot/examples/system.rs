use hikrobot::{Devices, HikRobot};

fn main() -> hikrobot::Result<()> {
    let hik = hik_robot()?;

    version(&hik);
    devices(&hik)?;

    Ok(())
}

fn hik_robot() -> hikrobot::Result<HikRobot> {
    let hik = HikRobot::new()?;

    println!("HikRobot::new()");
    println!("  initialized: true");

    Ok(hik)
}

fn version(hik: &HikRobot) {
    let version = hik.version();

    println!("HikRobot::version()");
    println!("  major: {:?}", version.major);
    println!("  minor: {:?}", version.minor);
    println!("  patch: {:?}", version.patch);
    println!("  build: {:?}", version.build);
    println!("  raw: {:?}", version.raw);
}

fn devices(hik: &HikRobot) -> hikrobot::Result<Devices<'_>> {
    let devices = hik.devices()?;

    println!("HikRobot::devices()");
    println!("  count: {:?}", devices.len());

    Ok(devices)
}
