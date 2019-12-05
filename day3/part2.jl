module Part2

export main

const input = split.(readlines("input.txt"), ",")::Array{Array{SubString{String},1},1}

struct Instruction
    direction::Char
    distance::Int64
end

to_instruction(input::String)::Instruction = Instruction(input[1], parse(Int, input[2:end]))

get_instructions(input::Array{SubString{String}})::Array{Instruction} = to_instruction.(string.(input))

struct Point
    x::Int64
    y::Int64

    steps::Int64
end

function move(p::Point, i::Instruction, steps::Int64)::Point
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

function intersection(self::Segment, other::Segment)::Union{Tuple{Int64,Int64},Nothing}
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

function line_segments(points::Array{Point,1})::Array{Segment,1}
    len::Int64 = length(points)
    j::UInt64 = 2
    segments::Array{Segment,1} = []

    while j < len
        push!(segments, Segment(points[j - 1:j]...))
        j += 1
    end

    segments
end

function all_points(input::Array{Instruction,1})::Array{Point,1}
    points::Array{Point,1} = [Point(0, 0, 0)]

    seen_points::Dict{Tuple{Int64,Int64},Point} = Dict()
    steps::Int64 = 0

    for i in input
        moved::Point = move(points[end], i, steps)
        steps = moved.steps

        if (moved.x, moved.y) in keys(seen_points)
            p = seen_points[moved.x, moved.y]
        else
            p = moved
        end

        push!(points, p::Point)
    end

    points
end

function main()
    as::Array{Segment,1}, bs::Array{Segment,1} = line_segments.(all_points.(get_instructions.(input)))

    intersections::Array{Tuple{Int64,Int64},1} = []

    for a in as, b in bs
        p::Union{Tuple{Int64,Int64},Nothing} = intersection(a, b)
        if p !== nothing && +(p...) != 0
            push!(intersections, p)
        end
    end

    stepses::Array{Int64,1} = map(ss->+(ss...), intersections)

    smallest = pop!(stepses)
    for d in stepses
        if d < smallest
            smallest = d
        end
    end

    smallest
end

end