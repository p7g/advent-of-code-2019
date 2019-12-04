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

  steps::Int
end

function move(p::Point, i::Instruction, steps::Int)::Point
  if i.direction == 'D'
    x, y = p.x, p.y - i.distance
  elseif i.direction == 'U'
    x, y = p.x, p.y + i.distance
  elseif i.direction == 'L'
    x, y = p.x - i.distance, p.y
  elseif i.direction == 'R'
    x, y = p.x + i.distance, p.y
  end

  Point(x, y, steps + i.distance)
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
    return (self.start.steps + abs(other.start.y - self.start.y),
            other.start.steps + abs(self.start.x - other.start.x))
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
    return (self.start.steps + abs(other.start.x - self.start.x),
            other.start.steps + abs(self.start.y - other.start.y))
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
  points = [Point(0, 0, 0)]

  seen_points = Dict()
  steps = 0

  for i in input
    moved = move(points[end], i, steps)
    steps = moved.steps

    if (moved.x, moved.y) in keys(seen_points)
      p = seen_points[moved.x, moved.y]
    else
      p = moved
    end

    push!(points, p)
  end

  points
end

function main()
  as, bs = line_segments.(all_points.(get_instructions.(input)))

  intersections = []

  for a in as
    for b in bs
      p = intersection(a, b)
      if p != nothing && +(p...) != 0
        push!(intersections, p)
      end
    end
  end

  stepses = map(ss -> +(ss...), intersections)

  smallest = pop!(stepses)
  for d in stepses
    if d < smallest
      smallest = d
    end
  end

  smallest
end
