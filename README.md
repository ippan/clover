# Clover

a scripting language created in Rust

still in development~

### features
* bytecode
* first class function

## Example

```lua,ruby
include Vector2D as Vector from "./vector"

public model Rect
  start
  size
end

implement Rect
  function new()
    local rect = Rect()
    rect.start = Vector.new()
    rect.size = Vector.new()
    rect
  end
end

model MyRect
end

# copy all function in Rect to MyRect
apply Rect to MyRect

function main()
  local rect = MyRect.new()
  rect
end
```

## Usage

    clover examples/main.luck