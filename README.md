# Clover

a toy object-oriented language created in Go~

still in development~

#### golang version is under development, only lexer have implemented. If you want to find the CoffeeScript version, go to https://github.com/ippan/clover/tree/coffee

### features
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

    clover ./samples/test.luck