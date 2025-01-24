using Plots, Test

function f1(x)
    if 0 ≤ x < 100
        1
    elseif 100 ≤ x < 200
        2
    elseif 200 ≤ x < 300
        3
    elseif 300 ≤ x < 400
        4
    elseif 400 ≤ x < 800
        5
    elseif 800 ≤ x < 1600
        6
    elseif 1600 ≤ x < 3200
        7
    elseif 3200 ≤ x < 6400
        8
    elseif 6400 ≤ x < 12800
        9
    else
        10
    end
end

function f2(x, r)
    max(1, 10 * (1 - exp(-x / r)))
end
f3(x, r) = ceil(f2(x, r))

r = -12800 / log(0.5)
x = 0:50000
p = plot(f1, x, label = "f1")
plot!(p, x -> f2(x, r), x, label = "f2")
plot!(p, x -> f3(x, r), x, label = "f3")
for n in (1600, 3200, 6400)
    let r = -n/log(0.5)
        plot!(p, x -> f3(x, r), x, label = "f3, r=$(r)")
    end
end

function breakpoints(f::F) where {F}
    xs = Vector{Int}(undef, 10)
    x = 0
    i = 1
    while i ≤ 10
        while f(x) < i
            x += 1
        end
        xs[i] = x
        i += 1
    end
    xs
end

function xp_to_next_level(x, r = -12800 / log(0.5))
    level = f3(x, r)
    if level == 10
        0
    else
        y = (level + 1) / 10.0
        thresh = ceil(Int, -log(1 - y) * r)
        thresh - x
    end
end

function max_xp_at_level(level::Int, r = -12800 / log(0.5))
    y = level / 10.0
    floor(Int, -log(1 - y) * r)
end

for i = 1:9
   @test max_xp_at_level(i) + 1 == xs[i+1]
end

[max_xp_at_level.(0:9) .+ 1 xs 0:9]
#=
10×3 Matrix{Int64}:
1      0  0
1946   1946  1
4121   4121  2
6587   6587  3
9434   9434  4
12801  12801  5
16921  16921  6
22234  22234  7
29721  29721  8
42521  42521  9

=#

function integer_sqrt(y::U) where {U<:Unsigned}
    iszero(y) && return y
    two = one(U) + one(U)
    xₖ₋₁ = one(U)
    xₖ = (xₖ₋₁ * xₖ₋₁ + y) ÷ (two * xₖ₋₁)
    xₖ₊₁ = (xₖ * xₖ + y) ÷ (two * xₖ)
    while xₖ₋₁ != xₖ₊₁
        xₖ₋₁ = xₖ
        xₖ = xₖ₊₁
        xₖ₊₁ = (xₖ₊₁ * xₖ₊₁ + y) ÷ (two * xₖ₊₁)
    end
    mn, mx = extrema((xₖ₋₁, xₖ, xₖ₊₁))
    mx * mx ≤ y ? mx : mn
end
using Test
for i in 0:10000000
    @test isqrt(i) == integer_sqrt(i)
end
