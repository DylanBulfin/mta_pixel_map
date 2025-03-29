extends Node

const NYC_ANGLE: float = -29 * PI / 180

# The normalized change in pixels that a change in 1 degree of longitude creates
const X_NORM = Vector2(cos(NYC_ANGLE), sin(NYC_ANGLE))
# The change in pixels that a change in 1 degree of latitude creates
const Y_NORM = Vector2(sin(NYC_ANGLE), -cos(NYC_ANGLE))
