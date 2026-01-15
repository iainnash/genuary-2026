#!/usr/bin/env ruby
require_relative '../drawing_interface'

# PostScript implementation of the drawing interface
class PostScriptDrawing < DrawingInterface
  def initialize(options = {})
    super
    @commands = []
    @commands << "%!PS-Adobe-3.0"
    @commands << "%%BoundingBox: 0 0 612 792"
    @commands << "stroke"
    @commands << "#{@options[:line_width]} setlinewidth"
    @commands << "#{@options[:color][0]} #{@options[:color][1]} #{@options[:color][2]} setrgbcolor"
    @current_letter = nil
  end

  def arc(x, y, radius, start_angle, end_angle)
    @commands << "newpath"
    @commands << "#{x} #{y} #{radius} #{start_angle} #{end_angle} arc"
    @commands << "stroke"
  end

  def line(x1, y1, x2, y2)
    @commands << "newpath"
    @commands << "#{x1} #{y1} newpath moveto"
    @commands << "#{x2} #{y2} lineto"
    @commands << "stroke"
  end

  def render
    @commands << "showpage"
    @commands.join("\n")
  end
  
  # Override draw_glyph to add letter comments
  def draw_glyph(glyph_data, x, y, scale = 1.0, letter = nil)
    # Add a comment for the letter if provided
    if letter && letter != @current_letter
      @commands << "\n% Letter: #{letter}"
      @current_letter = letter
    end
    
    # Call the parent method to draw the glyph
    super(glyph_data, x, y, scale)
  end
end