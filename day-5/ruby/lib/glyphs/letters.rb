#!/usr/bin/env ruby
require_relative '../glyph_dsl'

GlyphDSL.define_glyph ' ' do
  # pass :)
end

# Define all letter glyphs using the DSL with normalized coordinates (0-100)
GlyphDSL.define_glyph :g do
  # G is a circle with a line extending from the bottom right
  circle 50, 50, 50
  arc 50, 10, 50, 240, 390
end

GlyphDSL.define_glyph :e do
  # E is a 3/4 circle with a horizontal line
  arc 50, 50, 50, 0, 270
  line 50, 50, 100, 50
end

GlyphDSL.define_glyph :n do
  # N is a vertical line, half circle, and another vertical line
  vertical_line 0, 0, 100
  arc 50, 50, 50, 0, 180
  vertical_line 100, 0, 50
end

GlyphDSL.define_glyph :u do
  # U is a vertical line, half circle at bottom, and another vertical line
  vertical_line 0, 100, 50
  arc 50, 50, 50, 180, 360
  vertical_line 100, 100, 50
end

GlyphDSL.define_glyph :r do
  # R is a vertical line with a quarter circle at top right
  vertical_line 0, 0, 100
  arc 0, 50, 50, 0, 90
  # Make this letter narrower than default
  restrict_width 10
end

GlyphDSL.define_glyph :a do
  # A is a circle with a vertical line on the right
  circle 50, 50, 50
  vertical_line 100, 0, 100
end

GlyphDSL.define_glyph :y do
  # Y is a V shape with a stem
  line 0, 100, 50, 50
  line 50, 50, 100, 100
  vertical_line 50, 50, 0
end

# Additional letters
GlyphDSL.define_glyph :b do
  vertical_line 0, 0, 100
  arc 50, 25, 50, 270, 90
  arc 50, 75, 50, 270, 90
end

GlyphDSL.define_glyph :c do
  arc 50, 50, 50, 45, 315
  # Make this letter narrower than default
  restrict_width 70
end

GlyphDSL.define_glyph :d do
  vertical_line 100, 0, 100
  arc 50, 50, 50, 90, 270
end

GlyphDSL.define_glyph :f do
  vertical_line 25, 0, 100
  horizontal_line 25, 75, 100
  horizontal_line 25, 75, 50
  # Make this letter narrower than default
  restrict_width 75
end

GlyphDSL.define_glyph :h do
  vertical_line 0, 0, 100
  vertical_line 100, 0, 100
  horizontal_line 0, 100, 50
end

GlyphDSL.define_glyph :i do
  vertical_line 50, 0, 100
  circle 50, 100, 5
  # Make this letter much narrower
  restrict_width 30
end

GlyphDSL.define_glyph :j do
  vertical_line 75, 0, 75
  arc 50, 25, 25, 0, 180
  circle 75, 100, 5
  # Make this letter narrower
  restrict_width 75
end

GlyphDSL.define_glyph :k do
  vertical_line 0, 0, 100
  line 0, 50, 100, 100
  line 0, 50, 100, 0
  # Make this letter narrower
  restrict_width 80
end

GlyphDSL.define_glyph :l do
  vertical_line 50, 0, 100
  # Make this letter much narrower
  restrict_width 30
end

GlyphDSL.define_glyph :m do
  vertical_line 0, 0, 100
  vertical_line 50, 0, 100
  vertical_line 100, 0, 100
  arc 25, 100, 25, 180, 0
  arc 75, 100, 25, 180, 0
end

GlyphDSL.define_glyph :o do
  circle 50, 50, 50
  # Make this letter slightly narrower
  restrict_width 90
end

GlyphDSL.define_glyph :p do
  vertical_line 0, 0, 100
  arc 50, 75, 50, 270, 90
  # Make this letter narrower
  restrict_width 80
end

GlyphDSL.define_glyph :q do
  circle 50, 50, 50
  line 70, 30, 100, 0
end

GlyphDSL.define_glyph :s do
  arc 50, 75, 25, 0, 270
  arc 50, 25, 25, 180, 90
  # Make this letter narrower
  restrict_width 70
end

GlyphDSL.define_glyph :t do
  vertical_line 50, 0, 100
  horizontal_line 25, 75, 100
  # Make this letter narrower
  restrict_width 70
end

GlyphDSL.define_glyph :v do
  line 0, 100, 50, 0
  line 50, 0, 100, 100
end

GlyphDSL.define_glyph :w do
  line 0, 100, 25, 0
  line 25, 0, 50, 50
  line 50, 50, 75, 0
  line 75, 0, 100, 100
end

GlyphDSL.define_glyph :x do
  line 0, 0, 100, 100
  line 0, 100, 100, 0
end

GlyphDSL.define_glyph :z do
  horizontal_line 0, 100, 100
  line 100, 100, 0, 0
  horizontal_line 0, 100, 0
  # Make this letter narrower
  restrict_width 80
end

# Add basic numbers
GlyphDSL.define_glyph '0' do
  circle 50, 50, 50
  # Make this number narrower
  restrict_width 80
end

GlyphDSL.define_glyph '1' do
  vertical_line 50, 0, 100
  # Make this number much narrower
  restrict_width 30
end

GlyphDSL.define_glyph '2' do
  arc 50, 75, 50, 0, 270
  line 50, 75, 0, 0
  # Make this number narrower
  restrict_width 80
end