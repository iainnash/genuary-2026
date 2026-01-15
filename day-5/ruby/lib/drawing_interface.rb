#!/usr/bin/env ruby

# Base drawing interface for all renderers
class DrawingInterface
  def initialize(options = {})
    @options = { 
      line_width: 4, 
      color: [0.2, 0.2, 0.2],
      glyph_width: 40,
      glyph_height: 40
    }.merge(options)
  end

  def arc(x, y, radius, start_angle, end_angle)
    raise NotImplementedError, "Subclasses must implement arc"
  end

  def line(x1, y1, x2, y2)
    raise NotImplementedError, "Subclasses must implement line"
  end

  def circle(x, y, radius)
    arc(x, y, radius, 0, 360)
  end

  def render
    raise NotImplementedError, "Subclasses must implement render"
  end
  
  # Draw a glyph with normalized coordinates (0-100)
  # letter parameter is optional and can be used for comments
  def draw_glyph(glyph_data, x, y, scale = 1.0, letter = nil)
    width = @options[:glyph_width] * scale
    height = @options[:glyph_height] * scale
    
    glyph_data[:commands].each do |command|
      type = command[:type]
      params = command[:params].dup
      
      # Transform normalized coordinates (0-100) to actual coordinates
      case type
      when :line
        x1, y1, x2, y2 = params
        
        # Convert from normalized (0-100) to actual coordinates
        x1_actual = x + (x1 / 100.0) * width
        y1_actual = y + (y1 / 100.0) * height
        x2_actual = x + (x2 / 100.0) * width
        y2_actual = y + (y2 / 100.0) * height
        line(x1_actual, y1_actual, x2_actual, y2_actual)
      when :arc
        cx, cy, radius, start_angle, end_angle = params
        
        # Convert from normalized (0-100) to actual coordinates
        cx_actual = x + (cx / 100.0) * width
        cy_actual = y + (cy / 100.0) * height
        radius_actual = (radius / 100.0) * [width, height].min
        arc(cx_actual, cy_actual, radius_actual, start_angle, end_angle)
      when :circle
        cx, cy, radius = params
        
        # Convert from normalized (0-100) to actual coordinates
        cx_actual = x + (cx / 100.0) * width
        cy_actual = y + (cy / 100.0) * height
        radius_actual = (radius / 100.0) * [width, height].min
        circle(cx_actual, cy_actual, radius_actual)
      end
    end
  end
end