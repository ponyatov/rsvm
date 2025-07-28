//! generic GUI library for framebuffer-like output

#![allow(dead_code)]

use crate::config::*;

/// 2D coordinates
type Coord = (i16, i16);

/// color: RGB TrueColor
type Color = (u8, u8, u8);
