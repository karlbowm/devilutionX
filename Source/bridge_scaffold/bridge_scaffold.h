#pragma once
#include <cstdint>

namespace devilution {
struct Point;
}

namespace rust_devilution_bridge {
struct BridgedPoint {
	devilution::Point &point;
};

int32_t get_point_x(const BridgedPoint &point);
int32_t get_point_y(const BridgedPoint &point);
} // namespace rust_devilution_bridge