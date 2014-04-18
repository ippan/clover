# Clover

a toy language created in CoffeeScript~

still in development~

## Example

    sum = function(a, b)
      a + b
    end

    c = sum(1, 2)

    MyBaseClass = class 
      hello = 'hello'

      say = function()
        hello
      end
    end

    MyClass = class extends MyBaseClass

      say = function()
        base.say()
        base.say()
      end

    end

## Usage

    ./bin/clover ./samples/test.luck