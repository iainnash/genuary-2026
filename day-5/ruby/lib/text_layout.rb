#!/usr/bin/env ruby
require_relative 'glyph_dsl'

# Text layout class using the GlyphDSL
class TextLayout
  def initialize(x, y, scale = 1.0, spacing_factor = 1.0)
    @x = x
    @y = y
    @scale = scale
    @base_spacing = GlyphDSL::GLYPH_SPACING * spacing_factor * scale
    @text = ""
    @glyph_width = 40 * scale  # Default width
  end
  
  def text(text)
    @text = text
    self
  end
  
  def draw(drawing)
    current_x = @x
    
    @text.each_char do |char|
      glyph_data = GlyphDSL.get_glyph(char)
      next unless glyph_data
      
      # Pass the letter character to the drawing interface
      drawing.draw_glyph(glyph_data, current_x, @y, @scale, char)
      
      # Get width restriction if specified
      width_restriction = glyph_data[:width_restriction] or 100
      
      # Calculate advancement based on glyph width and restriction
      # For narrow letters, use their restricted width
      effective_width = @glyph_width
      
      # Add spacing that's proportional to the letter's visual width
      spacing = @base_spacing * (width_restriction / 100.0)
      
      # Advance the cursor
      current_x += effective_width + spacing
    end
  end
end