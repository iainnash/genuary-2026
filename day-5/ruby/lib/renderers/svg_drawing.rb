#!/usr/bin/env ruby
require_relative '../drawing_interface'

# SVG implementation of the drawing interface
class SVGDrawing < DrawingInterface
  def initialize(options = {})
    super
    @width = options[:width] || 612
    @height = options[:height] || 792
    @background_color = options[:background_color] || "white"
    @commands = []
    @commands << '<?xml version="1.0" encoding="UTF-8" standalone="no"?>'
    @commands << "<svg width=\"#{@width}\" height=\"#{@height}\" xmlns=\"http://www.w3.org/2000/svg\">"
    
    # Add a white background rectangle
    @commands << "<rect width=\"100%\" height=\"100%\" fill=\"#{@background_color}\" />"
    
    # Convert color from 0-1 range to 0-255 range for SVG
    r = (@options[:color][0] * 255).to_i
    g = (@options[:color][1] * 255).to_i
    b = (@options[:color][2] * 255).to_i
    @stroke_style = "stroke=\"rgb(#{r},#{g},#{b})\" stroke-width=\"#{@options[:line_width]}\" fill=\"none\""
    @current_letter = nil
    @current_group = nil
  end

  def arc(x, y, radius, start_angle, end_angle)
    # Check if this is a full circle (or very close to it)
    if (end_angle - start_angle).abs >= 359.9
      # For full circles, use the circle element
      y_flipped = @height - y
      @commands << "<circle cx=\"#{x}\" cy=\"#{y_flipped}\" r=\"#{radius}\" #{@stroke_style} />"
      return
    end
    
    # Convert angles from degrees to radians
    start_rad = start_angle * Math::PI / 180
    end_rad = end_angle * Math::PI / 180
    
    # Calculate start and end points
    start_x = x + radius * Math.cos(start_rad)
    start_y = @height - (y + radius * Math.sin(start_rad))  # Flip Y-coordinate
    end_x = x + radius * Math.cos(end_rad)
    end_y = @height - (y + radius * Math.sin(end_rad))      # Flip Y-coordinate
    
    # Determine if the arc is large or small (> 180 degrees)
    large_arc = (end_angle - start_angle).abs > 180 ? 1 : 0
    
    # When flipping Y, we need to change the sweep flag (direction) for arcs
    sweep_flag = start_angle < end_angle ? 0 : 1  # Reversed due to Y-flip
    
    # SVG path for arc
    path = "M #{start_x},#{start_y} A #{radius},#{radius} 0 #{large_arc},#{sweep_flag} #{end_x},#{end_y}"
    @commands << "<path d=\"#{path}\" #{@stroke_style} />"
  end

  def line(x1, y1, x2, y2)
    # Flip Y-coordinates
    y1_flipped = @height - y1
    y2_flipped = @height - y2
    @commands << "<line x1=\"#{x1}\" y1=\"#{y1_flipped}\" x2=\"#{x2}\" y2=\"#{y2_flipped}\" #{@stroke_style} />"
  end
  
  # Override the circle method for direct circle element creation
  def circle(x, y, radius)
    y_flipped = @height - y
    @commands << "<circle cx=\"#{x}\" cy=\"#{y_flipped}\" r=\"#{radius}\" #{@stroke_style} />"
  end

  def render
    # Close any open group
    if @current_group
      @commands << "</g>"
      @current_group = nil
    end
    
    @commands << "</svg>"
    @commands.join("\n")
  end
  
  # Override draw_glyph to add letter comments and group elements
  def draw_glyph(glyph_data, x, y, scale = 1.0, letter = nil)
    # If we have a new letter, close previous group and start a new one
    if letter && letter != @current_letter
      # Close previous group if there was one
      if @current_group
        @commands << "</g>"
      end
      
      # Start a new group with a comment
      @commands << "\n<!-- Letter: #{letter} -->"
      @commands << "<g id=\"letter-#{letter}\">"
      @current_letter = letter
      @current_group = letter
    end
    
    # Call the parent method to draw the glyph
    super(glyph_data, x, y, scale)
  end
end