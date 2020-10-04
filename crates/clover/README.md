# Clover

a toy object-oriented language created in Rust

still in development~

### features
* bytecode
* classes & inheritance
* first class function
* closure 

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
        base.say() + base.say()
      end

    end

## Usage

    CloverCli ./examples/main.luck