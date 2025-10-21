// Copyright 2018-2025 the Deno authors. MIT license.

use deno_core::anyhow::{AnyError, Context};
use std::fs::File;
use std::path::Path;

pub fn generate_coverage_badge(
  output_path: &Path,
  coverage_percentage: u8,
) -> Result<(), AnyError> {
  let badge_path = coverage_root.join("html/badge.svg");
  let badge_color = match coverage_percentage {
    70..=100 => "#97ca00", // Green
    50..=69 => "#fe7d37",  // Orange
    0..=49 => "#e05d44",   // Red
  };

  let badge_svg = format!(
    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"104\" height=\"20\" role=\"img\" aria-label=\"coverage: {percentage}%\"><title>coverage: {percentage}%</title><linearGradient id=\"s\" x2=\"0\" y2=\"100%\"><stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/><stop offset=\"1\" stop-opacity=\".1\"/></linearGradient><clipPath id=\"r\"><rect width=\"104\" height=\"20\" rx=\"3\" fill=\"#fff\"/></clipPath><g clip-path=\"url(#r)\"><rect width=\"61\" height=\"20\" fill=\"#555\"/><rect x=\"61\" width=\"43\" height=\"20\" fill=\"{color}\"/><rect width=\"104\" height=\"20\" fill=\"url(#s)\"/></g><g fill=\"#fff\" text-anchor=\"middle\" font-family=\"Verdana,Geneva,DejaVu Sans,sans-serif\" text-rendering=\"geometricPrecision\" font-size=\"110\"><text aria-hidden=\"true\" x=\"315\" y=\"150\" fill=\"#010101\" fill-opacity=\".3\" transform=\"scale(.1)\" textLength=\"510\">coverage</text><text x=\"315\" y=\"140\" transform=\"scale(.1)\" fill=\"#fff\" textLength=\"510\">coverage</text><text aria-hidden=\"true\" x=\"815\" y=\"150\" fill=\"#010101\" fill-opacity=\".3\" transform=\"scale(.1)\" textLength=\"330\">{percentage}%</text><text x=\"815\" y=\"140\" transform=\"scale(.1)\" fill=\"#fff\" textLength=\"330\">{percentage}%</text></g></svg>",
    color = badge_color,
    percentage = coverage_percentage
  );

  let mut file =
    File::create(output_path).context("Failed to create badge file")?;
  file
    .write_all(badge_svg.as_bytes())
    .context("Failed to write badge SVG")?;
  Ok(())
}
