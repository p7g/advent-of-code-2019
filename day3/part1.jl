input = split.(readlines("input.txt"), ",")

struct Instruction
  direction::Char
  distance::Int
end

to_instruction(input::String) = Instruction(input[1], parse(Int, input[2:end]))

get_instructions(input) = to_instruction.(map(string, input))

mutable struct Point
  x::Int
  y::Int
end

function move(p::Point, i::Instruction)::Point
  if i.direction == 'D'
    Point(p.x, p.y - i.distance)
  elseif i.direction == 'U'
    Point(p.x, p.y + i.distance)
  elseif i.direction == 'L'
    Point(p.x - i.distance, p.y)
  elseif i.direction == 'R'
    Point(p.x + i.distance, p.y)
  end
end

function taxicab(self::Point, other::Point)::Int
  abs(other.x - self.x) + abs(other.y - self.y)
end

struct Segment
  start::Point
  end_::Point
end

function intersection(self::Segment, other::Segment)
  # self is vertical and other is horizontal
  if self.start.y != self.end_.y && other.start.x != other.end_.x && ((
      self.start.y <= other.start.y
      && self.end_.y >= other.start.y
    ) || (
      self.end_.y <= other.start.y
      && self.start.y >= other.start.y
    )) && ((
      other.start.x <= self.start.x
      && other.end_.x >= self.start.x
    ) || (
      other.end_.x <= self.start.x
      && other.start.x >= self.start.x
    ))
    return Point(self.start.x, other.start.y)
    # other is vertical and self is horizontal
  elseif self.start.x != self.end_.x && other.start.y != other.end_.x && ((
      self.start.x <= other.start.x
      && self.end_.x >= other.start.x
    ) || (
      self.end_.x <= other.start.x
      && self.start.x >= other.start.x
    )) && ((
      other.start.y <= self.start.y
      && other.end_.y >= self.start.y
    ) || (
      other.end_.y <= self.start.y
      && other.start.y >= self.start.y
    ))
    return Point(self.start.y, other.start.x)
  end
end

function line_segments(points::AbstractArray{Point})
  len = length(points)
  j = 2
  segments = []

  while j < len
    push!(segments, Segment(points[j-1:j]...))
    j += 1
  end

  segments
end

function all_points(input::AbstractArray{Instruction})
  points = [Point(0, 0)]

  for i in input
    push!(points, move(points[end], i))
  end

  points
end

function main()
  as, bs = line_segments.(all_points.(get_instructions.(input)))

  intersections = []

  for a in as
    for b in bs
      p = intersection(a, b)
      if p != nothing && (p.x != 0 || p.y != 0)
        push!(intersections, p)
      end
    end
  end

  origin = Point(0, 0)

  distances = map(p -> taxicab(origin, p), intersections)

  smallest = pop!(distances)
  for d in distances
    if d < smallest
      smallest = d
    end
  end

  smallest
end
