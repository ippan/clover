# Clover

a toy object-oriented language created in C#~

still in development~

#### C# version is under development, some feature have not implemented. If you want to find the CoffeeScript version, go to https://github.com/ippan/clover/tree/coffee

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