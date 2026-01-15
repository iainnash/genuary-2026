#!/usr/bin/env ruby

# Glyph DSL module
module GlyphDSL
  # Standard spacing
  GLYPH_SPACING = 20
  
  # Storage for all defined glyphs
  @glyphs = {}
  
  # DSL method to define a glyph
  def self.define_glyph(name, &block)
    glyph = []
    builder = GlyphBuilder.new(glyph)
    builder.instance_eval(&block)
    @glyphs[name.to_s.downcase] = {
      commands: glyph,
      width_restriction: builder.width_restriction
    }
  end
  
  # Get a defined glyph
  def self.get_glyph(name)
    glyph_data = @glyphs[name.to_s.downcase]
    return nil unless glyph_data
    glyph_data
  end
  
  # Helper class for building glyphs
  class GlyphBuilder
    attr_reader :width_restriction
    
    def initialize(glyph)
      @glyph = glyph
      @width_restriction = 100 # Default to full width
    end
    
    # All coordinates are normalized to 0-100 range
    # where 50,50 is the center of the glyph
    
    def line(x1, y1, x2, y2)
      @glyph << { type: :line, params: [x1, y1, x2, y2] }
    end
    
    def arc(x, y, radius, start_angle, end_angle)
      @glyph << { type: :arc, params: [x, y, radius, start_angle, end_angle] }
    end
    
    def circle(x, y, radius)
      @glyph << { type: :circle, params: [x, y, radius] }
    end
    
    # Set width restriction (0-100)
    def restrict_width(width_percent)
      @width_restriction = [[0, width_percent].max, 100].min
    end
    
    # Convenience methods for normalized coordinates
    def vertical_line(x, y1, y2)
      line x, y1, x, y2
    end
    
    def horizontal_line(x1, x2, y)
      line x1, y, x2, y
    end
  end
end