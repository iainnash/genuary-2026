#!/usr/bin/env ruby

# Require all necessary files
require_relative 'lib/drawing_interface'
require_relative 'lib/renderers/postscript_drawing'
require_relative 'lib/renderers/svg_drawing'
require_relative 'lib/renderers/gcode_drawing'
require_relative 'lib/glyph_dsl'
require_relative 'lib/text_layout'
require_relative 'lib/glyphs/letters'

# Example usage
if __FILE__ == $PROGRAM_NAME
  # Create a PostScript drawing
  ps_drawing = PostScriptDrawing.new(line_width: 4, color: [0.2, 0.2, 0.2])
  
  # Create a text layout and add text
  layout = TextLayout.new(100, 400, 1.0, 1.0)
  layout.text("genurary")
  
  # Draw the text
  layout.draw(ps_drawing)
  
  # Output the PostScript code
  File.write("logo-ruby.ps", ps_drawing.render)
  
  # Create an SVG drawing
  svg_drawing = SVGDrawing.new(line_width: 4, color: [0.2, 0.2, 0.2])
  layout.text("genurary")
  layout.draw(svg_drawing)
  
  # Output the SVG code
  File.write("logo-ruby.svg", svg_drawing.render)
  
  # Create a G-code drawing
  gcode_drawing = GcodeDrawing.new(
    line_width: 0.5,
    feed_rate: 1500,
    pen_up_position: 80,
    pen_down_position: 40,
    origin_x: 20,
    origin_y: 20
  )
  
  # Create a text layout for G-code with appropriate scale
  gcode_layout = TextLayout.new(10, 50, 0.5, 1.5)
  gcode_layout.text("genurary")
  
  # Draw the text
  gcode_layout.draw(gcode_drawing)
  
  # Output the G-code
  File.write("logo-ruby.gcode", gcode_drawing.render)
  
  puts "PostScript, SVG, and G-code files generated successfully!"
end