#include "bridge_scaffold.h"
#include "engine/point.hpp"

namespace rust_devilution_bridge {
int32_t get_point_x(const BridgedPoint &point)
{
	return point.point.x;
}
int32_t get_point_y(const BridgedPoint &point)
{
	return point.point.y;
}

} // namespace rust_devilution_bridge