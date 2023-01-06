#[allow(dead_code)]
#[cxx::bridge]
mod ffi {
    #[derive(Eq, PartialEq)]
    #[repr(u8)]
    enum Direction {
        South,
        SouthWest,
        West,
        NorthWest,
        North,
        NorthEast,
        East,
        SouthEast,
    }

    #[namespace = "rust_devilution"]
    extern "Rust" {
        fn get_direction(start: &BridgedPoint, destination: &BridgedPoint) -> Direction;
    }

    #[namespace = "rust_devilution_bridge"]
    unsafe extern "C++" {
        include!("bridge_scaffold/bridge_scaffold.h");
        type BridgedPoint;

        pub fn get_point_x(point: &BridgedPoint) -> i32;
        pub fn get_point_y(point: &BridgedPoint) -> i32;
    }
}

pub fn get_direction(start: &ffi::BridgedPoint, destination: &ffi::BridgedPoint) -> ffi::Direction {
    let mut md;

    let mut mx = ffi::get_point_x(destination) - ffi::get_point_x(start);
    let mut my = ffi::get_point_y(destination) - ffi::get_point_y(start);
    if mx >= 0 {
        if my >= 0 {
            if 5 * mx <= (my * 2)
            // mx/my <= 0.4, approximation of tan(22.5)
            {
                return ffi::Direction::SouthWest;
            }
            md = ffi::Direction::South;
        } else {
            my = -my;
            if 5 * mx <= (my * 2) {
                return ffi::Direction::NorthEast;
            }
            md = ffi::Direction::East;
        }
        if 5 * my <= (mx * 2)
        // my/mx <= 0.4
        {
            md = ffi::Direction::SouthEast;
        }
    } else {
        mx = -mx;
        if my >= 0 {
            if 5 * mx <= (my * 2) {
                return ffi::Direction::SouthWest;
            }
            md = ffi::Direction::West;
        } else {
            my = -my;
            if 5 * mx <= (my * 2) {
                return ffi::Direction::NorthEast;
            }
            md = ffi::Direction::North;
        }
        if 5 * my <= (mx * 2) {
            md = ffi::Direction::NorthWest;
        }
    }
    md
}
