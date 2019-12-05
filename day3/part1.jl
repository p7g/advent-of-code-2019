module Part1

export main

const input = split.(readlines("input.txt"), ",")::Array{Array{SubString{String},1},1}

struct Instruction
    direction::Char
    distance::Int64
end

to_instruction(input::String)::Instruction = Instruction(input[1], parse(Int64, input[2:end]))

get_instructions(input::Array{SubString{String},1})::Array{Instruction,1} = to_instruction.(string.(input))

struct Point
    x::Int64
    y::Int64
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

function taxicab(self::Point, other::Point)::Int64
    abs(other.x - self.x) + abs(other.y - self.y)
end

struct Segment
    start::Point
    end_::Point
end

function intersection(self::Segment, other::Segment)::Union{Point,Nothing}
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

    return nothing
end

function line_segments(points::Array{Point,1})::Array{Segment,1}
    len::UInt64 = length(points)
    j::UInt64 = 2
    segments::Array{Segment,1} = []

    while j < len
        push!(segments, Segment(points[j - 1:j]...))
        j += 1
    end

    segments
end

function all_points(input::Array{Instruction,1})::Array{Point,1}
    points::Array{Point,1} = [Point(0, 0)]

    for i in input
        push!(points, move(points[end], i))
    end

    points
end

function main()::Int64
    as::Array{Segment,1}, bs::Array{Segment,1} = line_segments.(all_points.(get_instructions.(input)))

    intersections::Array{Point,1} = []

    origin::Point = Point(0, 0)

    for a in as, b in bs
        p = intersection(a, b)
        if p !== nothing && p != origin
            push!(intersections, p::Point)
        end
    end

    distances::Array{Int64,1} = map(p->taxicab(origin, p), intersections)

    smallest::Int64 = pop!(distances)
    for d in distances
        if d < smallest
            smallest = d
        end
    end

    smallest
end

end